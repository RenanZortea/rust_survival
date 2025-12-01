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

    loop {
        // HACK: Calculate line count for scrollbar using the exact same text generator as UI
        let log_text = app.get_log_content();
        app.log_line_count = log_text.lines().count() as u16;
        app.scroll_state = app.scroll_state.content_length(app.log_line_count as usize);

        terminal.draw(|f| ui::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.current_screen {
                    CurrentScreen::MainMenu => match key.code {
                        KeyCode::Up => app.menu_previous(),
                        KeyCode::Down => app.menu_next(),
                        KeyCode::Enter => {
                            if app.selected_item_index == 0 {
                                app.start_game();
                            } else {
                                app.current_screen = CurrentScreen::Exiting;
                            }
                        }
                        KeyCode::Esc => app.current_screen = CurrentScreen::Exiting,
                        _ => {}
                    },
                    CurrentScreen::Gameplay => {
                        app.handle_gameplay_input(key.code);
                    },
                    CurrentScreen::Exiting => break,
                }
            }
        }
        if let CurrentScreen::Exiting = app.current_screen {
            break;
        }
    }
    tui::restore()?;
    Ok(())
}
