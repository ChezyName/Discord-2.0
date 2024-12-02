use tauri::{AppHandle, Builder};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{InputCallbackInfo, OutputCallbackInfo, Sample, SampleFormat, StreamConfig};
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};

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
            if let Err(e) = socket.send_to(data, &server_ip).await {
                eprintln!("Failed to send data: {}", e);
                return; // Exit the loop if sending fails
            }
        } else {
            eprintln!("Socket is not initialized. Cannot send data.");
            return; // Exit the loop if the socket is not initialized
        }

        drop(driver);

        println!("Prepairing Audio Loops");
        let loop_driver_1 = Arc::clone(&driver_state);
        let loop_driver_2 = Arc::clone(&driver_state);
        
        //Audio Recieve Loop
        tauri::async_runtime::spawn(async move {
            loop {
                {
                    //println!("Sending Audio Packet");
                    let mut driver = loop_driver_1.lock().await;
    
                    //================================================================
                    // Audio Sending
                    // If can_send_audio is false, break the loop
                    if !driver.can_send_audio {
                        driver.is_connected = false;
                        break;
                    }
    
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

        //Audio Send Loop
        tauri::async_runtime::spawn(async move {
            loop {
                {
                    //println!("Sending Audio Packet");
                    let mut driver = loop_driver_2.lock().await;
    
                    //================================================================
                    // Audio Sending
                    // If can_send_audio is false, break the loop
                    if !driver.can_send_audio {
                        println!("Stopping audio loop, can_send_audio is false");
                        driver.is_connected = false;
    
                        //Send Goodbye Message
                        let server_ip = driver.server_ip.clone();
                        let data = "DISCONNECT".as_bytes();

                        // Send audio data & recieve audio data
                        if let Some(socket) = driver.socket.as_ref() {
                            if let Err(e) = socket.send_to(data, &server_ip).await {
                                eprintln!("Failed to send data: {}", e);
                                break; // Exit the loop if sending fails
                            }
                        } else {
                            eprintln!("Socket is not initialized. Cannot send data.");
                            break; // Exit the loop if the socket is not initialized
                        }
                        
                        drop(driver);
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
                    } else {
                        eprintln!("Socket is not initialized. Cannot send data.");
                        break; // Exit the loop if the socket is not initialized
                    }
    
                    drop(driver);
                }
            }
        });
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

#[tauri::command]
fn set_server_ip(state: tauri::State<Arc<Mutex<AudioDriver>>>, server_ip: String) {
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
    let driver = Arc::new(Mutex::new(AudioDriver {
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

pub async fn execute_audio_debug() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();

    // Get default input and output devices
    let input_device = host.default_input_device().expect("Failed to find input device");
    let output_device = host.default_output_device().expect("Failed to find output device");

    // Get input and output configurations
    let input_config: StreamConfig = input_device.default_input_config()?.into();
    let output_config: StreamConfig = output_device.default_output_config()?.into();

    println!("Using input device: \"{}\"", input_device.name()?);
    println!("Using output device: \"{}\"", output_device.name()?);

    // The buffer to share samples
    //150ms delay
    let latency_frames = ((1000.0) / 1_000.0) * input_config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * input_config.channels as usize; 
    let ring = HeapRb::<f32>::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.try_push(0.0).unwrap();
    }

    // Shared buffer between input and output
    let buffer = Arc::new(Mutex::new(Vec::<f32>::new()));

    // Create the input stream
    let buffer_clone = Arc::clone(&buffer);
    let input_stream = input_device.build_input_stream(
        &input_config,
        move |data: &[f32], _: &InputCallbackInfo| {
            let mut output_fell_behind = false;
            for &sample in data {
                if producer.try_push(sample).is_err() {
                    output_fell_behind = true;
                }
            }
            if output_fell_behind {
                eprintln!("output stream fell behind: try increasing latency");
            }
        },
        |err| {
            eprintln!("Error in input stream: {}", err);
        },
        None, // Provide an optional latency duration
    )?;

    // Create the output stream
    let buffer_clone = Arc::clone(&buffer);
    let output_stream = output_device.build_output_stream(
        &output_config,
        move |data: &mut [f32], _: &OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match consumer.try_pop() {
                    Some(s) => s,
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }
        },
        |err| {
            eprintln!("Error in output stream: {}", err);
        },
        None, // Provide an optional latency duration
    )?;

    // Start the streams
    input_stream.play()?;
    output_stream.play()?;

    // Keep the application running
    println!("Playing for 5 seconds... ");
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Ended Audio Loopback ");
    
    drop(input_stream);
    drop(output_stream);

    Ok(())
}