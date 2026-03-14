use std::sync::Arc;

use crossterm::event::KeyCode;
use parking_lot::Mutex;

use crate::{history::History, metrics::Snapshot};

pub enum AppState {
    Running,
    Quitting,
}

pub struct App {
    pub state: AppState,
    /// Latest hardware snapshot shared with collector tasks.
    pub snapshot: Arc<Mutex<Snapshot>>,
    /// Rolling history for sparklines.
    pub history: History,
    /// Currently selected row in the process table.
    pub selected_process: usize,
}

impl App {
    pub fn new(snapshot: Arc<Mutex<Snapshot>>) -> Self {
        Self {
            state: AppState::Running,
            snapshot,
            history: History::new(),
            selected_process: 0,
        }
    }

    /// Called on every timer tick — pull latest snapshot into history buffers.
    pub fn tick(&mut self) {
        let snap = self.snapshot.lock().clone();
        self.history.update(&snap);
    }

    /// Handle a keyboard event.
    pub fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.state = AppState::Quitting;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_process = self.selected_process.saturating_add(1);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected_process = self.selected_process.saturating_sub(1);
            }
            _ => {}
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, AppState::Running)
    }
}
