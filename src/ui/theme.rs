use ratatui::style::{Color, Modifier, Style};

// ── Palette ──────────────────────────────────────────────────────────────────

pub const NEON_CYAN: Color = Color::Rgb(0, 230, 210);
pub const NEON_GREEN: Color = Color::Rgb(50, 230, 50);
pub const NEON_YELLOW: Color = Color::Rgb(240, 210, 0);
pub const NEON_ORANGE: Color = Color::Rgb(255, 140, 0);
pub const NEON_RED: Color = Color::Rgb(240, 50, 50);
pub const NEON_PURPLE: Color = Color::Rgb(160, 60, 240);
pub const NEON_BLUE: Color = Color::Rgb(60, 140, 240);

pub const SURFACE: Color = Color::Rgb(18, 18, 28);
pub const SURFACE2: Color = Color::Rgb(28, 28, 42);
pub const BORDER: Color = Color::Rgb(55, 55, 80);
pub const BORDER_BRIGHT: Color = Color::Rgb(90, 90, 130);

pub const TEXT_BRIGHT: Color = Color::Rgb(220, 220, 245);
pub const TEXT_DIM: Color = Color::Rgb(120, 120, 150);
pub const TEXT_MUTED: Color = Color::Rgb(70, 70, 95);

pub const GAUGE_EMPTY: Color = Color::Rgb(35, 35, 50);

// ── Dynamic colour helpers ────────────────────────────────────────────────────

/// Temperature: green → yellow → orange → red
pub fn temp_color(celsius: u32) -> Color {
    match celsius {
        0..=54 => NEON_GREEN,
        55..=69 => NEON_YELLOW,
        70..=84 => NEON_ORANGE,
        _ => NEON_RED,
    }
}

/// Utilisation %: cyan → yellow → orange → red
pub fn util_color(pct: u32) -> Color {
    match pct {
        0..=59 => NEON_CYAN,
        60..=79 => NEON_YELLOW,
        80..=94 => NEON_ORANGE,
        _ => NEON_RED,
    }
}

/// Same but takes f32
pub fn util_color_f(pct: f32) -> Color {
    util_color(pct as u32)
}

// ── Reusable styles ───────────────────────────────────────────────────────────

pub fn label_style() -> Style {
    Style::default().fg(TEXT_DIM)
}

pub fn section_title_style() -> Style {
    Style::default()
        .fg(BORDER_BRIGHT)
        .add_modifier(Modifier::BOLD)
}

pub fn dim_style() -> Style {
    Style::default().fg(TEXT_MUTED)
}
