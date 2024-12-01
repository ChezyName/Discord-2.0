// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Create a Tokio runtime
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    // Run the Tauri application and audio debug function
    runtime.block_on(async {
        // Run the asynchronous execute_audio_debug function
        if let Err(err) = discord2_lib::execute_audio_debug().await {
            eprintln!("Error during audio debug: {}", err);
        }
        
        // Run the Tauri application
        discord2_lib::run();
    });
}
