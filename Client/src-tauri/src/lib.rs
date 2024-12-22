use tauri::{AppHandle, Builder};
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{InputCallbackInfo, OutputCallbackInfo, StreamConfig, Stream};
use ringbuf::{
    traits::*,
    HeapRb,
};

mod audiodriver;

#[derive(Default)]
struct DiscordDriver {
    is_connected: bool,
    can_send_audio: bool,
    server_ip: String,
    socket: Option<tokio::net::UdpSocket>,
    user_name: String,
}

#[tauri::command]
fn start_audio_loop(state: tauri::State<Arc<Mutex<DiscordDriver>>>) {
    println!("Running Audio Loop Checks");

    let driver_state = Arc::clone(&state);
    //Audio Loop
    tauri::async_runtime::spawn(async move {
        println!("Init Connecting To Server");

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

        println!("Sending Init Data to Server");
        //Send Initial Data Like Username
        let server_ip = driver.server_ip.clone();
        let username = driver.user_name.to_owned().clone();
        let full = "username:".to_string() + &username;
        let data = full.as_bytes();

        if let Some(socket) = driver.socket.as_ref() {
            
            let socket = Arc::new(Mutex::new(socket));

            println!("Prepairing Audio Loops");
            let loop_driver = Arc::clone(&driver_state);

            let mut audio_driver = audiodriver::AudioDriver::default();
            //let input_stream = audio_driver.start_audio_capture(socket.clone());

            audio_driver.audio_debugger();

            drop(driver);
            
            //Audio Recieve Loop
            tauri::async_runtime::spawn(async move {
                println!("[LIB] Sending / Receiving Audio Data");
                loop {
                    {
                        /*
                            This Function locks the AudioDriver constantly causing the other function
                            stop_audio_loop to not set can_send_audio to false;
                        */
                        //println!("Sending Audio Packet");
                        let can_send_audio = {
                            let driver = loop_driver.lock().await;
                            driver.can_send_audio
                        };
        
                        //================================================================
                        // Audio Sending
                        // If can_send_audio is false, break the loop
                        if can_send_audio {
                            let mut driver = loop_driver.lock().await;
                            driver.is_connected = false;
                            drop(driver);

                            println!("[LIB] Main Thread Stopping Audio Sending...");
                            audio_driver.stop_input_stream();
                            audio_driver.stop_output_stream();
                            drop(audio_driver);
                            break;
                        }
        
                        let mut driver = loop_driver.lock().await;
                        // Clone server_ip to avoid holding the lock during async calls
                        let server_ip = driver.server_ip.clone();
        
                        // Send audio data & recieve audio data
                        if let Some(socket) = driver.socket.as_ref() {
                            let mut buf = [0; 1024];
                            if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
                                println!("{:?} bytes received from {:?}", len, addr);
                                //Play Audio
                            }
                        } else {
                            eprintln!("Socket is not initialized. Cannot send data.");
                            break; // Exit the loop if the socket is not initialized
                        }
        
                        drop(driver);
                    }
                }
            });
        } else {
            eprintln!("Socket is not initialized. Cannot send data.");
            return; // Exit the loop if the socket is not initialized
        }
    });
}

#[tauri::command]
fn stop_audio_loop(state: tauri::State<Arc<Mutex<DiscordDriver>>>) {
    println!("Main Thread Stopping Audio Sending...");

    let driver_state = Arc::clone(&state);
    tauri::async_runtime::spawn(async move {
        let mut driver = driver_state.lock().await;
        driver.can_send_audio = false;
        drop(driver);
        drop(driver_state);
    });
}

#[tauri::command]
fn set_server_ip(state: tauri::State<Arc<Mutex<DiscordDriver>>>, server_ip: String) {
    let driver_state = Arc::clone(&state);
    tauri::async_runtime::spawn(async move {
        let mut driver = driver_state.lock().await;
        driver.server_ip = server_ip;
        drop(driver);
        drop(driver_state);
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let driver = Arc::new(Mutex::new(DiscordDriver {
        is_connected: false,
        can_send_audio: false, // Set to true for testing
        server_ip: "127.0.0.1:3000".to_string(),
        socket: None,
        user_name: "Username Not Set".to_string(),
    }));
    
    tauri::Builder::default()
        .manage(driver)
        .invoke_handler(tauri::generate_handler![stop_audio_loop,start_audio_loop,set_server_ip])
        //.invoke_handler(tauri::generate_handler![stop_audio_loop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}