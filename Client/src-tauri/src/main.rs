// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audiodriver;

fn main() {
    //audiodriver::run_audio_debugger();
    //discord2_lib::run();

    match audiodriver::AudioDriver::new() {
        Ok(driver) => {
            driver.get_input_devices();
            driver.get_output_devices();
        }
        Err(e) => {
            eprintln!("Error initializing AudioDriver: {}", e);
        }
    }
}
