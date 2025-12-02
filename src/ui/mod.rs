pub mod menu;
pub mod mission_01;
pub mod mission_02;
pub mod shared;

use crate::app::{App, CurrentScreen};
use ratatui::Frame;

pub fn ui(f: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::MainMenu => menu::render_main_menu(f, app),
        CurrentScreen::LevelSelection => menu::render_level_selection(f, app),
        CurrentScreen::Gameplay => shared::render_gameplay_shell(f, app),
        CurrentScreen::Exiting => {}
    }
}
