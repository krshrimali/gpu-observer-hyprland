mod app;
mod collector;
mod history;
mod metrics;
mod ui;

use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use parking_lot::Mutex;
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use metrics::Snapshot;

/// How often collectors push new data.
const COLLECT_INTERVAL: Duration = Duration::from_millis(750);
/// Render / tick rate (~30 fps).
const TICK_RATE: Duration = Duration::from_millis(33);

#[tokio::main]
async fn main() -> Result<()> {
    // ── Shared state ──────────────────────────────────────────────────────────
    let shared = Arc::new(Mutex::new(Snapshot::default()));

    // ── Spawn background collectors ───────────────────────────────────────────
    let gpu_shared = shared.clone();
    tokio::spawn(async move {
        if let Err(e) = collector::gpu::gpu_collector(gpu_shared, COLLECT_INTERVAL).await {
            eprintln!("[gpu collector] {e}");
        }
    });

    let cpu_shared = shared.clone();
    tokio::spawn(async move {
        if let Err(e) = collector::cpu::cpu_collector(cpu_shared, COLLECT_INTERVAL).await {
            eprintln!("[cpu collector] {e}");
        }
    });

    let mem_shared = shared.clone();
    tokio::spawn(async move {
        if let Err(e) = collector::memory::memory_collector(mem_shared, COLLECT_INTERVAL).await {
            eprintln!("[mem collector] {e}");
        }
    });

    // ── Terminal setup ────────────────────────────────────────────────────────
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // ── App ───────────────────────────────────────────────────────────────────
    let mut app = App::new(shared);

    // Allow collectors a short warm-up before first render.
    tokio::time::sleep(Duration::from_millis(300)).await;

    // ── Main event loop ───────────────────────────────────────────────────────
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        // Poll for input; timeout keeps renders smooth even without input.
        let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }
                app.on_key(key.code);
            }
        }

        if !app.is_running() {
            break;
        }

        if last_tick.elapsed() >= TICK_RATE {
            app.tick();
            last_tick = Instant::now();
        }
    }

    // ── Teardown ──────────────────────────────────────────────────────────────
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
