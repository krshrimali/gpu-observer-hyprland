use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Sparkline},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::app::App;

use super::{theme, widgets};

const LABEL_W: u16 = 14;
const VALUE_W: u16 = 9; // e.g. " 200 MHz" or " 100%"

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let snap = app.snapshot.lock();
    let gpu = &snap.gpu;

    // ── Block / border ────────────────────────────────────────────────────────
    let title_color = theme::temp_color(gpu.temperature_c);
    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" GPU ", Style::default().fg(theme::TEXT_BRIGHT).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("· {} ", gpu.name),
                Style::default().fg(theme::TEXT_DIM),
            ),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::SURFACE));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.height < 6 {
        return;
    }

    // ── Vertical layout inside the block ─────────────────────────────────────
    // BigText for temperature uses HalfHeight pixels → 4 rows per glyph.
    // "XXC" = 3 glyphs wide (24 cols). We reserve 4 rows for it.
    let temp_big_rows: u16 = 4;

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(temp_big_rows), // temperature BigText
            Constraint::Length(1),             // GPU util gauge
            Constraint::Length(1),             // Mem BW gauge
            Constraint::Length(1),             // Power gauge
            Constraint::Length(1),             // spacer / divider
            Constraint::Length(3),             // GPU util sparkline
            Constraint::Length(1),             // Graphics clock
            Constraint::Length(1),             // Memory clock
            Constraint::Length(1),             // Fan speed
            Constraint::Fill(1),               // padding
        ])
        .split(inner);

    // ── Temperature (BigText) ─────────────────────────────────────────────────
    render_temp_big(f, rows[0], gpu.temperature_c, title_color);

    // ── GPU Utilisation gauge ─────────────────────────────────────────────────
    let gpu_util_color = theme::util_color(gpu.utilization_gpu_pct);
    widgets::gauge_row(
        f,
        rows[1],
        "GPU Util",
        gpu.utilization_gpu_pct as f64 / 100.0,
        gpu_util_color,
        &format!("{:3}%", gpu.utilization_gpu_pct),
        LABEL_W,
        VALUE_W,
    );

    // ── Memory Bandwidth gauge ────────────────────────────────────────────────
    let mem_bw_color = theme::util_color(gpu.utilization_mem_pct);
    widgets::gauge_row(
        f,
        rows[2],
        "Mem BW",
        gpu.utilization_mem_pct as f64 / 100.0,
        mem_bw_color,
        &format!("{:3}%", gpu.utilization_mem_pct),
        LABEL_W,
        VALUE_W,
    );

    // ── Power gauge ───────────────────────────────────────────────────────────
    let (power_w, limit_w) = (gpu.power_draw_mw / 1000, gpu.power_limit_mw / 1000);
    let power_ratio = if limit_w > 0 {
        power_w as f64 / limit_w as f64
    } else {
        0.0
    };
    let power_color = theme::util_color((power_ratio * 100.0) as u32);
    widgets::gauge_row(
        f,
        rows[3],
        "Power",
        power_ratio,
        power_color,
        &format!("{:3}W", power_w),
        LABEL_W,
        VALUE_W,
    );

    // ── Section divider ───────────────────────────────────────────────────────
    f.render_widget(widgets::section_divider("history"), rows[4]);

    // ── GPU utilisation sparkline ─────────────────────────────────────────────
    let hist = app.history.gpu_util.as_ordered_vec();
    f.render_widget(
        Sparkline::default()
            .data(&hist)
            .max(100)
            .style(Style::default().fg(theme::NEON_CYAN).bg(theme::SURFACE)),
        rows[5],
    );

    // ── Clock / fan info rows ─────────────────────────────────────────────────
    render_kv_row(
        f,
        rows[6],
        "Gfx Clock",
        &format!("{:4} MHz", gpu.clock_graphics_mhz),
        theme::NEON_BLUE,
    );
    render_kv_row(
        f,
        rows[7],
        "Mem Clock",
        &format!("{:4} MHz", gpu.clock_memory_mhz),
        theme::NEON_BLUE,
    );
    render_kv_row(
        f,
        rows[8],
        "Fan",
        &format!("{:3}%", gpu.fan_speed_pct),
        if gpu.fan_speed_pct > 0 {
            theme::NEON_PURPLE
        } else {
            theme::TEXT_DIM
        },
    );
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Renders temperature as a large BigText number + small "°C" unit beside it.
fn render_temp_big(f: &mut Frame, area: Rect, temp: u32, color: Color) {
    if area.width < 10 || area.height < 4 {
        // Fallback to plain text if no room
        f.render_widget(
            Paragraph::new(format!("  {}°C", temp))
                .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
            area,
        );
        return;
    }

    // Split: BigText number | unit label column
    // Each HalfHeight glyph = 8 cols wide, 4 rows tall.
    let digits = format!("{}", temp);
    let big_cols = digits.len() as u16 * 8; // exact width of BigText

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(big_cols.min(area.width.saturating_sub(6))),
            Constraint::Fill(1),
        ])
        .split(area);

    let big = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .lines(vec![Line::from(digits.as_str())])
        .build();
    f.render_widget(big, cols[0]);

    // Unit + power limit note stacked in the right column
    let unit_area = cols[1];
    if unit_area.height >= 1 {
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(" °C", Style::default().fg(color)),
            ]))
            .alignment(Alignment::Left),
            // Place at bottom of the BigText rows
            Rect {
                y: unit_area.y + unit_area.height.saturating_sub(2),
                height: 1,
                ..unit_area
            },
        );
    }
}

/// Simple two-column key-value row (no gauge bar).
fn render_kv_row(f: &mut Frame, area: Rect, label: &str, value: &str, value_color: Color) {
    if area.height == 0 {
        return;
    }
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(LABEL_W), Constraint::Fill(1)])
        .split(area);

    f.render_widget(
        Paragraph::new(label).style(theme::label_style()),
        cols[0],
    );
    f.render_widget(
        Paragraph::new(value)
            .alignment(Alignment::Right)
            .style(Style::default().fg(value_color).add_modifier(Modifier::BOLD)),
        cols[1],
    );
}
