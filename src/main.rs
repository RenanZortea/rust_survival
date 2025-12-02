mod app;
mod gameplay;
mod tui;
mod ui;
mod levels;

use anyhow::Result;
use app::{App, CurrentScreen};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::new();

// Example loop in main.rs
loop {
    terminal.draw(|f| ui::ui(f, &app))?;
    
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            app.handle_input(key.code);
            if let CurrentScreen::Exiting = app.current_screen {
                break;
            }
        }
    }
    }
    tui::restore()?;
    Ok(())
}
