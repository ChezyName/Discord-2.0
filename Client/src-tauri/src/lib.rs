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
    use std::time::Duration;
    use cpal::{
        traits::{DeviceTrait, HostTrait, StreamTrait},
        StreamConfig,
    };
    use rtrb::HeapRb;

    let host = cpal::default_host();

    // Get default input and output devices
    let input_device = host.default_input_device().expect("Failed to find input device");
    let output_device = host.default_output_device().expect("Failed to find output device");

    // Get input and output configurations
    let input_config: StreamConfig = input_device.default_input_config()?.into();
    let output_config: StreamConfig = output_device.default_output_config()?.into();

    println!("Using input device: \"{}\"", input_device.name()?);
    println!("Using output device: \"{}\"", output_device.name()?);

    // Calculate the number of samples to keep for 1 second
    let sample_rate = input_config.sample_rate.0 as usize;
    let channels = input_config.channels as usize;
    let samples_per_second = sample_rate * channels;

    // Create a ring buffer to hold 1 second of audio
    let ring = HeapRb::<f32>::new(samples_per_second);
    let (mut producer, mut consumer) = ring.split();

    // Create the input stream
    let input_stream = input_device.build_input_stream(
        &input_config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            for &sample in data {
                if producer.is_full() {
                    // Overwrite the oldest data
                    producer.pop();
                }
                producer.push(sample).expect("Failed to write to producer");
            }
        },
        |err| {
            eprintln!("Error in input stream: {}", err);
        },
        None, // Optional latency duration
    )?;

    // Create the output stream
    let output_stream = output_device.build_output_stream(
        &output_config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = consumer.pop().unwrap_or(0.0); // Output silence if buffer is empty
            }
        },
        |err| {
            eprintln!("Error in output stream: {}", err);
        },
        None, // Optional latency duration
    )?;

    // Start the streams
    input_stream.play()?;
    output_stream.play()?;

    // Keep the application running
    println!("Streaming audio for 10 seconds...");
    std::thread::sleep(Duration::from_secs(10));
    println!("Audio streaming ended.");

    // Drop streams to stop audio processing
    drop(input_stream);
    drop(output_stream);

    Ok(())
}
