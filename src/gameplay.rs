use crate::levels::mission_01::Mission01State;
use crate::levels::mission_02::Mission02State;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Clone, PartialEq)]
pub enum MissionStatus {
    Locked,
    Active,
    Success,
    Failed(String), // Contains the compiler error message
}

pub struct Mission {
    pub id: u32,
    pub title: &'static str,
    pub description: &'static str,
    pub path: &'static str,
    pub status: MissionStatus,
    // Added to satisfy UI requirements (Option<u64> allows None if no binary exists)
    pub binary_size: Option<u64>,
}

impl Mission {
    pub fn new(
        id: u32,
        title: &'static str,
        description: &'static str,
        path: &'static str,
    ) -> Self {
        Self {
            id,
            title,
            description,
            path,
            status: MissionStatus::Active,
            binary_size: None,
        }
    }

    /// Attempts to compile the mission source code located at `self.path`.
    pub fn compile_binary(&mut self, output_name: &str) -> bool {
        let source_path = Path::new(self.path);

        // 1. Check if file exists locally
        if !source_path.exists() {
            self.status = MissionStatus::Failed(format!(
                "ERROR: File not found: {}\n\nDid you delete it? Run --init to restore.",
                self.path
            ));
            self.binary_size = None;
            return false;
        }

        // 2. Invoke rustc
        let output = Command::new("rustc")
            .arg(source_path)
            .arg("-o")
            .arg(output_name)
            .output();

        match output {
            Ok(o) => {
                if o.status.success() {
                    self.status = MissionStatus::Success;

                    // Update binary size for the UI
                    if let Ok(metadata) = fs::metadata(output_name) {
                        self.binary_size = Some(metadata.len());
                    }

                    return true;
                } else {
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    self.status = MissionStatus::Failed(stderr.to_string());
                    self.binary_size = None;
                    return false;
                }
            }
            Err(e) => {
                self.status = MissionStatus::Failed(format!(
                    "CRITICAL ERROR: Could not run 'rustc'.\nIs Rust installed?\nDetails: {}",
                    e
                ));
                self.binary_size = None;
                return false;
            }
        }
    }
}

pub enum GameState {
    MainMenu,
    Mission01(Mission01State),
    Mission02(Mission02State),
}
