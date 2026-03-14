use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::app::App;
use super::{theme, widgets};

// Fixed column widths (chars)
const COL_PID: u16 = 7;
const COL_VRAM: u16 = 10;
// Name column takes the remaining width (Min)

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let snap = app.snapshot.lock();
    let processes = &snap.gpu.processes;

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(
                " GPU PROCESSES ",
                Style::default()
                    .fg(theme::TEXT_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("({}) ", processes.len()),
                Style::default().fg(theme::TEXT_DIM),
            ),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::SURFACE));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.height < 2 {
        return;
    }

    // ── Table widths ──────────────────────────────────────────────────────────
    // [PID] [NAME: fill] [VRAM]
    let widths = [
        Constraint::Length(COL_PID),
        Constraint::Min(10),
        Constraint::Length(COL_VRAM),
    ];

    // ── Header row ────────────────────────────────────────────────────────────
    let header = Row::new(vec![
        Cell::from("PID").style(
            Style::default()
                .fg(theme::BORDER_BRIGHT)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("PROCESS").style(
            Style::default()
                .fg(theme::BORDER_BRIGHT)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("VRAM").style(
            Style::default()
                .fg(theme::BORDER_BRIGHT)
                .add_modifier(Modifier::BOLD),
        ),
    ])
    .height(1);

    // ── Data rows ─────────────────────────────────────────────────────────────
    let rows: Vec<Row> = if processes.is_empty() {
        // Placeholder row
        vec![Row::new(vec![
            Cell::from(""),
            Cell::from("No active GPU processes").style(theme::dim_style()),
            Cell::from(""),
        ])]
    } else {
        processes
            .iter()
            .map(|p| {
                let vram_str = widgets::fmt_mib(p.vram_bytes);
                Row::new(vec![
                    Cell::from(format!("{:>COL_PID$}", p.pid, COL_PID = COL_PID as usize))
                        .style(Style::default().fg(theme::TEXT_DIM)),
                    Cell::from(widgets::fixed_width(&p.name, 40))
                        .style(Style::default().fg(theme::TEXT_BRIGHT)),
                    Cell::from(format!("{:>COL_VRAM$}", vram_str, COL_VRAM = COL_VRAM as usize))
                        .style(
                            Style::default()
                                .fg(theme::NEON_CYAN)
                                .add_modifier(Modifier::BOLD),
                        ),
                ])
                .height(1)
            })
            .collect()
    };

    let mut state = TableState::default();
    if !processes.is_empty() {
        state.select(Some(app.selected_process.min(processes.len() - 1)));
    }

    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(1)
        .row_highlight_style(
            Style::default()
                .bg(theme::SURFACE2)
                .fg(theme::NEON_CYAN)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(table, inner, &mut state);
}
