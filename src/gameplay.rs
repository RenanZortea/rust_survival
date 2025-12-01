use std::process::Command;

pub enum MissionStatus {
    Locked,
    Active,
    Success,
    Failed(String),
}

pub struct Mission {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub path: String,
    pub status: MissionStatus,
}

impl Mission {
    pub fn new(id: u32, title: &str, description: &str, path: &str) -> Self {
        Mission {
            id,
            title: title.to_string(),
            description: description.to_string(),
            path: path.to_string(),
            status: MissionStatus::Active,
        }
    }

    /// Compiles the user's file into a temporary binary `user_gps_bin`
    pub fn compile_binary(&mut self) -> bool {
        let output = Command::new("rustc")
            .arg(&self.path)
            .arg("-o")
            .arg("user_gps_bin") // The binary name
            .output();

        match output {
            Ok(c) => {
                if c.status.success() {
                    self.status = MissionStatus::Active;
                    true
                } else {
                    let stderr = String::from_utf8_lossy(&c.stderr);
                    self.status = MissionStatus::Failed(format!("COMPILE ERROR:\n{}", stderr));
                    false
                }
            }
            Err(_) => {
                self.status = MissionStatus::Failed("Rust compiler not found!".to_string());
                false
            }
        }
    }

    /// Runs the compiled binary with coordinates as CLI args
    pub fn run_binary(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> String {
        let output = Command::new("./user_gps_bin")
            .arg(x1.to_string())
            .arg(y1.to_string())
            .arg(x2.to_string())
            .arg(y2.to_string())
            .output();

        match output {
            Ok(c) => {
                if c.status.success() {
                    String::from_utf8_lossy(&c.stdout).to_string()
                } else {
                    "CRASH".to_string()
                }
            }
            Err(_) => "EXEC_ERR".to_string(),
        }
    }
}
