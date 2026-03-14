/// Shared low-level widget helpers used across panels.
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Gauge, Paragraph},
    Frame,
};

use super::theme;

/// Renders a single-row metric: `LABEL ▓▓▓▓▓░░░░░ VALUE`
///
/// - `label`  : left-aligned text, padded to `label_width` chars
/// - `ratio`  : 0.0–1.0 fill for the gauge bar
/// - `bar_color`: fill colour
/// - `value_str`: right-aligned string shown to the right of the bar
///
/// Layout (horizontal):
///   [label_width] [gauge: fill] [value_width]
pub fn gauge_row(
    f: &mut Frame,
    area: Rect,
    label: &str,
    ratio: f64,
    bar_color: Color,
    value_str: &str,
    label_width: u16,
    value_width: u16,
) {
    if area.height == 0 || area.width < label_width + value_width + 3 {
        return;
    }

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(label_width),
            Constraint::Min(4),
            Constraint::Length(value_width),
        ])
        .split(area);

    // Label
    f.render_widget(
        Paragraph::new(label).style(theme::label_style()),
        cols[0],
    );

    // Gauge bar (no built-in label — value goes in its own column)
    let ratio = ratio.clamp(0.0, 1.0);
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(bar_color).bg(theme::GAUGE_EMPTY))
            .ratio(ratio)
            .label(""),
        cols[1],
    );

    // Value
    f.render_widget(
        Paragraph::new(value_str)
            .alignment(Alignment::Right)
            .style(Style::default().fg(bar_color)),
        cols[2],
    );
}

/// A horizontal divider line with an optional centred title.
/// Rendered as a plain Paragraph (no Block border).
pub fn section_divider<'a>(title: &'a str) -> Paragraph<'a> {
    let line = if title.is_empty() {
        Line::from(Span::styled(
            "─".repeat(60),
            Style::default().fg(theme::BORDER),
        ))
    } else {
        let pad = "──";
        Line::from(vec![
            Span::styled(pad, Style::default().fg(theme::BORDER)),
            Span::styled(
                format!(" {} ", title),
                theme::section_title_style(),
            ),
            Span::styled(pad, Style::default().fg(theme::BORDER)),
        ])
    };
    Paragraph::new(line)
}

/// Right-pads `s` with spaces so its display width is exactly `width`.
/// Truncates with "…" if too long.
pub fn fixed_width(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        let mut t: String = s.chars().take(width.saturating_sub(1)).collect();
        t.push('…');
        t
    } else {
        format!("{:<width$}", s, width = width)
    }
}

/// Formats bytes as "X.X GiB" always (for consistent column widths).
pub fn fmt_gib(bytes: u64) -> String {
    const GIB: f64 = (1024 * 1024 * 1024) as f64;
    format!("{:.2} GiB", bytes as f64 / GIB)
}

/// Formats bytes as "X MiB" always.
pub fn fmt_mib(bytes: u64) -> String {
    format!("{} MiB", bytes / (1024 * 1024))
}
