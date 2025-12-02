mod app;
mod gameplay;
mod tui;
mod ui;
mod levels;
mod setup; 

use anyhow::Result;
use app::{App, CurrentScreen};
use clap::Parser;
use crossterm::event::{self, Event};

// Define CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Initialize the mission files in the current directory
    #[arg(long)]
    init: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Handle the initialization command
    if args.init {
        setup::initialize_workspace()?;
        return Ok(());
    }

    // 2. Safety Check: Ensure missions exist
    if !std::path::Path::new("missions").exists() {
        eprintln!("ERROR: Critical Mission Files Missing.");
        eprintln!("Run 'rust_survival --init' to generate the workspace.");
        return Ok(());
    }

    // 3. Start the Game Loop
    let mut terminal = tui::init()?;
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::ui(f, &app))?;
        
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
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
