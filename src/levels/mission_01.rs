use rand::Rng;
use std::process::Command;

#[derive(Clone)]
pub struct Mission01State {
    pub player_x: i32,
    pub player_y: i32,
    pub target_x: i32,
    pub target_y: i32,
    pub grid_width: i32,
    pub grid_height: i32,
    
    pub gps_output: String,
    pub is_gps_compiled: bool,
    pub is_finished: bool, // NEW: Track completion
}

impl Mission01State {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Mission01State {
            player_x: 2,
            player_y: 2,
            target_x: rng.gen_range(10..28),
            target_y: rng.gen_range(5..15),
            grid_width: 30,
            grid_height: 18,
            gps_output: "NO_SIGNAL".to_string(),
            is_gps_compiled: false,
            is_finished: false,
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        if self.is_finished { return; } // Freeze movement if done

        let new_x = (self.player_x + dx).clamp(1, self.grid_width - 2);
        let new_y = (self.player_y + dy).clamp(1, self.grid_height - 2);
        self.player_x = new_x;
        self.player_y = new_y;

        self.update_gps();
    }

    pub fn update_gps(&mut self) {
        if self.player_x == self.target_x && self.player_y == self.target_y {
            self.gps_output = "TARGET_ACQUIRED! SHELTER FOUND.".to_string();
            self.is_finished = true; // Trigger completion
            return;
        }

        if self.is_gps_compiled {
            self.gps_output = self.run_gps_binary(self.player_x, self.player_y, self.target_x, self.target_y);
        } else {
            self.gps_output = "ERR: FIRMWARE MISSING".to_string();
        }
    }

    fn run_gps_binary(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> String {
        let output = Command::new("./user_gps_bin")
            .arg(x1.to_string())
            .arg(y1.to_string())
            .arg(x2.to_string())
            .arg(y2.to_string())
            .output();

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
            .arg("0").arg("0").arg("3").arg("4")
            .output();

        match output {
            Ok(c) => {
                let stdout = String::from_utf8_lossy(&c.stdout);
                let stderr = String::from_utf8_lossy(&c.stderr);

                if !c.status.success() {
                    return Err(format!("RUNTIME ERROR:\n{}\n{}", stderr, stdout));
                }
                
                let val = stdout.trim();
                if val == "5.00" {
                    Ok(())
                } else {
                    Err(format!("LOGIC ERROR: Expected 5.00, got '{}'", val))
                }
            }
            Err(e) => Err(format!("System Error: {}", e)),
        }
    }
}
