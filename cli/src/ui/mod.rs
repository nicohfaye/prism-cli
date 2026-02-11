pub mod dashboard;
pub mod theme;

use ratatui::Frame;

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    dashboard::render(f, app);
}
