use crate::gameplay::Mission;
use rand::Rng;
use ratatui::widgets::ScrollbarState; // Import this!

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
            MenuItem::Start => "BOOT_SEQUENCE (Find Shelter)",
            MenuItem::Quit => "POWER_DOWN",
        }
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub selected_item_index: usize,
    pub active_mission: Mission,
    
    // --- GAMEPLAY STATE ---
    pub player_x: i32,
    pub player_y: i32,
    pub target_x: i32,
    pub target_y: i32,
    pub grid_width: i32,
    pub grid_height: i32,
    pub gps_output: String, 
    pub is_gps_compiled: bool,

    // --- UI STATE ---
    pub current_tab: usize, // 0 = Nav, 1 = Logs
    pub vertical_scroll: u16,
    pub scroll_state: ScrollbarState,
    pub log_line_count: u16, // To calculate scroll limits
}

impl App {
    pub fn new() -> App {
        let mut rng = rand::thread_rng();
        
        App {
            current_screen: CurrentScreen::MainMenu,
            selected_item_index: 0,
            active_mission: Mission::new(
                1,
                "FOG NAVIGATOR",
                "Fix the GPS firmware. Use the distance readout to find the hidden bunker.",
                "missions/01_shelter.rs",
            ),
            player_x: 2,
            player_y: 2,
            target_x: rng.gen_range(10..28), 
            target_y: rng.gen_range(5..15),
            grid_width: 30,
            grid_height: 18,
            gps_output: "NO_SIGNAL".to_string(),
            is_gps_compiled: false,
            
            // UI Init
            current_tab: 0,
            vertical_scroll: 0,
            scroll_state: ScrollbarState::default(),
            log_line_count: 0,
        }
    }

    // ... (Keep menu_next/previous exactly as before) ...
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
            // Simple clamp based on line count
            if self.vertical_scroll < self.log_line_count.saturating_sub(1) {
                self.vertical_scroll += 1;
            }
        }
        self.scroll_state = self.scroll_state.position(self.vertical_scroll as usize);
    }

    pub fn compile_mission_code(&mut self) {
        self.gps_output = "COMPILING...".to_string(); // Immediate feedback
        let success = self.active_mission.compile_binary();
        self.is_gps_compiled = success;
        
        if success {
            self.gps_output = "FIRMWARE_UPDATED.".to_string();
            // Switch to Map automatically on success
            self.current_tab = 0; 
        } else {
            self.gps_output = "COMPILATION_ERROR".to_string();
            // Switch to Logs automatically on failure so they see the error
            self.current_tab = 1; 
            self.vertical_scroll = 0;
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        // Only allow movement if in Navigation tab
        if self.current_tab != 0 { return; }

        let new_x = (self.player_x + dx).clamp(1, self.grid_width - 2);
        let new_y = (self.player_y + dy).clamp(1, self.grid_height - 2);
        self.player_x = new_x;
        self.player_y = new_y;

        if self.player_x == self.target_x && self.player_y == self.target_y {
            self.gps_output = "TARGET_ACQUIRED! SHELTER FOUND.".to_string();
            return;
        }

        if self.is_gps_compiled {
            let output = self.active_mission.run_binary(
                self.player_x as f64, self.player_y as f64, 
                self.target_x as f64, self.target_y as f64
            );
            self.gps_output = format!("DIST: {}m", output.trim());
        } else {
            self.gps_output = "ERR: FIRMWARE MISSING".to_string();
        }
    }
}
