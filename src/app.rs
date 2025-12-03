use crate::gameplay::{GameState, Mission, MissionStatus};
use crate::levels::mission_01::Mission01State;
use crate::levels::mission_02::Mission02State;
use crossterm::event::KeyCode;
use ratatui::widgets::ScrollbarState;

pub enum CurrentScreen {
    MainMenu,
    LevelSelection,
    Gameplay,
    Exiting,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MenuItem {
    Start,
    SelectLevel,
    Quit,
}

impl MenuItem {
    pub fn all() -> Vec<MenuItem> {
        vec![MenuItem::Start, MenuItem::SelectLevel, MenuItem::Quit]
    }

    pub fn label(&self) -> &str {
        match self {
            MenuItem::Start => " > INITIATE_SURVIVAL",
            MenuItem::SelectLevel => " > MISSION_SELECT",
            MenuItem::Quit => " > POWER_DOWN",
        }
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub selected_item_index: usize,
    pub mission_selection_index: usize,
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
            mission_selection_index: 0,
            active_mission: Mission::new(
                1,
                "FOG NAVIGATOR",
                "Fix the GPS.",
                "missions/01_shelter.rs",
            ),
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
            MissionStatus::Success => {
                "COMPILATION SUCCESSFUL.\n\nTESTS PASSED.\nSYSTEM READY.".to_string()
            }
            _ => "NO LOGS.".to_string(),
        }
    }

    pub fn get_mission_list() -> Vec<(u32, &'static str, &'static str)> {
        vec![
            (1, "FOG NAVIGATOR", "Repair GPS to find shelter."),
            (2, "TOXIC FLOW", "Program the water purification unit."),
        ]
    }

    pub fn start_game(&mut self) {
        self.load_mission_01();
    }

    pub fn load_mission_01(&mut self) {
        self.current_screen = CurrentScreen::Gameplay;
        self.state = GameState::Mission01(Mission01State::new());
        self.active_mission =
            Mission::new(1, "FOG NAVIGATOR", "Fix GPS.", "missions/01_shelter.rs");
        self.current_tab = 0;
    }

    pub fn load_mission_02(&mut self) {
        self.current_screen = CurrentScreen::Gameplay;
        self.state = GameState::Mission02(Mission02State::new());
        self.active_mission = Mission::new(
            2,
            "TOXIC FLOW",
            "Water supply contaminated. Program the purification unit.",
            "missions/02_water.rs",
        );
        self.current_tab = 0;
    }

    pub fn start_selected_level(&mut self) {
        match self.mission_selection_index {
            0 => self.load_mission_01(),
            1 => self.load_mission_02(),
            _ => {}
        }
    }

    // --- NAVIGATION LOGIC ---

    pub fn menu_next(&mut self) {
        let max = MenuItem::all().len() - 1;
        if self.selected_item_index < max {
            self.selected_item_index += 1;
        } else {
            self.selected_item_index = 0;
        }
    }

    pub fn menu_previous(&mut self) {
        if self.selected_item_index > 0 {
            self.selected_item_index -= 1;
        } else {
            self.selected_item_index = MenuItem::all().len() - 1;
        }
    }

    pub fn level_select_next(&mut self) {
        let max = Self::get_mission_list().len() - 1;
        if self.mission_selection_index < max {
            self.mission_selection_index += 1;
        } else {
            self.mission_selection_index = 0;
        }
    }

    pub fn level_select_previous(&mut self) {
        let max = Self::get_mission_list().len() - 1;
        if self.mission_selection_index > 0 {
            self.mission_selection_index -= 1;
        } else {
            self.mission_selection_index = max;
        }
    }

    pub fn toggle_tab(&mut self) {
        self.current_tab = if self.current_tab == 0 { 1 } else { 0 };
    }

    pub fn scroll_text(&mut self, up: bool) {
        if up {
            self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
        } else if self.vertical_scroll < self.log_line_count.saturating_sub(1) {
            self.vertical_scroll += 1;
        }
        self.scroll_state = self.scroll_state.position(self.vertical_scroll as usize);
    }

    pub fn compile_mission_code(&mut self) {
        // Dynamic binary name based on the current mission
        let binary_name = match self.state {
            GameState::Mission01(_) => "user_gps_bin",
            GameState::Mission02(_) => "user_water_bin",
            _ => "temp_bin",
        };

        let compiled = self.active_mission.compile_binary(binary_name);
        if !compiled {
            self.handle_fail();
            return;
        }

        match &mut self.state {
            GameState::Mission01(s) => {
                if let Err(e) = Mission01State::verify_integrity() {
                    self.active_mission.status = MissionStatus::Failed(e);
                    self.handle_fail();
                    return;
                }
                s.is_gps_compiled = true;
                s.update_gps();
            }
            GameState::Mission02(s) => {
                s.is_compiled = true;
                s.check_water(); // Call the new water check logic
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

    // Consolidated Input Handler
    pub fn handle_input(&mut self, key_code: KeyCode) {
        match self.current_screen {
            CurrentScreen::MainMenu => match key_code {
                KeyCode::Up => self.menu_previous(),
                KeyCode::Down => self.menu_next(),
                KeyCode::Enter => match MenuItem::all()[self.selected_item_index] {
                    MenuItem::Start => self.start_game(),
                    MenuItem::SelectLevel => self.current_screen = CurrentScreen::LevelSelection,
                    MenuItem::Quit => self.current_screen = CurrentScreen::Exiting,
                },
                _ => {}
            },
            CurrentScreen::LevelSelection => match key_code {
                KeyCode::Up => self.level_select_previous(),
                KeyCode::Down => self.level_select_next(),
                KeyCode::Enter => self.start_selected_level(),
                KeyCode::Esc => self.current_screen = CurrentScreen::MainMenu,
                _ => {}
            },
            CurrentScreen::Gameplay => self.handle_gameplay_input(key_code),
            _ => {}
        }
    }

    fn handle_gameplay_input(&mut self, key_code: KeyCode) {
        // Check level transitions first
        match &self.state {
            GameState::Mission01(s) if s.is_finished && key_code == KeyCode::Enter => {
                self.load_mission_02();
                return;
            }
            GameState::Mission02(s) if s.is_finished && key_code == KeyCode::Enter => {
                self.current_screen = CurrentScreen::Exiting;
                return;
            }
            _ => {}
        }

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
                KeyCode::PageUp => {
                    for _ in 0..5 {
                        self.scroll_text(true)
                    }
                }
                KeyCode::PageDown => {
                    for _ in 0..5 {
                        self.scroll_text(false)
                    }
                }
                _ => {}
            }
            return;
        }

        // Mission specific controls
        if let GameState::Mission01(s) = &mut self.state {
            match key_code {
                KeyCode::Up => s.move_player(0, -1),
                KeyCode::Down => s.move_player(0, 1),
                KeyCode::Left => s.move_player(-1, 0),
                KeyCode::Right => s.move_player(1, 0),
                _ => {}
            }
        }
    }
}
