pub mod cpu_panel;
pub mod gpu_panel;
pub mod mem_panel;
pub mod process_panel;
pub mod theme;
pub mod widgets;

use ratatui::{layout::{Constraint, Direction, Layout}, Frame};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    // ── Root: top panels + process panel ─────────────────────────────────────
    // Process panel is fixed 9 rows; the rest goes to the three-column top area.
    let vstack = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(20),   // GPU / MEM / CPU panels
            Constraint::Length(9), // Process panel
        ])
        .split(area);

    // ── Three equal top columns ───────────────────────────────────────────────
    let top_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(vstack[0]);

    gpu_panel::render(f, app, top_cols[0]);
    mem_panel::render(f, app, top_cols[1]);
    cpu_panel::render(f, app, top_cols[2]);
    process_panel::render(f, app, vstack[1]);

    // ── Global app title rendered as the outer window title ───────────────────
    // We draw a transparent outer block just to carry the title bar.
    // It sits on top of the entire area but has no fill — panels draw over it.
    render_title_bar(f, app, area);
}

/// Renders a thin title row at the very top, overlapping the panel borders so
/// the panel titles look like they belong to the same surface.
fn render_title_bar(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // We only draw text, no opaque block — the panels' own borders show through.
    // Place a Paragraph in a 1-row strip across the top.
    // Actually, the panel blocks already occupy the full area, so we don't need
    // an extra title bar — the panel block titles serve as column headers.
    // Instead, render a subtle keybind hint at the bottom-right of the process panel.
    let _ = (f, app, area); // no-op: reserved for future use
}
