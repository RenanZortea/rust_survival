// ===================================================================================
//  SURVIVAL OS - MISSION LOG 02
// ===================================================================================
//  LOCATION:  Underground Bunker B-7
//  STATUS:    Shelter secured. Oxygen levels nominal.
//  CRITICAL:  Water supply contaminated.
//
//  You have successfully sealed the blast doors. The shelter is safe, but the 
//  previous occupants left the Water Purification Unit (WPU-7) in a debug state.
//  The heavy metal filters are clogged and the chemical injector has been wiped.
//  
//  To survive, you must reprogram the injector controller to neutralize the 
//  toxins based on real-time sensor data.
// ===================================================================================

// !!! [SYSTEM HARDWARE INTERFACE] - DO NOT EDIT THE CODE BELOW THIS LINE !!!
use std::env;

fn main() {
    // Simulate reading from hardware sensors
    let args: Vec<String> = env::args().collect();
    
    // Defaults for local testing if no args provided
    let turbidity = if args.len() > 1 { args[1].parse().unwrap_or(0.0) } else { 20.0 };
    let ph = if args.len() > 2 { args[2].parse().unwrap_or(7.0) } else { 7.0 };
    
    // Call the user's firmware logic
    let injection_amount = calculate_chlorine(turbidity, ph);
    
    // Output signal to the pump
    println!("{:.2}", injection_amount);
}
// !!! [END OF SYSTEM HARDWARE INTERFACE] - DO NOT EDIT THE CODE ABOVE THIS LINE !!!


// ===================================================================================
//  USER FIRMWARE SECTION
// ===================================================================================
//  TASK:
//  Calculate the correct 'chlorine_mg' based on sensor readings.
//
//  MANUAL PAGE 42:
//  1. Base Injection = Turbidity / 10.0
//  2. If Water is ACIDIC (pH < 7.0) -> Add 2.0 mg/L
//  3. If Water is BASIC  (pH > 7.0) -> Subtract 1.0 mg/L
//  4. Safety Protocol: Injection amount cannot be negative.
// ===================================================================================

fn calculate_chlorine(turbidity: f32, ph: f32) -> f32 {
    let mut amount = 0.0;

    // --- WRITE YOUR CODE BELOW ---
    
    // 1. Base injection
    amount = turbidity / 10.0;

    // 2. Adjust for pH
    if ph < 7.0 {
        amount += 2.0;
    } else if ph > 7.0 {
        amount -= 1.0;
    }

    // 3. Safety clamp (cannot be negative)
    if amount < 0.0 {
        amount = 0.0;
    }

    // --- END OF CODE ---

    return amount;
}
