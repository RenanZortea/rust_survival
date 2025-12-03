use std::process::Command;

#[derive(Clone)]
pub struct Mission02State {
    pub turbidity: f32,
    pub ph: f32,
    pub output_log: String,
    pub is_compiled: bool,
    pub is_finished: bool,
}

impl Mission02State {
    pub fn new() -> Self {
        Mission02State {
            turbidity: 25.0, // Dirty water
            ph: 6.5,         // Slightly acidic
            output_log: "AWAITING FIRMWARE UPDATE...".to_string(),
            is_compiled: false,
            is_finished: false,
        }
    }

    pub fn check_water(&mut self) {
        if self.is_compiled {
            // We run the user's compiled binary with sensor data as arguments
            // Usage: ./user_water_bin <turbidity> <ph>
            let output = Command::new("./user_water_bin")
                .arg(self.turbidity.to_string())
                .arg(self.ph.to_string())
                .output();

            match output {
                Ok(c) => {
                    let out_str = String::from_utf8_lossy(&c.stdout).trim().to_string();

                    // Parse the user's output (expected: a float string like "4.50")
                    match out_str.parse::<f32>() {
                        Ok(user_val) => {
                            let expected = self.calculate_correct_chlorine(self.turbidity, self.ph);

                            // Check if the value is close enough (allow small float error)
                            if (user_val - expected).abs() < 0.1 {
                                self.output_log = format!(
                                    "INJECTION: {:.2} mg/L [TARGET MATCHED] -> WATER PURIFIED",
                                    user_val
                                );
                                self.is_finished = true;
                            } else {
                                self.output_log = format!(
                                    "INJECTION: {:.2} mg/L [UNSAFE] (Expected {:.2})",
                                    user_val, expected
                                );
                            }
                        }
                        Err(_) => {
                            self.output_log = format!("ERROR: Invalid Sensor Output '{}'", out_str);
                        }
                    }
                }
                Err(_) => self.output_log = "HARDWARE FAILURE: PUMP CONNECTION LOST".to_string(),
            }
        }
    }

    // This is the internal "Truth" logic to verify the user's code
    fn calculate_correct_chlorine(&self, turbidity: f32, ph: f32) -> f32 {
        let mut amount = turbidity / 10.0;
        if ph < 7.0 {
            amount += 2.0;
        } else if ph > 7.0 {
            amount -= 1.0;
        }
        if amount < 0.0 {
            amount = 0.0;
        }
        amount
    }
}
