// MISSION 01: FOG OF WAR
// DEVICE: WRIST_GPS_V2 (CLI FIRMWARE)
//
// CONTEXT:
// The radiation fog is too thick to see more than 1 meter.
// Your GPS hardware works, but the distance algorithm is corrupted.
//
// TASK:
// 1. Implement `calculate_distance` to return the Euclidean distance.
//    Formula: sqrt( (x2 - x1)^2 + (y2 - y1)^2 )
// 2. If the result is NaN (due to radiation bitflips), return 0.0 to prevent a crash.

use std::env;

fn main() {
    // The game engine passes coordinates as command line arguments
    // args: [binary_name, x1, y1, x2, y2]
    let args: Vec<String> = env::args().collect();

    // Safety check for arguments
    if args.len() < 5 {
        println!("ERROR: Hardware Malfunction (Missing Args)");
        return;
    }

    let x1: f64 = args[1].parse().unwrap_or(0.0);
    let y1: f64 = args[2].parse().unwrap_or(0.0);
    let x2: f64 = args[3].parse().unwrap_or(0.0);
    let y2: f64 = args[4].parse().unwrap_or(0.0);

    let distance = calculate_distance(x1, y1, x2, y2);

    // This print statement is what appears on your in-game HUD!
    println!("{:.2}", distance);
}

// --- EDIT THIS FUNCTION ---
fn calculate_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    // TODO: Fix the math.
    // Hint: Use (x2 - x1).powi(2) or (x2-x1) * (x2-x1)
    // let mut dx = (x2 - x1).powi(2);
    // let mut dy = (y2 - y1).powi(2);
    // let mut distance = (dx + dy).sqrt();
    // distance
}
