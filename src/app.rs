use crate::gameplay::{Mission, GameState, MissionStatus};
use crate::levels::mission_01::Mission01State;
use crate::levels::mission_02::Mission02State;
use ratatui::widgets::ScrollbarState;
use crossterm::event::KeyCode;

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
    
    pub active_mission: Mission,
    pub state: GameState, 

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
            active_mission: Mission::new(1, "FOG NAVIGATOR", "Fix the GPS.", "missions/01_shelter.rs"),
            state: GameState::MainMenu,
            current_tab: 0,
            vertical_scroll: 0,
            scroll_state: ScrollbarState::default(),
            log_line_count: 0,
        }
    }

    pub fn get_log_content(&self) -> String {
        match &self.active_mission.status {
            MissionStatus::Failed(err) => err.clone(),
            MissionStatus::Success => "COMPILATION SUCCESSFUL.\n\nTESTS PASSED.\nSYSTEM READY.".to_string(),
            _ => "NO LOGS.".to_string(),
        }
    }

    pub fn start_game(&mut self) {
        self.current_screen = CurrentScreen::Gameplay;
        self.state = GameState::Mission01(Mission01State::new());
        self.active_mission = Mission::new(1, "FOG NAVIGATOR", "Fix GPS.", "missions/01_shelter.rs");
    }

    // NEW: Transition Logic
    pub fn load_mission_02(&mut self) {
        self.active_mission = Mission::new(
            2, 
            "RADIO SILENCE", 
            "The bunker radio is dead. Fix the frequency scanner to call for help.", 
            "missions/02_radio.rs"
        );
        self.state = GameState::Mission02(Mission02State::new());
        self.current_tab = 0;
        self.active_mission.status = MissionStatus::Active; // Reset log status
    }

    pub fn menu_next(&mut self) {
        let max = MenuItem::all().len() - 1;
        if self.selected_item_index < max { self.selected_item_index += 1; } else { self.selected_item_index = 0; }
    }
    pub fn menu_previous(&mut self) {
        if self.selected_item_index > 0 { self.selected_item_index -= 1; } else { self.selected_item_index = MenuItem::all().len() - 1; }
    }
    pub fn toggle_tab(&mut self) { self.current_tab = if self.current_tab == 0 { 1 } else { 0 }; }

    pub fn scroll_text(&mut self, up: bool) {
        if up { self.vertical_scroll = self.vertical_scroll.saturating_sub(1); } 
        else if self.vertical_scroll < self.log_line_count.saturating_sub(1) { self.vertical_scroll += 1; }
        self.scroll_state = self.scroll_state.position(self.vertical_scroll as usize);
    }

    pub fn compile_mission_code(&mut self) {
        let compiled = self.active_mission.compile_binary("user_gps_bin");
        if !compiled { self.handle_fail(); return; }

        match &mut self.state {
            GameState::Mission01(s) => {
                if let Err(e) = Mission01State::verify_integrity() {
                    self.active_mission.status = MissionStatus::Failed(e);
                    self.handle_fail();
                    return;
                }
                s.is_gps_compiled = true;
                s.update_gps();
            },
            GameState::Mission02(s) => {
                // Mission 2 compilation logic
                s.is_compiled = true;
                s.check_signal();
            }
            _ => {}
        }
        
        self.active_mission.status = MissionStatus::Success;
        self.current_tab = 0;
    }

    fn handle_fail(&mut self) {
        match &mut self.state {
            GameState::Mission01(s) => s.is_gps_compiled = false,
            GameState::Mission02(s) => s.is_compiled = false,
            _ => {}
        }
        self.current_tab = 1; 
        self.vertical_scroll = 0;
    }

    pub fn handle_gameplay_input(&mut self, key_code: KeyCode) {
        // 1. Check for Level Transition (Press Enter when Finished)
        match &self.state {
            GameState::Mission01(s) if s.is_finished => {
                if key_code == KeyCode::Enter {
                    self.load_mission_02();
                    return;
                }
            },
            GameState::Mission02(s) if s.is_finished => {
                if key_code == KeyCode::Enter {
                    // Mission 3 would go here, or Exit
                    self.current_screen = CurrentScreen::Exiting;
                    return;
                }
            }
            _ => {}
        }

        // 2. Standard Input
        match key_code {
            KeyCode::Char('c') | KeyCode::Char('C') => { self.compile_mission_code(); return; }
            KeyCode::Tab => { self.toggle_tab(); return; }
            KeyCode::Esc => { self.current_screen = CurrentScreen::MainMenu; return; }
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

        // 3. Level Specific Input
        match &mut self.state {
            GameState::Mission01(s) => {
                match key_code {
                    KeyCode::Up => s.move_player(0, -1),
                    KeyCode::Down => s.move_player(0, 1),
                    KeyCode::Left => s.move_player(-1, 0),
                    KeyCode::Right => s.move_player(1, 0),
                    _ => {}
                }
            }
            GameState::Mission02(_) => {
                // Mission 2 has no movement controls, just 'C' to compile
            }
            _ => {}
        }
    }
}
