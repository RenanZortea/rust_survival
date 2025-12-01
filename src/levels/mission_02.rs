use std::process::Command;

#[derive(Clone)]
pub struct Mission02State {
    pub frequency_output: String,
    pub is_compiled: bool,
    pub is_finished: bool,
}

impl Mission02State {
    pub fn new() -> Self {
        Mission02State {
            frequency_output: "NO_CARRIER".to_string(),
            is_compiled: false,
            is_finished: false,
        }
    }

    pub fn check_signal(&mut self) {
        if self.is_compiled {
            // Run the user's radio decoder
            // We expect it to print "742.5" (example frequency)
            let output = Command::new("./user_gps_bin").output(); // Reusing the bin name for simplicity

            match output {
                Ok(c) => {
                    let out = String::from_utf8_lossy(&c.stdout).trim().to_string();
                    if out.contains("742.5") {
                        self.frequency_output =
                            "SIGNAL LOCKED: 742.5 MHz [ENCRYPTED STREAM RECEIVED]".to_string();
                        self.is_finished = true;
                    } else {
                        self.frequency_output = format!("NOISE: '{}' (Expected 742.5)", out);
                    }
                }
                Err(_) => self.frequency_output = "HARDWARE_FAILURE".to_string(),
            }
        }
    }
}
