mod app;
mod gameplay;
mod tui;
mod ui;

use anyhow::Result;
use app::{App, CurrentScreen};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::new();

    loop {
        // HACK: Update content length for scrollbar before drawing
        let log_text = if let gameplay::MissionStatus::Failed(e) = &app.active_mission.status {
            e
        } else {
            ""
        };
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
                                app.current_screen = CurrentScreen::Gameplay;
                            } else {
                                app.current_screen = CurrentScreen::Exiting;
                            }
                        }
                        KeyCode::Esc => app.current_screen = CurrentScreen::Exiting,
                        _ => {}
                    },
                    CurrentScreen::Gameplay => match key.code {
                        // Global Gameplay Keys
                        KeyCode::Char('c') | KeyCode::Char('C') => app.compile_mission_code(),
                        KeyCode::Tab => app.toggle_tab(),
                        KeyCode::Esc => app.current_screen = CurrentScreen::MainMenu,

                        // Context Specific Keys
                        _ => {
                            if app.current_tab == 0 {
                                // NAVIGATION MODE
                                match key.code {
                                    KeyCode::Up => app.move_player(0, -1),
                                    KeyCode::Down => app.move_player(0, 1),
                                    KeyCode::Left => app.move_player(-1, 0),
                                    KeyCode::Right => app.move_player(1, 0),
                                    _ => {}
                                }
                            } else {
                                // DEBUG MODE (SCROLLING)
                                match key.code {
                                    KeyCode::Up => app.scroll_text(true),
                                    KeyCode::Down => app.scroll_text(false),
                                    KeyCode::PageUp => {
                                        for _ in 0..5 {
                                            app.scroll_text(true);
                                        }
                                    }
                                    KeyCode::PageDown => {
                                        for _ in 0..5 {
                                            app.scroll_text(false);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
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
