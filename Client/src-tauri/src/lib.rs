use tauri::{AppHandle, Builder};
use tokio::sync::{watch, Mutex};
use std::sync::Arc;
use std::io;
use tokio::time::{sleep, Duration};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{InputCallbackInfo, OutputCallbackInfo, StreamConfig, Stream};
use audiopus::{coder::Decoder, Application, Channels, SampleRate, Bandwidth};
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
    user_name: String,
}

#[tauri::command]
fn start_audio_loop(state: tauri::State<Arc<Mutex<DiscordDriver>>>) {
    println!("[LIB] Trying Connecting to Server");

    let driver_state = Arc::clone(&state);
    //Audio Loop
    tauri::async_runtime::spawn(async move {
        println!("[LIB] Init Connecting To Server");

        let mut driver = driver_state.lock().await;
        if driver.can_send_audio { 
            eprintln!("[LIB] Cannot Send Audio, Another Thread is Already Sending Audio");
            return;
        }

        // Create a UDP socket
        let new_socket = match tokio::net::UdpSocket::bind("0.0.0.0:0").await {
            Ok(new_socket) => new_socket,
            Err(e) => {
                eprintln!("[LIB] Failed to bind UDP socket: {}", e);
                return;
            }
        };


        let socket = Arc::new(new_socket);

        driver.can_send_audio = true;

        println!("[LIB] Sending User Data to Server");
        //Send Initial Data Like Username
        let server_ip = Arc::new(driver.server_ip.clone());
        let username = driver.user_name.to_owned().clone();
        let full = "username:".to_string() + &username;
        let data = full.as_bytes();
        
        //Sending The User Data
        let temp_socket = Arc::clone(&socket);
        let temp_ip = Arc::clone(&server_ip);
        if let Err(e) = temp_socket.send_to(data, &*temp_ip).await {
            eprintln!("[LIB] Failed to send data: {}", e);
            return; // Exit the loop if sending fails
        }
        else { println!("[LIB] Sent User Data to Server") }
        drop(temp_socket);

        println!("[LIB] Prepairing Audio Loops");
        let loop_driver1 = Arc::clone(&driver_state);
        let loop_driver2 = Arc::clone(&driver_state);

        let mut audio_driver = Arc::new(Mutex::new(audiodriver::AudioDriver::default()));
        //let input_stream = audio_driver.start_audio_capture(socket.clone());

        let audio_sender_socket = Arc::clone(&socket);
        let audio_sender_ip = Arc::clone(&server_ip);

        let mut audio_driver_locked = audio_driver.lock().await;
        audio_driver_locked.start_audio_capture(audio_sender_socket,audio_sender_ip);
        audio_driver_locked.start_audio_player();
        drop(audio_driver_locked);

        drop(driver);

        let (sender, reciever) = watch::channel(());

        //Server Checks
        let disconnect_socket = Arc::clone(&socket);
        let disconnect_ip = Arc::clone(&server_ip);
        let disconnect_audio_driver = Arc::clone(&audio_driver);
        tauri::async_runtime::spawn(async move {
            println!("[LIB] Sending / Receiving Audio Data");
            loop {
                {
    
                    let mut driver = loop_driver1.lock().await;
                    if !driver.can_send_audio {
                        driver.is_connected = false;

                        sender.send(()).unwrap();

                        println!("[LIB] Disconnecting from Server");

                        println!("[LIB] Dropping Audio Input Stream / Output Stream");
                        let mut audio_driver_temp = disconnect_audio_driver.lock().await;
                        audio_driver_temp.stop_input_stream();
                        audio_driver_temp.stop_output_stream();
                        drop(audio_driver_temp);

                        //Send Server Disconnect Information
                        if let Err(e) = disconnect_socket.send_to("disconnect".to_string().as_bytes(), &*disconnect_ip).await {
                            eprintln!("[LIB] Failed to send data: {}\nClosed heartbeat sub-task", e);
                            return; // Exit the loop if sending fails
                        }
                        else {
                            println!("[LIB] Sent Heartbeat...");
                        }

                        drop(driver);

                        break;
                    }

                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        });
        
        //Audio Recieve Loop
        let mut audio_rx = reciever.clone();
        let data_recieve_socket = Arc::clone(&socket);
        let recieve_audio_driver = Arc::clone(&audio_driver);
        tauri::async_runtime::spawn(async move {
            println!("[LIB] Receiving Audio Data");
            tokio::select! {
                _ = async {
                    let mut temp_decoder = Decoder::new(SampleRate::Hz48000, Channels::Stereo);
            
                    match temp_decoder {
                        Ok(mut decoder) => {
                            loop {
                                let mut buf = [0; 2048];
                                let mut pcm_audio = vec![0.0; 1920]; //1920 = 48khz * 0.02 * 2

                                if let Ok((len, addr)) = data_recieve_socket.recv_from(&mut buf).await {
                                    println!("[LIB] {:?} bytes received from {:?}", len, addr);
                                    // Opus Decode -> Play Audio

                                    //Need data in type of u8
                                    let vec_buf: Vec<u8> = Vec::from(&buf[..len]);

                                    // Decode the received Opus data into PCM samples
                                    match decoder.decode_float(Some(&vec_buf), &mut pcm_audio, false) {
                                        Ok(decoded_len) => {
                                            println!("[AUDIO DRIVER/LIB] Decoded {} samples of audio.", decoded_len);

                                            //Play the recieved audio instantly
                                            let mut audio_driver_temp = recieve_audio_driver.lock().await;
                                            audio_driver_temp.play_audio(&pcm_audio);
                                            drop(audio_driver_temp);
                                        }
                                        Err(e) => {
                                            eprintln!("[AUDIO DRIVER/LIB] Failed to decode Opus data: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[AUDIO DRIVER/LIB] Failed to create encoder: {}", e);
                        }
                    }
                } => {}
                _ = audio_rx.changed() => {
                    println!("[LIB] Audio Loop (RECIEVE) Terminated");
                }
            }
        });

        //Heartbeat - Send Data Every ~ 1 Sec to not be disconnected
        let mut heartbeat_rx = reciever.clone();
        let heartbeat_socket = Arc::clone(&socket);
        let heartbeat_ip = Arc::clone(&server_ip);
        tauri::async_runtime::spawn(async move {
            println!("[LIB] Starting Heartbeat Sub-Task");
            tokio::select! {
                _ = async {
                    loop {
                        //Wait 1 second
                        sleep(Duration::from_secs(1)).await;

                        //Send Nothing, Just Let Serevr know i'm Alive
                        if let Err(e) = heartbeat_socket.send_to("hb".to_string().as_bytes(), &*heartbeat_ip).await {
                            eprintln!("[LIB] Failed to send data: {}\nClosed heartbeat sub-task", e);
                            return; // Exit the loop if sending fails
                        }
                    }
                } => {}
                _ = heartbeat_rx.changed() => {
                    println!("[LIB] Audio Loop (RECIEVE) Terminated");
                }
            }
        });
    });
}

#[tauri::command]
fn stop_audio_loop(state: tauri::State<Arc<Mutex<DiscordDriver>>>) {
    println!("\n[LIB] Trying to Disconnect from Server");

    let driver_state = Arc::clone(&state);
    tauri::async_runtime::spawn(async move {
        println!("[LIB] Side-Thread waiting for State to be cleared.");
        let mut driver = driver_state.lock().await;
        driver.can_send_audio = false;
        drop(driver);
        drop(driver_state);
        println!("[LIB] Completed Server Disconnection");
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
        user_name: "Username Not Set".to_string(),
    }));
    
    tauri::Builder::default()
        .manage(driver)
        .invoke_handler(tauri::generate_handler![stop_audio_loop,start_audio_loop,set_server_ip])
        //.invoke_handler(tauri::generate_handler![stop_audio_loop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}