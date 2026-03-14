use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Borders, Paragraph, Sparkline},
    Frame,
};

use crate::app::App;

use super::{theme, widgets};

const LABEL_W: u16 = 14;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let snap = app.snapshot.lock();
    let cpu = &snap.cpu;

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(
                " CPU ",
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

    if inner.height < 4 {
        return;
    }

    // ── Layout ────────────────────────────────────────────────────────────────
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // CPU brand / model
            Constraint::Length(1), // Global util gauge
            Constraint::Length(1), // spacer / divider
            Constraint::Length(5), // Per-core bar chart
            Constraint::Length(1), // spacer / divider
            Constraint::Length(3), // CPU util sparkline
            Constraint::Length(1), // Avg frequency
            Constraint::Length(1), // Max frequency
            Constraint::Fill(1),   // padding
        ])
        .split(inner);

    // ── CPU brand ─────────────────────────────────────────────────────────────
    let brand = widgets::fixed_width(&cpu.brand, inner.width as usize);
    f.render_widget(
        Paragraph::new(brand)
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(theme::TEXT_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            ),
        rows[0],
    );

    // ── Global utilisation gauge ──────────────────────────────────────────────
    let util_color = theme::util_color_f(cpu.global_utilization_pct);
    widgets::gauge_row(
        f,
        rows[1],
        "Total Util",
        cpu.global_utilization_pct as f64 / 100.0,
        util_color,
        &format!("{:5.1}%", cpu.global_utilization_pct),
        LABEL_W,
        8,
    );

    // ── Divider ───────────────────────────────────────────────────────────────
    f.render_widget(widgets::section_divider("cores"), rows[2]);

    // ── Per-core bar chart ────────────────────────────────────────────────────
    render_core_bars(f, rows[3], &cpu.per_core_utilization);

    // ── Divider ───────────────────────────────────────────────────────────────
    f.render_widget(widgets::section_divider("history"), rows[4]);

    // ── CPU utilisation sparkline ─────────────────────────────────────────────
    let hist = app.history.cpu_util.as_ordered_vec();
    f.render_widget(
        Sparkline::default()
            .data(&hist)
            .max(100)
            .style(Style::default().fg(util_color).bg(theme::SURFACE)),
        rows[5],
    );

    // ── Frequency rows ────────────────────────────────────────────────────────
    let avg_freq = if cpu.per_core_frequency_mhz.is_empty() {
        0
    } else {
        cpu.per_core_frequency_mhz.iter().sum::<u64>()
            / cpu.per_core_frequency_mhz.len() as u64
    };
    let max_freq = cpu.per_core_frequency_mhz.iter().copied().max().unwrap_or(0);

    render_kv_row(
        f,
        rows[6],
        "Avg Freq",
        &format!("{:5} MHz", avg_freq),
        theme::NEON_BLUE,
    );
    render_kv_row(
        f,
        rows[7],
        "Max Freq",
        &format!("{:5} MHz", max_freq),
        theme::NEON_PURPLE,
    );
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Renders a compact per-core bar chart.
/// Bar width is 1, gap is 1 so each core takes 2 columns.
/// Labels omitted to pack as many cores as possible.
fn render_core_bars(f: &mut Frame, area: Rect, utilization: &[f32]) {
    if area.height == 0 || utilization.is_empty() {
        return;
    }

    // Determine how many cores fit: each bar = 1 col + 1 gap = 2 cols, except last.
    let available_cols = area.width as usize;
    let n_cores = utilization.len();
    // Fit as many as possible: n * 2 - 1 <= available  →  n <= (available + 1) / 2
    let visible = ((available_cols + 1) / 2).min(n_cores);

    let bars: Vec<Bar> = utilization[..visible]
        .iter()
        .enumerate()
        .map(|(i, &pct)| {
            let color = theme::util_color_f(pct);
            Bar::default()
                .value(pct as u64)
                .label(Line::from(format!("{:X}", i % 16))) // hex digit fits 1 col
                .style(Style::default().fg(color))
                .value_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        })
        .collect();

    let bar_chart = BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .bar_width(1)
        .bar_gap(1)
        .max(100)
        .style(Style::default().bg(theme::SURFACE));

    f.render_widget(bar_chart, area);
}

fn render_kv_row(
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
