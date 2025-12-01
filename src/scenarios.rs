use rand::Rng;

// Enum to hold different types of mission states
pub enum Scenario {
    Booting, // Added to prevent irrefutable pattern warning
    Navigation(NavState),
    // Future: Decryption(DecryptState),
}

impl Scenario {
    // Helper to get NavState if active (simplifies App code)
    pub fn as_nav(&self) -> Option<&NavState> {
        match self {
            Scenario::Navigation(s) => Some(s),
            _ => None,
        }
    }
}

pub struct NavState {
    pub player: (i32, i32),
    pub target: (i32, i32),
    pub grid_size: (i32, i32),
}

impl NavState {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        NavState {
            player: (2, 2),
            target: (rng.gen_range(10..28), rng.gen_range(5..15)),
            grid_size: (30, 18),
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let (w, h) = self.grid_size;
        let (px, py) = self.player;

        self.player = ((px + dx).clamp(1, w - 2), (py + dy).clamp(1, h - 2));
    }

    pub fn is_at_target(&self) -> bool {
        self.player == self.target
    }
}
