use tauri;
//use tauri::async_runtime::Mutex;

struct AudioDriver {
    is_connected: bool,
    can_send_audio: bool,
    server_ip:  String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let driver = AudioDriver {
        is_connected: false,
        can_send_audio: false,
        server_ip: "localhost".to_string(),
    };
    
    tauri::Builder::default()
        .manage(driver)
        .plugin(tauri_plugin_shell::init())
        //.invoke_handler(tauri::generate_handler![sendAudio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
