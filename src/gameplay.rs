use std::process::Command;
use std::fs;
use crate::levels::mission_01::Mission01State;
use crate::levels::mission_02::Mission02State;

#[derive(Clone)]
pub enum MissionStatus {
    Locked,
    Active,
    Success,
    Failed(String),
}

#[derive(Clone)]
pub enum GameState {
    MainMenu,
    Mission01(Mission01State),
    Mission02(Mission02State),
}

#[derive(Clone)]
pub struct Mission {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub script_path: String,
    pub status: MissionStatus,
    pub binary_size: Option<u64>, // Stores size in bytes
}

impl Mission {
    pub fn new(id: u32, title: &str, description: &str, path: &str) -> Self {
        Mission {
            id,
            title: title.to_string(),
            description: description.to_string(),
            script_path: path.to_string(),
            status: MissionStatus::Active,
            binary_size: None,
        }
    }

    pub fn compile_binary(&mut self, output_name: &str) -> bool {
        let output = Command::new("rustc")
            .arg(&self.script_path)
            .arg("--color").arg("never")
            .arg("-o").arg(output_name)
            .output();

        match output {
            Ok(c) => {
                if c.status.success() {
                    self.status = MissionStatus::Success;
                    
                    // Capture binary size
                    if let Ok(metadata) = fs::metadata(output_name) {
                        self.binary_size = Some(metadata.len());
                    } else {
                        self.binary_size = None;
                    }
                    
                    true
                } else {
                    let stderr = String::from_utf8_lossy(&c.stderr);
                    self.status = MissionStatus::Failed(format!("COMPILATION FAILED:\n\n{}", stderr));
                    self.binary_size = None;
                    false
                }
            }
            Err(e) => {
                self.status = MissionStatus::Failed(format!("SYSTEM ERROR: {}", e));
                self.binary_size = None;
                false
            }
        }
    }
}
