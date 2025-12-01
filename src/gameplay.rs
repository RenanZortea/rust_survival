use std::process::Command;
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
    Mission02(Mission02State), // NEW STATE
}

#[derive(Clone)]
pub struct Mission {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub script_path: String,
    pub status: MissionStatus,
}

impl Mission {
    pub fn new(id: u32, title: &str, description: &str, path: &str) -> Self {
        Mission {
            id,
            title: title.to_string(),
            description: description.to_string(),
            script_path: path.to_string(),
            status: MissionStatus::Active,
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
                    true
                } else {
                    let stderr = String::from_utf8_lossy(&c.stderr);
                    self.status = MissionStatus::Failed(format!("COMPILATION FAILED:\n\n{}", stderr));
                    false
                }
            }
            Err(e) => {
                self.status = MissionStatus::Failed(format!("SYSTEM ERROR: {}", e));
                false
            }
        }
    }
}
