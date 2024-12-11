// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audiodriver;

fn main() {
    audiodriver::run_audio_debugger();
    //discord2_lib::run();
}
