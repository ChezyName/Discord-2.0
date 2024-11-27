use tauri::{AppHandle, Builder};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

#[derive(Default)]
struct AudioDriver {
    is_connected: bool,
    can_send_audio: bool,
    server_ip: String,
    socket: Option<tokio::net::UdpSocket>,
    user_name: String,
}

#[tauri::command]
fn start_audio_loop(state: tauri::State<Arc<Mutex<AudioDriver>>>) {
    println!("Running Audio Loop Checks");

    let driver_state = Arc::clone(&state);
    //Audio Loop
    tauri::async_runtime::spawn(async move {
        let mut driver = driver_state.lock().await;
        if driver.can_send_audio { 
            println!("Cannot Send Audio, Another Thread is Already Sending Audio");
            return;
        }

        // Create a UDP socket
        if !driver.socket.is_some() {
            let new_socket = match tokio::net::UdpSocket::bind("0.0.0.0:0").await {
                Ok(new_socket) => new_socket,
                Err(e) => {
                    eprintln!("Failed to bind UDP socket: {}", e);
                    return;
                }
            };
            driver.socket = Some(new_socket)
        }

        driver.can_send_audio = true;

        //Send Initial Data Like Username
        let server_ip = driver.server_ip.clone();
        let username = driver.user_name.to_owned().clone();
        let full = "username:".to_string() + &username;
        let data = full.as_bytes();

        if let Some(socket) = driver.socket.as_ref() {
            if let Err(e) = socket.send_to(data, &server_ip).await {
                eprintln!("Failed to send data: {}", e);
                return; // Exit the loop if sending fails
            }
        } else {
            eprintln!("Socket is not initialized. Cannot send data.");
            return; // Exit the loop if the socket is not initialized
        }

        drop(driver);
        
        loop {
            {
                //println!("Sending Audio Packet");
                let mut driver = driver_state.lock().await;

                //================================================================
                // Audio Sending
                // If can_send_audio is false, break the loop
                if !driver.can_send_audio {
                    println!("Stopping audio loop, can_send_audio is false");
                    driver.is_connected = false;
                    break;
                }

                // Clone server_ip to avoid holding the lock during async calls
                let server_ip = driver.server_ip.clone();
                let data = "Audio Packet".as_bytes();

                // Send audio data & recieve audio data
                if let Some(socket) = driver.socket.as_ref() {
                    if let Err(e) = socket.send_to(data, &server_ip).await {
                        eprintln!("Failed to send data: {}", e);
                        break; // Exit the loop if sending fails
                    }

                    let mut buf = [0; 1024];
                    if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
                        println!("{:?} bytes received from {:?}", len, addr);
                    }
                } else {
                    eprintln!("Socket is not initialized. Cannot send data.");
                    break; // Exit the loop if the socket is not initialized
                }

                drop(driver);
            }
        }
    });
}

#[tauri::command]
fn stop_audio_loop(state: tauri::State<Arc<Mutex<AudioDriver>>>) {
    let driver_state = Arc::clone(&state);
    tauri::async_runtime::spawn(async move {
        let mut driver = driver_state.lock().await;
        driver.can_send_audio = false;
        drop(driver);
        drop(driver_state);
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let driver = Arc::new(Mutex::new(AudioDriver {
        is_connected: false,
        can_send_audio: false, // Set to true for testing
        server_ip: "127.0.0.1:3000".to_string(),
        socket: None,
        user_name: "Username Not Set".to_string(),
    }));

    tauri::Builder::default()
        .manage(driver)
        .invoke_handler(tauri::generate_handler![stop_audio_loop,start_audio_loop])
        //.invoke_handler(tauri::generate_handler![stop_audio_loop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
