use include_dir::{include_dir, Dir};
use std::path::Path;
use std::fs; // <--- Brought this back
use anyhow::{Result, Context};

// Embed the "missions" directory from the project root into the binary at compile time.
static MISSIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/missions");

pub fn initialize_workspace() -> Result<()> {
    let target_dir = Path::new("missions");

    if target_dir.exists() {
        println!("Checked 'missions' folder... exists.");
        println!("Skipping overwrite to protect your progress.");
    } else {
        println!("Initializing survival workspace...");
        
        // FIX: We must create the directory MANUALLY before extracting files into it.
        fs::create_dir_all(target_dir).context("Failed to create 'missions' directory")?;
        
        // Extract the embedded directory to the user's disk
        MISSIONS_DIR.extract(target_dir)
            .context("Failed to extract mission files")?;
            
        println!("✔ Created 'missions/' directory.");
        println!("✔ Extracted mission files.");
        println!("\nYou are now ready. Edit the files in 'missions/' and run the game to test them.");
    }

    Ok(())
}
