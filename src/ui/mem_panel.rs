use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Sparkline},
    Frame,
};

use crate::app::App;

use super::{theme, widgets};

const LABEL_W: u16 = 14;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let snap = app.snapshot.lock();
    let gpu = &snap.gpu;
    let ram = &snap.ram;

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(
                " MEMORY ",
                Style::default()
                    .fg(theme::TEXT_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::SURFACE));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.height < 8 {
        return;
    }

    // ── Layout ────────────────────────────────────────────────────────────────
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // "── VRAM ──" divider
            Constraint::Length(1), // VRAM used gauge
            Constraint::Length(1), // VRAM used kv
            Constraint::Length(1), // VRAM free kv
            Constraint::Length(1), // VRAM total kv
            Constraint::Length(3), // VRAM sparkline
            Constraint::Length(1), // "── RAM ──" divider
            Constraint::Length(1), // RAM used gauge
            Constraint::Length(1), // RAM used kv
            Constraint::Length(1), // RAM available kv
            Constraint::Length(1), // RAM cached kv
            Constraint::Length(1), // RAM free kv
            Constraint::Fill(1),   // padding
        ])
        .split(inner);

    // ── VRAM section ──────────────────────────────────────────────────────────
    f.render_widget(widgets::section_divider("VRAM"), rows[0]);

    let vram_ratio = if gpu.vram_total_bytes > 0 {
        gpu.vram_used_bytes as f64 / gpu.vram_total_bytes as f64
    } else {
        0.0
    };
    let vram_color = theme::util_color((vram_ratio * 100.0) as u32);

    widgets::gauge_row(
        f,
        rows[1],
        "Used / Total",
        vram_ratio,
        vram_color,
        &format!("{:3}%", (vram_ratio * 100.0) as u32),
        LABEL_W,
        5,
    );

    kv_row(
        f,
        rows[2],
        "  Used",
        &widgets::fmt_gib(gpu.vram_used_bytes),
        vram_color,
    );
    kv_row(
        f,
        rows[3],
        "  Free",
        &widgets::fmt_gib(gpu.vram_free_bytes),
        theme::NEON_GREEN,
    );
    kv_row(
        f,
        rows[4],
        "  Total",
        &widgets::fmt_gib(gpu.vram_total_bytes),
        theme::TEXT_DIM,
    );

    // VRAM history sparkline
    let vram_hist = app.history.vram_used_mib.as_ordered_vec();
    let vram_max = (gpu.vram_total_bytes / (1024 * 1024)).max(1);
    f.render_widget(
        Sparkline::default()
            .data(&vram_hist)
            .max(vram_max)
            .style(Style::default().fg(vram_color).bg(theme::SURFACE)),
        rows[5],
    );

    // ── RAM section ───────────────────────────────────────────────────────────
    f.render_widget(widgets::section_divider("RAM"), rows[6]);

    let ram_ratio = if ram.total_bytes > 0 {
        ram.used_bytes as f64 / ram.total_bytes as f64
    } else {
        0.0
    };
    let ram_color = theme::util_color((ram_ratio * 100.0) as u32);

    widgets::gauge_row(
        f,
        rows[7],
        "Used / Total",
        ram_ratio,
        ram_color,
        &format!("{:3}%", (ram_ratio * 100.0) as u32),
        LABEL_W,
        5,
    );

    kv_row(
        f,
        rows[8],
        "  Used",
        &widgets::fmt_gib(ram.used_bytes),
        ram_color,
    );
    kv_row(
        f,
        rows[9],
        "  Available",
        &widgets::fmt_gib(ram.available_bytes),
        theme::NEON_GREEN,
    );
    kv_row(
        f,
        rows[10],
        "  Cached",
        &widgets::fmt_gib(ram.cached_approx_bytes),
        theme::NEON_BLUE,
    );
    kv_row(
        f,
        rows[11],
        "  Free",
        &widgets::fmt_gib(ram.free_bytes),
        theme::TEXT_DIM,
    );
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn kv_row(
    f: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    value_color: ratatui::style::Color,
) {
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
