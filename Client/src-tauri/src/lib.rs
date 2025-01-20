use audiopus::{coder::Encoder, coder::Decoder, Application, Bandwidth, Channels, SampleRate};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{InputCallbackInfo, OutputCallbackInfo, Stream, StreamConfig};
use ringbuf::{traits::*, HeapRb};
use std::io;
use std::sync::Arc;
use tauri::{AppHandle, Builder, Emitter};
use tokio::sync::{watch, Mutex};
use tokio::time::{sleep, Duration};
use std::sync::atomic::{AtomicBool, Ordering};
use samplerate::{convert, ConverterType};

mod audiodriver;

#[derive(Default)]
struct DiscordDriver {
    is_connected: bool,
    can_send_audio: bool,
    server_ip: String,
    user_name: String,
    is_audio_debug: bool, //If True, Cannot Send Audio, Audio is Being Tested
    audio_settings_changed: bool, //Create new stream as audio driver has been changed
}

#[tauri::command]
fn start_audio_loop(state: tauri::State<Arc<Mutex<DiscordDriver>>>) {
    println!("[LIB] Trying Connecting to Server");

    let driver_state = Arc::clone(&state);
    //Audio Thread
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

        println!("[LIB] Sent Username: {}, to Server: {}", &username, &driver.server_ip);

        //Sending The User Data
        let temp_socket = Arc::clone(&socket);
        let temp_ip = Arc::clone(&server_ip);
        if let Err(e) = temp_socket.send_to(data, &*temp_ip).await {
            eprintln!("[LIB] Failed to send data: {}", e);
            return; // Exit the loop if sending fails
        } else {
            println!("[LIB] Sent User Data to Server")
        }
        drop(temp_socket);

        println!("[LIB] Prepairing Audio Loops");
        let loop_driver1 = Arc::clone(&driver_state);
        let loop_driver2 = Arc::clone(&driver_state);

        let mut audio_driver = Arc::new(Mutex::new(audiodriver::AudioDriver::default()));
        //let input_stream = audio_driver.start_audio_capture(socket.clone());

        let audio_sender_socket_a = Arc::clone(&socket);
        let audio_sender_socket_b = Arc::clone(&socket);
        let audio_sender_ip = Arc::clone(&server_ip);

        let mut audio_driver_locked = audio_driver.lock().await;
        audio_driver_locked.start_audio_capture(audio_sender_socket_a, audio_sender_ip);
        audio_driver_locked.start_audio_playback(audio_sender_socket_b);
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
                        if let Err(e) = disconnect_socket
                            .send_to("disconnect".to_string().as_bytes(), &*disconnect_ip)
                            .await
                        {
                            eprintln!(
                                "[LIB] Failed to send data: {}\nClosed heartbeat sub-task",
                                e
                            );
                            return; // Exit the loop if sending fails
                        } else {
                            println!("[LIB] Sent Heartbeat...");
                        }

                        drop(driver);

                        break;
                    }

                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        });

        //Audio Settings Changed Loop
        let mut settings_rx = reciever.clone();
        let settings_audio_driver = Arc::clone(&audio_driver);
        let settings_driver = Arc::clone(&driver_state);
        let settings_audio_sender_socket = Arc::clone(&socket);
        let settings_audio_sender_ip = Arc::clone(&server_ip);
        tauri::async_runtime::spawn(async move {
            println!("[LIB] Starting Audio Hardware Settings Change Event Watcher Subtask (AHSCEW)");
            tokio::select! {
                _ = async {
                    loop {
                        let mut driver = settings_driver.lock().await;

                        //Wait for 'audio_settings_changed' to change to swap audio hardware
                        if driver.audio_settings_changed {
                            println!("[LIB] Audio Hardware Changed, Applying Settings NOW!");
                            let mut audio_driver_temp = settings_audio_driver.lock().await;

                            let clone_socket_a = Arc::clone(&settings_audio_sender_socket);
                            let clone_socket_b = Arc::clone(&settings_audio_sender_socket);
                            let clone_ip = Arc::clone(&settings_audio_sender_ip);

                            //Restart Audio Stream
                            audio_driver_temp.stop_input_stream();
                            audio_driver_temp.stop_output_stream();

                            audio_driver_temp.start_audio_capture(clone_socket_a, clone_ip);
                            audio_driver_temp.start_audio_playback(clone_socket_b);

                            drop(audio_driver_temp);

                            driver.audio_settings_changed = false;
                        }

                        //Drop to allow other threads to use Driver
                        drop(driver);

                        //Sleep for ~15ms to allow other threads to use driver
                        sleep(Duration::from_millis(15)).await;
                    }
                } => {}
                _ = settings_rx.changed() => {
                    println!("[LIB] Audio Loop (RECIEVE) Terminated");
                }
            }
        });

        //Audio Recieve Loop
        /*
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
                                            //println!("[AUDIO DRIVER/LIB] Decoded {} samples of audio.", decoded_len);

                                            //Play the recieved audio instantly
                                            let mut audio_driver_temp = recieve_audio_driver.lock().await;
                                            //audio_driver_temp.play_audio(&pcm_audio);
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
        */

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
    println!("[LIB/SETTINGS] Waiting to Set IP");
    tauri::async_runtime::spawn(async move {
        let mut driver = driver_state.lock().await;
        println!("[LIB/SETTINGS] Setting Server IP as {}", &server_ip);
        driver.server_ip = server_ip;
        drop(driver);
        drop(driver_state);
    });
}

#[tauri::command]
fn get_input_devices(state: tauri::State<Arc<Mutex<DiscordDriver>>>) -> Vec<String> {
    return audiodriver::AudioDriver::get_input_devices();
}

#[tauri::command]
fn get_output_devices(state: tauri::State<Arc<Mutex<DiscordDriver>>>) -> Vec<String> {
    return audiodriver::AudioDriver::get_output_devices();
}

#[tauri::command]
fn get_current_devices(state: tauri::State<Arc<Mutex<DiscordDriver>>>) -> Vec<String> {
    return audiodriver::AudioDriver::get_current_audio_devices();
}

//When Changing Inputs, Disconnect from Sever, Change Input, Reconnect
#[tauri::command]
fn change_current_input_device(state: tauri::State<Arc<Mutex<DiscordDriver>>>, input_device: String) {
    let device = audiodriver::AudioDriver::get_current_audio_devices();
    if device.len() > 1 {
        let output_device = &device[1];
        audiodriver::AudioDriver::writeAudioConfig(&input_device, &output_device);
        println!("[AUDIO DRIVER/CID] Changed Input Device to: {}", &input_device)
    }
    else {
        println!("[AUDIO DRIVER/CID] Failed to Read Device List (in Audio File)")
    }
}

#[tauri::command]
fn change_current_output_device(state: tauri::State<Arc<Mutex<DiscordDriver>>>, output_device: String) {
    let device = audiodriver::AudioDriver::get_current_audio_devices();
    if device.len() > 1 {
        let input_device = &device[0];
        audiodriver::AudioDriver::writeAudioConfig(&input_device, &output_device);
        println!("[AUDIO DRIVER/COD] Changed Output Device to: {}", &output_device)
    } else {
        println!("[AUDIO DRIVER/COD] Failed to Read Device List (in Audio File)")
    }
}

#[tauri::command]
fn on_audio_settings_changed(state: tauri::State<Arc<Mutex<DiscordDriver>>>) {
    let driver_state = Arc::clone(&state);
    tauri::async_runtime::spawn(async move {
        let mut driver = driver_state.lock().await;
        driver.audio_settings_changed = true;
        println!("[AUDIO DRIVER/SETTINGS] Settings Changed for Audio Driver, Applying Changes Shortly.");
        drop(driver);
        drop(driver_state);
    });
}

#[tauri::command]
fn set_username(state: tauri::State<Arc<Mutex<DiscordDriver>>>, username: String) {
    let driver_state = Arc::clone(&state);
    tauri::async_runtime::spawn(async move {
        let mut driver = driver_state.lock().await;
        println!("[LIB/SETTINGS] Username set as {}", &username);
        driver.user_name = username;
        drop(driver);
        drop(driver_state);
    });
}

#[tauri::command]
fn start_audio_test(app: AppHandle, state: tauri::State<Arc<Mutex<DiscordDriver>>>) {
    let driver_state = Arc::clone(&state);
    let driver_state_loop = Arc::clone(&state);
    
    //Audio Thread for Audio Debugging
    tauri::async_runtime::spawn(async move {
        let mut driver = driver_state.lock().await;
        driver.is_audio_debug = true;
        drop(driver);

        let host = cpal::default_host();

        println!("[AUDIO DRIVER/DEBUG] Started Audio Debugger");

        // Get the output device (based on the Users Input)
        let devices = audiodriver::AudioDriver::get_current_audio_devices();
        let output_device_name = devices.get(1);
        let output_device = match output_device_name {
            Some(name) => audiodriver::AudioDriver::get_output_device_by_name(name),
            None => None,
        };
    
        // If output_device is still None, fall back to the host's default output device.
        let output_device = match output_device {
            Some(device) => device,
            None => match host.default_output_device() {
                Some(device) => device,
                None => {
                    eprintln!("[AUDIO DRIVER/DEBUG] No Output device available.");
                    return;
                }
            },
        };

        // Get output config [ERROR at LINE 538 (LINE BELOW v)]
        let output_config: StreamConfig = match output_device.default_output_config() {
            Ok(config) => config.into(),
            Err(err) => {
                eprintln!("[AUDIO DRIVER/DEBUG] Failed to get default output configuration: {:?}", err);
                return;
            }
        };

        //====================================================================
        //INPUT DEVICES
        let idevices = audiodriver::AudioDriver::get_current_audio_devices();
        let input_device_name = idevices.get(0);
        let input_device = match input_device_name {
            Some(name) => audiodriver::AudioDriver::get_input_device_by_name(name),
            None => None,
        };
    
        // If input_device is still None, fall back to the host's default input device.
        let input_device = match input_device {
            Some(device) => device,
            None => match host.default_input_device() {
                Some(device) => device,
                None => {
                    eprintln!("[AUDIO DRIVER/DEBUG] No input device available.");
                    return;
                }
            },
        };

        // Get input config
        let input_config: StreamConfig = input_device.default_input_config().unwrap().into();

        let output_sample_rate = output_config.sample_rate.0;
        let input_sample_rate = input_config.sample_rate.0;
        let output_channels = output_config.channels;

        let ring_a = HeapRb::<f32>::new(input_sample_rate as usize * 2);
        let ring_b = HeapRb::<f32>::new(output_sample_rate as usize * 2);
        
        let (mut g_producer, mut g_consumer) = ring_a.split();
        let (mut producer, mut consumer) = ring_b.split();

        let stream_active = Arc::new(AtomicBool::new(true));

        let input_stream_active = stream_active.clone();
        tauri::async_runtime::spawn(async move {
            let frame_samples: usize = (input_sample_rate as f32 * 0.02 * 2.0) as usize;
            let mut pcm_buffer: Vec<f32> = vec![0.0; frame_samples];
            let mut compressed_data: Vec<u8> = vec![0; frame_samples];

            let input_stream = input_device
            .build_input_stream(
                &input_config,
                move |data: &[f32], _: &InputCallbackInfo| {
                    let mut buffer = Vec::new();
                    let mut buffer_playback = Vec::new();

                    // Handle mono or stereo input
                    let num_channels = input_config.channels as usize;

                    for &sample in data {
                        if num_channels == 1 { buffer.push(sample); buffer_playback.push(sample) }
                        buffer.push(sample); // Right channel
                        buffer_playback.push(sample)
                    }

                    // Resample if necessary
                    if input_sample_rate != 48000 {
                        if !buffer.is_empty() {
                            buffer = convert(
                                input_sample_rate,
                                48000,
                                2, // Dual-channel audio
                                ConverterType::Linear,
                                &buffer,
                            )
                            .expect("Resampling failed");
                        }
                    }

                    if input_sample_rate != output_sample_rate {
                        if !buffer_playback.is_empty() {
                            buffer_playback = convert(
                                input_sample_rate,
                                output_sample_rate,
                                2, // Dual-channel audio
                                ConverterType::Linear,
                                &buffer,
                            )
                            .expect("Resampling failed");
                        }
                    }

                    // Push samples to the ring buffer
                    for &sample in buffer.iter() {
                        producer.try_push(sample).unwrap_or_else(|_| {
                            // Drop the oldest sample if the buffer is full
                            consumer.try_pop();
                        });
                    }

                    for &sample in buffer_playback.iter() {
                        g_producer.try_push(sample);
                    }

                    // TODO: Send audio via UDP socket every 20ms
                    // Example: Encode with Opus @ 48kHz dual channel
                    if consumer.iter().count() >= frame_samples {
                        //println!("[AUDIO DRIVER] Prepairing to Send 20ms of Data to Server");
                        //println!("[AUDIO DRIVER]    or {} samples", frame_samples);

                        // println!("[AUDIO DRIVER/LIB] Sending Audio Data to Server");

                        // Fill the PCM buffer with 20ms worth of samples
                        pcm_buffer.clear();
                        for _ in 0..frame_samples {
                            if let Some(sample) = consumer.try_pop() {
                                pcm_buffer.push(sample as f32); // Convert f32 to i16
                            }
                        }

                        if let Err(err) = app.emit("audio-sample", pcm_buffer.as_slice()) {
                            eprintln!("Failed to Emit Audio Debug Data: {:?}", err);
                        }
                    }
                },
                |err| eprintln!("[AUDIO DRIVER/DEBUG] Input stream error: {}", err),
                None,
            )
            .expect("[AUDIO DRIVER/DEBUG] Failed to create input stream.");

            input_stream.play().expect("[AUDIO DRIVER/DEBUG] Failed to start input stream");

                    
            while input_stream_active.load(Ordering::SeqCst)
            {
                // Allow other threads to work
                std::thread::sleep(std::time::Duration::from_millis(5));
            }

            drop(input_stream);
            println!("[AUDIO DRIVER/DEBUG] Stopping Audio Debug INPUT Thread Stopped.");
        });

        let output_stream_active = stream_active.clone();
        tauri::async_runtime::spawn(async move {
            let output_stream = output_device.build_output_stream(&output_config, move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for (i, sample) in data.iter_mut().enumerate() {
                        // Attempt to pop a sample from the ring buffer
                        match g_consumer.try_pop() {
                            Some(value) => {
                                *sample = value; // Use the sample from the buffer

                                //Send to Front-end
                            },
                            None => {
                                *sample = 0.0; // Fill with silence if the buffer is empty
                            }
                        }
                    }
                }, 
            |err| eprintln!("[AUDIO DRIVER/DEBUG] Error in output stream: {}", err), 
            None
            ).expect("[AUDIO DRIVER/DEBUG] Failed to build output stream");
            
            output_stream.play().expect("[AUDIO DRIVER/DEBUG] Failed to start output stream");

                    
            while output_stream_active.load(Ordering::SeqCst)
            {
                // Allow other threads to work
                std::thread::sleep(std::time::Duration::from_millis(5));
            }

            drop(output_stream);
            println!("[AUDIO DRIVER/DEBUG] Stopping Audio Debug OUTPUT Thread Stopped.");
        });

        let stream_stopper = stream_active.clone();
        tauri::async_runtime::spawn(async move {
            loop {
                let mut driver = driver_state_loop.lock().await;
                if !driver.is_audio_debug {
                    stream_active.store(false, Ordering::SeqCst);
                    println!("[AUDIO DRIVER/DEBUG] Stopping Audio Debug Threads Shortly...");
                    break;
                }

                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        });
    });
}

#[tauri::command]
fn stop_audio_test(state: tauri::State<Arc<Mutex<DiscordDriver>>>) {
    println!("[AUDIO DRIVER/DEBUG] Stopping Audio Debug");

    let driver_state = Arc::clone(&state);
    tauri::async_runtime::spawn(async move {
        let mut driver = driver_state.lock().await;
        driver.is_audio_debug = false;
        drop(driver);
        drop(driver_state);
        println!("[AUDIO DRIVER/DEBUG] Stopped Audio Debug");
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    //Init Driver
    let driver = Arc::new(Mutex::new(DiscordDriver {
        is_connected: false,
        can_send_audio: false, // Set to true for testing
        server_ip: "127.0.0.1:3000".to_string(),
        user_name: "Username Not Set".to_string(),
        is_audio_debug: false, //if true, stops the sending of audio and instead 'tests' by pipe'ing audio into headphones
        audio_settings_changed: false,
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(driver)
        .invoke_handler(tauri::generate_handler![
            stop_audio_loop,
            start_audio_loop,
            set_server_ip,
            get_input_devices,
            get_output_devices,
            get_current_devices,
            change_current_input_device,
            change_current_output_device,
            on_audio_settings_changed,
            set_username,
            start_audio_test,
            stop_audio_test
        ])
        .setup(|app| {
            audiodriver::AudioDriver::initFiles();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
