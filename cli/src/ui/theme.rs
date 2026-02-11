use ratatui::style::{Color, Modifier, Style};

// startup banner
pub const BANNER: &str = r"
       ___           ___                       ___           ___
      /\  \         /\  \          ___        /\  \         /\__\
     /::\  \       /::\  \        /\  \      /::\  \       /::|  |
    /:/\:\  \     /:/\:\  \       \:\  \    /:/\ \  \     /:|:|  |
   /::\~\:\  \   /::\~\:\  \      /::\__\  _\:\~\ \  \   /:/|:|__|__
  /:/\:\ \:\__\ /:/\:\ \:\__\  __/:/\/__/ /\ \:\ \ \__\ /:/ |::::\__\
  \/__\:\/:/  / \/_|::\/:/  / /\/:/  /    \:\ \:\ \/__/ \/__/~~/:/  /
       \::/  /     |:|::/  /  \::/__/      \:\ \:\__\         /:/  /
        \/__/      |:|\/__/    \:\__\       \:\/:/  /        /:/  /
                   |:|  |       \/__/        \::/  /        /:/  /
                    \|__|                     \/__/         \/__/
";

// base palette
pub const SURFACE: Color = Color::Rgb(28, 28, 38);
pub const BORDER: Color = Color::Rgb(58, 58, 78);
pub const BORDER_FOCUSED: Color = Color::Rgb(120, 120, 220);
pub const TEXT: Color = Color::Rgb(200, 200, 220);
pub const TEXT_DIM: Color = Color::Rgb(100, 100, 130);
pub const ACCENT: Color = Color::Rgb(130, 140, 255);

// status colors
pub const GREEN: Color = Color::Rgb(80, 220, 120);
pub const YELLOW: Color = Color::Rgb(240, 200, 60);
pub const RED: Color = Color::Rgb(240, 80, 80);

// styles
pub fn header() -> Style {
    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
}

pub fn table_header() -> Style {
    Style::default().fg(TEXT_DIM).add_modifier(Modifier::BOLD)
}

pub fn row_normal() -> Style {
    Style::default().fg(TEXT)
}

pub fn status_style(status: &str) -> Style {
    let color = match status {
        "Running" | "Succeeded" => GREEN,
        "Pending" | "ContainerCreating" => YELLOW,
        "Failed" | "CrashLoopBackOff" | "Error" | "ImagePullBackOff" => RED,
        _ => TEXT,
    };
    Style::default().fg(color)
}

pub fn highlight() -> Style {
    Style::default()
        .bg(SURFACE)
        .fg(TEXT)
        .add_modifier(Modifier::BOLD)
}

pub fn border(focused: bool) -> Style {
    let c = if focused { BORDER_FOCUSED } else { BORDER };
    Style::default().fg(c)
}
