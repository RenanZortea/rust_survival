use rand::Rng;
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Ground,
    Tree,
    Rock,
    Ruin,
}

#[derive(Clone)]
pub struct Mission01State {
    pub player_x: i32,
    pub player_y: i32,
    pub target_x: i32,
    pub target_y: i32,

    // NEW: Huge map dimensions
    pub grid_width: i32,
    pub grid_height: i32,

    pub terrain: Vec<Vec<TileType>>,

    pub gps_output: String,
    pub is_gps_compiled: bool,
    pub is_finished: bool,

    pub last_runtime: Option<Duration>,
}

impl Mission01State {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        // HUGE MAP: 100x100 (10,000 tiles)
        let width = 100;
        let height = 100;

        let mut terrain = vec![vec![TileType::Ground; width as usize]; height as usize];
        for y in 0..height {
            for x in 0..width {
                let roll = rng.gen_range(0..100);
                terrain[y as usize][x as usize] = match roll {
                    0..=15 => TileType::Tree, // More trees (forest density)
                    16..=18 => TileType::Rock,
                    19..=20 => TileType::Ruin,
                    _ => TileType::Ground,
                };
            }
        }

        let start_x = 5;
        let start_y = 5;
        terrain[start_y as usize][start_x as usize] = TileType::Ground;

        // Target is far away now
        let target_x = rng.gen_range(50..95);
        let target_y = rng.gen_range(50..95);
        terrain[target_y as usize][target_x as usize] = TileType::Ground;

        Mission01State {
            player_x: start_x,
            player_y: start_y,
            target_x,
            target_y,
            grid_width: width,
            grid_height: height,
            terrain,
            gps_output: "NO_SIGNAL".to_string(),
            is_gps_compiled: false,
            is_finished: false,
            last_runtime: None,
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        if self.is_finished {
            return;
        }

        let new_x = (self.player_x + dx).clamp(0, self.grid_width - 1);
        let new_y = (self.player_y + dy).clamp(0, self.grid_height - 1);

        // Optional: Collision (Uncomment to enable hard collisions)
        /*
        if self.terrain[new_y as usize][new_x as usize] == TileType::Rock {
            self.gps_output = "PATH_BLOCKED".to_string();
            return;
        }
        */

        self.player_x = new_x;
        self.player_y = new_y;

        self.update_gps();
    }

    pub fn update_gps(&mut self) {
        if self.player_x == self.target_x && self.player_y == self.target_y {
            self.gps_output = "TARGET_ACQUIRED! SHELTER FOUND.".to_string();
            self.is_finished = true;
            return;
        }

        if self.is_gps_compiled {
            self.gps_output =
                self.run_gps_binary(self.player_x, self.player_y, self.target_x, self.target_y);
        } else {
            self.gps_output = "ERR: FIRMWARE MISSING".to_string();
        }
    }

    fn run_gps_binary(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> String {
        let start_time = Instant::now();
        let output = Command::new("./user_gps_bin")
            .arg(x1.to_string())
            .arg(y1.to_string())
            .arg(x2.to_string())
            .arg(y2.to_string())
            .output();

        self.last_runtime = Some(start_time.elapsed());

        match output {
            Ok(c) => {
                if c.status.success() {
                    let out = String::from_utf8_lossy(&c.stdout).to_string();
                    format!("DIST: {}m", out.trim())
                } else {
                    "CRASH".to_string()
                }
            }
            Err(_) => "EXEC_ERR".to_string(),
        }
    }

    pub fn verify_integrity() -> Result<(), String> {
        let output = Command::new("./user_gps_bin")
            .arg("0")
            .arg("0")
            .arg("3")
            .arg("4")
            .output();
        match output {
            Ok(c) => {
                if !c.status.success() {
                    return Err("Runtime Error".to_string());
                }
                if String::from_utf8_lossy(&c.stdout).trim() == "5.00" {
                    Ok(())
                } else {
                    Err("Logic Error".to_string())
                }
            }
            Err(e) => Err(format!("System Error: {}", e)),
        }
    }
}
