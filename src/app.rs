use crate::gameplay::{Mission, GameState, MissionStatus};
use crate::levels::mission_01::Mission01State;
use ratatui::widgets::ScrollbarState;

pub enum CurrentScreen {
    MainMenu,
    Gameplay,
    Exiting,
}

#[derive(Clone, Copy)]
pub enum MenuItem {
    Start,
    Quit,
}

impl MenuItem {
    pub fn all() -> Vec<MenuItem> { vec![MenuItem::Start, MenuItem::Quit] }
    pub fn label(&self) -> &str {
        match self {
            MenuItem::Start => "BOOT_SEQUENCE",
            MenuItem::Quit => "POWER_DOWN",
        }
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub selected_item_index: usize,
    
    // --- MISSION DATA ---
    pub active_mission: Mission,
    pub state: GameState, 

    // --- UI STATE ---
    pub current_tab: usize, 
    pub vertical_scroll: u16,
    pub scroll_state: ScrollbarState,
    pub log_line_count: u16, 
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::MainMenu,
            selected_item_index: 0,
            
            active_mission: Mission::new(
                1,
                "FOG NAVIGATOR",
                "Fix the GPS firmware. Use the distance readout to find the hidden bunker.",
                "missions/01_shelter.rs",
            ),
            state: GameState::MainMenu,

            current_tab: 0,
            vertical_scroll: 0,
            scroll_state: ScrollbarState::default(),
            log_line_count: 0,
        }
    }

    /// SINGLE SOURCE OF TRUTH for Log Text
    pub fn get_log_content(&self) -> String {
        match &self.active_mission.status {
            MissionStatus::Failed(err) => err.clone(),
            MissionStatus::Success => "COMPILATION SUCCESSFUL.\n\nFIRMWARE UPLOADED.\nSYSTEM READY.".to_string(),
            _ => "NO LOGS AVAILABLE.\nPRESS 'C' TO COMPILE.".to_string(),
        }
    }

    pub fn start_game(&mut self) {
        self.current_screen = CurrentScreen::Gameplay;
        self.state = GameState::Mission01(Mission01State::new());
    }

    pub fn menu_next(&mut self) {
        let max = MenuItem::all().len() - 1;
        if self.selected_item_index < max { self.selected_item_index += 1; } else { self.selected_item_index = 0; }
    }
    
    pub fn menu_previous(&mut self) {
        if self.selected_item_index > 0 { self.selected_item_index -= 1; } else { self.selected_item_index = MenuItem::all().len() - 1; }
    }

    pub fn toggle_tab(&mut self) {
        self.current_tab = if self.current_tab == 0 { 1 } else { 0 };
    }

    pub fn scroll_text(&mut self, up: bool) {
        if up {
            self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
        } else {
            if self.vertical_scroll < self.log_line_count.saturating_sub(1) {
                self.vertical_scroll += 1;
            }
        }
        self.scroll_state = self.scroll_state.position(self.vertical_scroll as usize);
    }

    pub fn compile_mission_code(&mut self) {
        let success = self.active_mission.compile_binary("user_gps_bin");
        
        if success {
            match &mut self.state {
                GameState::Mission01(s) => {
                    s.is_gps_compiled = true;
                    s.update_gps(); 
                },
                _ => {}
            }
            self.current_tab = 0; 
        } else {
            // IF FAIL: Disable GPS, Switch to Logs, Reset Scroll
            match &mut self.state {
                GameState::Mission01(s) => s.is_gps_compiled = false,
                _ => {}
            }
            self.current_tab = 1; 
            self.vertical_scroll = 0;
        }
    }

    pub fn handle_gameplay_input(&mut self, key_code: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode;

        match key_code {
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.compile_mission_code();
                return;
            }
            KeyCode::Tab => {
                self.toggle_tab();
                return;
            }
            KeyCode::Esc => {
                self.current_screen = CurrentScreen::MainMenu;
                return;
            }
            _ => {}
        }

        if self.current_tab == 1 {
            match key_code {
                KeyCode::Up => self.scroll_text(true),
                KeyCode::Down => self.scroll_text(false),
                KeyCode::PageUp => for _ in 0..5 { self.scroll_text(true) },
                KeyCode::PageDown => for _ in 0..5 { self.scroll_text(false) },
                _ => {}
            }
            return;
        }

        match &mut self.state {
            GameState::Mission01(mission_state) => {
                match key_code {
                    KeyCode::Up => mission_state.move_player(0, -1),
                    KeyCode::Down => mission_state.move_player(0, 1),
                    KeyCode::Left => mission_state.move_player(-1, 0),
                    KeyCode::Right => mission_state.move_player(1, 0),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
