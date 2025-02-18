/**
 * Flawwed Audio Driver That Holds Audio Functions For Getting and Recieving Audio,
 * Audio is created and destroyed frequently and not held in a global state
 * 
 * 
 * To who ever is reading this and thinks they can deal with this file, I wish you the best of luck.
 * TO DO = Improve File By Splitting
 *  - Audio Input Sub File
 *  - Audio Output Sub File
 *  - Audio Debugger Sub File
 */

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

use audiopus::{coder::Encoder, coder::Decoder, Application, Bandwidth, Channels, SampleRate};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Device, Host, InputCallbackInfo, OutputCallbackInfo, Stream, StreamConfig,
};
use cpal::{SampleFormat, SupportedBufferSize};
use ringbuf::{storage::Heap, traits::*, wrap::caching::Caching, HeapRb, SharedRb};
use samplerate::{convert, ConverterType};
use std::error::Error;
use std::string::String;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{watch, Mutex};
use std::path::{Path, PathBuf};
use dirs;
use serde_json::{json, Value};
use std::fs::{File, write};
use std::io::{self, BufReader};
use std::fs;

pub fn run_audio_debugger() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the default audio host
    let host = cpal::default_host();

    // Get default input and output devices
    let input_device = host
        .default_input_device()
        .expect("Failed to find input device");
    let output_device = host
        .default_output_device()
        .expect("Failed to find output device");

    // Get input and output configurations
    let input_config: StreamConfig = input_device.default_input_config()?.into();
    let output_config: StreamConfig = output_device.default_output_config()?.into();

    println!(
        "[AUDIO DRIVER] Using input device: \"{}\"",
        input_device.name()?
    );
    println!(
        "[AUDIO DRIVER] Using output device: \"{}\"",
        output_device.name()?
    );
    println!(
        "[AUDIO DRIVER] Using input config: sample rate: {}, channels: {}",
        input_config.sample_rate.0, input_config.channels
    );
    println!(
        "[AUDIO DRIVER] Using output config: sample rate: {}, channels: {}",
        output_config.sample_rate.0, output_config.channels
    );

    // Create a buffer for raw audio
    let buffer_capacity = input_config.sample_rate.0 as usize; // Buffer for 1 second of audio over two channels
    let ring = HeapRb::<f32>::new(buffer_capacity * 2);
    let (mut producer, mut consumer) = ring.split();

    // Input stream
    let input_stream = input_device.build_input_stream(
        &input_config,
        move |data: &[f32], _: &InputCallbackInfo| {
            // Push audio samples to the producer
            for &sample in data {
                if !producer.is_full() {
                    //turn mono to 2 channel
                    if (input_config.channels == 1) {
                        producer.try_push(sample).unwrap();
                    }
                    producer.try_push(sample).unwrap();
                }
            }
        },
        |err| eprintln!("[AUDIO DRIVER] Error in input stream: {}", err),
        None,
    )?;

    // Output stream
    let output_stream = output_device.build_output_stream(
        &output_config,
        move |output_data: &mut [f32], _: &OutputCallbackInfo| {
            let input_sample_rate = input_config.sample_rate.0 as u32;
            let output_sample_rate = output_config.sample_rate.0 as u32;
            let mut buffer = Vec::new();

            // Read samples from the consumer
            while let Some(sample) = consumer.try_pop() {
                buffer.push(sample);
            }

            // Resample if necessary
            if input_sample_rate != output_sample_rate {
                if !buffer.is_empty() {
                    buffer = convert(
                        input_sample_rate,
                        output_sample_rate,
                        1, // Assuming mono audio
                        ConverterType::Linear,
                        &buffer,
                    )
                    .expect("Resampling failed");
                }
            }

            // Fill the output buffer
            for (i, sample) in output_data.iter_mut().enumerate() {
                *sample = buffer.get(i).cloned().unwrap_or(0.0);
            }
        },
        |err| eprintln!("[AUDIO DRIVER] Error in output stream: {}", err),
        None,
    )?;

    // Start streams
    input_stream.play()?;
    output_stream.play()?;

    println!("[AUDIO DRIVER] Audio loopback started. Press Enter to stop...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}

//================================================================
//================================================================

//Use this to connect with both the input and output
pub struct AudioDriver {
    input_stream: Arc<AtomicBool>,
    output_stream: Arc<AtomicBool>,
}

impl Default for AudioDriver {
    fn default() -> Self {
        Self {
            input_stream: Arc::new(AtomicBool::new(false)),
            output_stream: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl AudioDriver {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            input_stream: Arc::new(AtomicBool::new(false)),
            output_stream: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn get_output_devices() -> Vec<String>{
        // Iterate through available devices from the host
        let mut devices: Vec<String> = Vec::new();
        let host = cpal::default_host();
        for device in host.output_devices().unwrap() {
            // Check if the device supports output
            if let Ok(configs) = device.supported_input_configs() {
                /*
                println!(
                    "[AUDIO DRIVER] Input Device: {}",
                    device.name().unwrap_or("Unknown Device".to_string())
                );
                */

                if let Ok(device_name) = device.name() {
                    devices.push(device_name)
                }

                // Optional: Print supported configurations
                for config in configs {
                    /*
                    println!(
                        "[AUDIO DRIVER]   Channels: {}, Sample Rates: {}-{}, Sample Format: {:?}",
                        config.channels(),
                        config.min_sample_rate().0,
                        config.max_sample_rate().0,
                        config.sample_format()
                    );
                    */
                }
            }
        }

        return devices
    }

    pub fn get_input_devices() -> Vec<String>{
        // Iterate through available devices from the host
        let mut devices: Vec<String> = Vec::new();
        let host = cpal::default_host();
        for device in host.input_devices().unwrap() {
            // Check if the device supports output
            if let Ok(configs) = device.supported_input_configs() {
                /*
                println!(
                    "[AUDIO DRIVER] Input Device: {}",
                    device.name().unwrap_or("Unknown Device".to_string())
                );
                 */

                if let Ok(device_name) = device.name() {
                    devices.push(device_name)
                }

                // Optional: Print supported configurations
                for config in configs {
                    /*
                    println!(
                        "[AUDIO DRIVER]   Channels: {}, Sample Rates: {}-{}, Sample Format: {:?}",
                        config.channels(),
                        config.min_sample_rate().0,
                        config.max_sample_rate().0,
                        config.sample_format()
                    );
                     */
                }
            }
        }

        return devices
    }

    pub fn get_output_device_by_name(target: &str) -> Option<Device> {
        let host = cpal::default_host();
        let mut found_device = host.default_input_device();

        match host.output_devices() {
            Ok(devices) => {
                for device in devices {
                    // Get the device name
                    if let Ok(name) = device.name() {
                        if name == target {
                            // Check if the device supports input
                            if device.default_output_config().is_ok() {
                                found_device = Some(device);
                                println!("[AUDIO DRIVER] Found output device: {}", name);
                                break;
                            }
                        }
                    }
                }

                if found_device.is_none() {
                    println!(
                        "[AUDIO DRIVER] Input device with name '{}' not found.",
                        target
                    );
                }
            }
            Err(err) => eprintln!("[AUDIO DRIVER] Failed to enumerate devices: {}", err),
        }

        return found_device;
    }

    pub fn get_input_device_by_name(target: &str) -> Option<Device> {
        let host = cpal::default_host();
        let mut found_device = host.default_input_device();

        match host.input_devices() {
            Ok(devices) => {
                for device in devices {
                    // Get the device name
                    if let Ok(name) = device.name() {
                        if name == target {
                            // Check if the device supports input
                            if device.default_input_config().is_ok() {
                                found_device = Some(device);
                                println!("[AUDIO DRIVER] Found input device: {}", name);
                                break;
                            }
                        }
                    }
                }

                if found_device.is_none() {
                    println!(
                        "[AUDIO DRIVER] Input device with name '{}' not found.",
                        target
                    );
                }
            }
            Err(err) => eprintln!("[AUDIO DRIVER] Failed to enumerate devices: {}", err),
        }

        return found_device;
    }

    pub fn start_audio_capture(
        &mut self,
        socket: Arc<tokio::net::UdpSocket>,
        server_ip: Arc<String>,
    ) {
        let input_stream_active = self.input_stream.clone();

        tauri::async_runtime::spawn(async move {
            let host = cpal::default_host();

            println!("[AUDIO DRIVER] Started Audio Recording");

            // Get the input device (based on the Users Input)
            let devices = AudioDriver::get_current_audio_devices();
            let input_device_name = devices.get(0);
            let input_device = match input_device_name {
                Some(name) => AudioDriver::get_input_device_by_name(name),
                None => None,
            };
        
            // If input_device is still None, fall back to the host's default input device.
            let input_device = match input_device {
                Some(device) => device,
                None => match host.default_input_device() {
                    Some(device) => device,
                    None => {
                        eprintln!("[AUDIO DRIVER] No input device available.");
                        return;
                    }
                },
            };

            // Get input config
            let input_config: StreamConfig = input_device.default_input_config().unwrap().into();

            // Extract input sample rate and format
            let input_sample_rate = input_config.sample_rate.0;
            let output_sample_rate = 48000; // Use 48kHz for output

            println!(
                "[AUDIO DRIVER] Selected Device Config:\n     Input Device {};\n     Channels: {};\n     Sample Rate: {};",
                input_device.name().unwrap_or("Unknown Device".to_string()),
                input_config.channels,
                input_config.sample_rate.0,
            );

            // Create a ring buffer for audio samples   {Force Dual Channel}
            let ring = HeapRb::<f32>::new(input_sample_rate as usize * 2);
            let (mut producer, mut consumer) = ring.split();

            let volumes = AudioDriver::get_current_audio_volumes();
            let input_volume: f32 = *volumes.get(0).unwrap_or(&100.0) / 100.0;

            //Encode the Audio with Opus
            let mut temp_encoder =
                Encoder::new(SampleRate::Hz48000, Channels::Stereo, Application::Voip);

            match temp_encoder {
                Ok(mut encoder) => {
                    // Set the bitrate and max bandwidth after successfully creating the encoder
                    encoder.set_bitrate(audiopus::Bitrate::BitsPerSecond(64_000));
                    encoder.set_max_bandwidth(Bandwidth::Fullband);

                    println!("[AUDIO DRIVER] Encoder Successfully Initialized");

                    //0.02 = 20ms, 2 = 2 channels
                    let frame_samples: usize = (output_sample_rate as f32 * 0.02 * 2.0) as usize;

                    //PCM Buffer to convert to Opus
                    let mut pcm_buffer: Vec<f32> = vec![0.0; frame_samples];
                    let mut compressed_data: Vec<u8> = vec![0; frame_samples];

                    println!("Finished Audio Input Prep...");

                    // Build the input stream
                    // Input stream
                    let input_stream = input_device
                        .build_input_stream(
                            &input_config,
                            move |data: &[f32], _: &InputCallbackInfo| {
                                let mut buffer = Vec::new();

                                // Handle mono or stereo input
                                let num_channels = input_config.channels as usize;
                                if num_channels == 1 {
                                    // Convert mono to stereo
                                    for &sample in data {
                                        buffer.push(sample * input_volume); // Left channel
                                        buffer.push(sample * input_volume); // Right channel
                                    }
                                } else if num_channels == 2 {
                                    // Data will be like the following [Left, Right, Left, Right]
                                    // Since its Dual Channel Audio
                                    for &sample in data {
                                        buffer.push(sample * input_volume);
                                    }
                                }

                                // Resample if necessary
                                if input_sample_rate != output_sample_rate {
                                    if !buffer.is_empty() {
                                        buffer = convert(
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

                                // TODO: Send audio via UDP socket every 20ms
                                // Example: Encode with Opus @ 48kHz dual channel
                                if consumer.iter().count() >= frame_samples {
                                    println!("[AUDIO DRIVER] Prepairing to Send 20ms of Data to Server");
                                    println!("[AUDIO DRIVER]    or {} samples", frame_samples);

                                    // println!("[AUDIO DRIVER/LIB] Sending Audio Data to Server");

                                    // Fill the PCM buffer with 20ms worth of samples
                                    pcm_buffer.clear();
                                    for _ in 0..frame_samples {
                                        if let Some(sample) = consumer.try_pop() {
                                            pcm_buffer.push(sample as f32);
                                        }
                                    }

                                    println!("Uploading to Server with Buffer Size: {}", pcm_buffer.len());


                                    // Compress the PCM data
                                    match encoder.encode_float(&pcm_buffer, &mut compressed_data) {
                                        Ok(compressed_size) => {
                                            compressed_data.truncate(compressed_size);

                                            println!("Sending Data to Server of Size {} bytes.", compressed_size);

                                            let temp_data = compressed_data.clone(); // Clone the data for the async task
                                            let temp_ip = Arc::clone(&server_ip); // Clone server_ip
                                            let temp_socket = Arc::clone(&socket); // Clone the socket
                                            tauri::async_runtime::spawn(async move {
                                                temp_socket.send_to(&temp_data, &*temp_ip).await
                                            });
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "[AUDIO DRIVER/LIB] Failed to encode PCM data: {}",
                                                e
                                            );
                                        }
                                    }
                                }
                            },
                            |err| eprintln!("[AUDIO DRIVER] Input stream error: {}", err),
                            None,
                        )
                        .expect("[AUDIO DRIVER] Failed to create input stream.");

                    //print_type_of(&input_stream);

                    // Start the input stream
                    if let Err(err) = input_stream.play() {
                        eprintln!(
                            "[AUDIO DRIVER : DEBUG] Failed to start input stream: {}",
                            err
                        );
                        return;
                    } else {
                        input_stream_active.store(true, Ordering::SeqCst);
                    }

                    println!("[AUDIO DRIVER] Audio capture started.");

                    while input_stream_active.load(Ordering::SeqCst) {
                        // Allow other threads to work (wait 5 samples of time)
                        let sample_time = 60.0 / input_sample_rate as f32;
                        std::thread::sleep(std::time::Duration::from_millis(
                            (sample_time * 5.0) as u64,
                        ));
                    }

                    println!("[AUDIO DRIVER] Audio capture ended.");
                }
                Err(e) => {
                    eprintln!("[AUDIO DRIVER] Failed to create encoder: {}\n[AUDIO DRIVER] Stopped Audio Capture", e);
                }
            }
        });
    }

    pub fn start_audio_playback(&mut self, socket: Arc<tokio::net::UdpSocket>) {
        //Create thread for CPAL AUdio Output waiting for new data and playing that new data
        let output_stream_active_a = self.output_stream.clone();
        let output_stream_active_b = self.output_stream.clone();

        tauri::async_runtime::spawn(async move {
            let host = cpal::default_host();
            let (sender, mut reciever) = watch::channel(());

            println!("[AUDIO DRIVER] Started Audio Recording");

            // Get the output device (based on the Users Input)
            let devices = AudioDriver::get_current_audio_devices();
            let output_device_name = devices.get(1);
            let output_device = match output_device_name {
                Some(name) => AudioDriver::get_output_device_by_name(name),
                None => None,
            };
        
            // If output_device is still None, fall back to the host's default output device.
            let output_device = match output_device {
                Some(device) => device,
                None => match host.default_output_device() {
                    Some(device) => device,
                    None => {
                        eprintln!("[AUDIO DRIVER] No Output device available.");
                        return;
                    }
                },
            };

            // Get output config [ERROR at LINE 538 (LINE BELOW v)]
            let output_config: StreamConfig = match output_device.default_output_config() {
                Ok(config) => config.into(),
                Err(err) => {
                    eprintln!("[AUDIO DRIVER] Failed to get default output configuration: {:?}", err);
                    return;
                }
            };

            // Extract output sample rate and format
            let output_sample_rate = output_config.sample_rate.0;
            let input_sample_rate = 48000; // Use 48kHz for output
            let channels = output_config.channels;

            println!(
                "[AUDIO DRIVER] Selected Device Config:\n     Input Device {};\n     Channels: {};\n     Sample Rate: {};",
                output_device.name().unwrap_or("Unknown Device".to_string()),
                output_config.channels,
                output_config.sample_rate.0,
            );

            // Create a ring buffer for audio samples   {Force Dual Channel} * {2 seconds}
            let ring = HeapRb::<f32>::new(input_sample_rate as usize * 2 * 2);
            let (mut producer, mut consumer) = ring.split();

            let volumes = AudioDriver::get_current_audio_volumes();
            let output_volume: f32 = *volumes.get(1).unwrap_or(&100.0) / 100.0;

            tauri::async_runtime::spawn(async move {
                let stream = output_device.build_output_stream(&output_config, move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        for (i, sample) in data.iter_mut().enumerate() {
                            // Attempt to pop a sample from the ring buffer
                            match consumer.try_pop() {
                                Some(value) => {
                                    *sample = value * output_volume; // Use the sample from the buffer
                                },
                                None => {
                                    *sample = 0.0; // Fill with silence if the buffer is empty
                                }
                            }
                        }
                    }, 
                |err| eprintln!("[AUDIO DRIVER] Error in output stream: {}", err), 
                None
                ).expect("Failed to build output stream");

                output_stream_active_a.store(true, Ordering::SeqCst);
                stream.play().expect("Failed to start output stream");

                while output_stream_active_a.load(Ordering::SeqCst) {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }

                drop(stream)
            });

            println!("[AUDIO DRIVER] Audio Output Stream Started.");

            //ERROR. WHEN PUTTING THIS CODE INTO A THREAD
            println!("[AUDIO DRIVER/NET] Audio Decoder Creating...");
            let mut temp_decoder = Decoder::new(SampleRate::Hz48000, Channels::Stereo);
        
            match temp_decoder {
                Ok(mut decoder) => {
                    println!("[AUDIO DRIVER/NET] Waiting for Socket Data...");
                    loop {
                        let mut buf = [0; 2048];
                        let mut pcm_audio = vec![0.0; 1920]; //1920 = 48khz * 0.02 * 2

                        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
                            println!("[AUDIO DRIVER/NET] {:?} bytes received from {:?}", len, addr);
                            // Opus Decode -> Play Audio

                            //Need data in type of u8
                            let vec_buf: Vec<u8> = Vec::from(&buf[..len]);

                            // Decode the received Opus data into PCM samples
                            match decoder.decode_float(Some(&vec_buf), &mut pcm_audio, false) {
                                Ok(decoded_len) => {
                                    //Change to Different Sample Rate
                                    println!("[AUDIO DRIVER/NET] Recieved Packets, Decoded of Size: {}", decoded_len);
                                    if input_sample_rate != output_sample_rate {
                                        if !pcm_audio.is_empty() {
                                            pcm_audio = convert(
                                                input_sample_rate,
                                                output_sample_rate,
                                                2,
                                                ConverterType::Linear,
                                                &pcm_audio,
                                            )
                                            .expect("Resampling failed");
                                        }
                                        else {
                                            println!("[AUDIO DRIVER/NET] Failed to Resample, No PCM Audio Given");
                                        }
                                    }

                                    //Put in Ring Buffer
                                    //If Mono Output, Skip Every Other [L, R, L, R, ...]
                                    for (i, sample) in pcm_audio.iter().enumerate() {
                                        // Convert sample to f32 and push to the producer
                                        //println!("[AUDIO DRIVER/NET] Pushing {} Samples into Buffer.", channels);
                                        if (channels == 2) {
                                            if let Err(e) = producer.try_push(*sample) {
                                                eprintln!("[LIB/OUT] Failed to push audio sample to ring buffer: {:?}", e);
                                            }
                                        }
                                        else if (channels == 1 && i % 2 == 0){
                                            if let Err(e) = producer.try_push(*sample) {
                                                eprintln!("[LIB/OUT/SINGLE CHANNEL] Failed to push audio sample to ring buffer: {:?}", e);
                                            }
                                        }
                                    }
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

            println!("[AUDIO DRIVER] Audio Output Recieve Thread Created.");

            while output_stream_active_b.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(15));
            }

            std::thread::sleep(std::time::Duration::from_secs(15));

            println!("[AUDIO DRIVER] Audio Output Stream Ended.");
            sender.send(()).unwrap();
        });
    }

    pub fn swap_audio_ouput(&self) {

    }

    pub fn get_default_output_device_name() -> String {
        let host = cpal::default_host();
        if let Some(device) =  host.default_output_device() {
            if let Ok(device_name) = device.name() {
                return device_name;
            }
        }

        return "".to_string()
    }

    pub fn get_default_input_device_name() -> String {
        let host = cpal::default_host();
        if let Some(device) =  host.default_input_device() {
            if let Ok(device_name) = device.name() {
                return device_name;
            }
        }

        return "".to_string()
    }

    pub fn audio_debugger(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Clone atomic flags for thread-safe sharing
        let input_stream_active = self.input_stream.clone();
        let output_stream_active = self.output_stream.clone();

        tauri::async_runtime::spawn(async move {
            println!("[AUDIO DRIVER : DEBUG] Starting audio debugger...");

            // Initialize the default audio host
            let host = cpal::default_host();

            // Get default input and output devices
            let input_device = match host.default_input_device() {
                Some(device) => device,
                None => {
                    eprintln!("[AUDIO DRIVER : DEBUG] No input device found.");
                    return;
                }
            };

            let output_device = match host.default_output_device() {
                Some(device) => device,
                None => {
                    eprintln!("[AUDIO DRIVER : DEBUG] No output device found.");
                    return;
                }
            };

            // Get input and output configurations
            let input_config: StreamConfig = input_device.default_input_config().unwrap().into();
            let output_config: StreamConfig = output_device.default_output_config().unwrap().into();

            println!(
                "[AUDIO DRIVER : DEBUG] Using input device: {} with config: {:?}",
                match input_device.name() {
                    Ok(name) => name,
                    Err(_) => "Unknown Input Device".to_string(),
                },
                input_config
            );

            println!(
                "[AUDIO DRIVER : DEBUG] Using output device: {} with config: {:?}\n",
                match output_device.name() {
                    Ok(name) => name,
                    Err(_) => "Unknown Output Device".to_string(),
                },
                output_config
            );

            // Create a ring buffer for audio samples
            let buffer_capacity =
                input_config.sample_rate.0 as usize * input_config.channels as usize;
            let ring = HeapRb::<f32>::new(buffer_capacity);
            let (mut producer, mut consumer) = ring.split();

            // Input stream
            let input_stream = input_device
                .build_input_stream(
                    &input_config,
                    move |data: &[f32], _: &InputCallbackInfo| {
                        // Push audio samples to the producer
                        for &sample in data {
                            if !producer.is_full() {
                                //turn mono to 2 channel
                                if (input_config.channels == 1) {
                                    producer.try_push(sample).unwrap();
                                }
                                producer.try_push(sample).unwrap();
                            }
                        }
                    },
                    |err| eprintln!("[AUDIO DRIVER : DEBUG] Input stream error: {}", err),
                    None,
                )
                .expect("[AUDIO DRIVER : DEBUG] Failed to create input stream.");

            // Output stream
            let output_stream = output_device
                .build_output_stream(
                    &output_config,
                    move |output_data: &mut [f32], _: &OutputCallbackInfo| {
                        let input_sample_rate = input_config.sample_rate.0 as u32;
                        let output_sample_rate = output_config.sample_rate.0 as u32;
                        let mut buffer = Vec::new();

                        // Read samples from the consumer
                        while let Some(sample) = consumer.try_pop() {
                            buffer.push(sample);
                        }

                        // Resample if necessary
                        if input_sample_rate != output_sample_rate {
                            if !buffer.is_empty() {
                                buffer = convert(
                                    input_sample_rate,
                                    output_sample_rate,
                                    1, // Assuming mono audio
                                    ConverterType::Linear,
                                    &buffer,
                                )
                                .expect("Resampling failed");
                            }
                        }

                        // Fill the output buffer
                        for (i, sample) in output_data.iter_mut().enumerate() {
                            *sample = buffer.get(i).cloned().unwrap_or(0.0);
                        }
                    },
                    |err| eprintln!("[AUDIO DRIVER : DEBUG] Output stream error: {}", err),
                    None,
                )
                .expect("[AUDIO DRIVER : DEBUG] Failed to create output stream.");

            // Play streams
            if let Err(err) = input_stream.play() {
                eprintln!(
                    "[AUDIO DRIVER : DEBUG] Failed to start input stream: {}",
                    err
                );
                return;
            } else {
                input_stream_active.store(true, Ordering::SeqCst);
            }

            if let Err(err) = output_stream.play() {
                eprintln!(
                    "[AUDIO DRIVER : DEBUG] Failed to start output stream: {}",
                    err
                );
                return;
            } else {
                output_stream_active.store(true, Ordering::SeqCst);
            }

            println!("[AUDIO DRIVER : DEBUG] Streams started successfully.");

            // Monitor and stop streams based on atomic flags
            while input_stream_active.load(Ordering::SeqCst)
                || output_stream_active.load(Ordering::SeqCst)
            {
                // Allow other threads to work
                std::thread::sleep(std::time::Duration::from_millis(100));
            }

            println!("\n[AUDIO DRIVER : DEBUG] Stopping streams...");
            drop(input_stream);
            drop(output_stream);
            println!("[AUDIO DRIVER : DEBUG] Streams stopped.");
        });

        Ok(())
    }

    pub fn stop_input_stream(&mut self) {
        println!("[AUDIO DRIVER] Stopping Audio INPUT Stream");
        self.input_stream.store(false, Ordering::SeqCst);
    }

    pub fn stop_output_stream(&mut self) {
        println!("[AUDIO DRIVER] Stopping Audio OUTPUT Stream");
        self.output_stream.store(false, Ordering::SeqCst);
    }


    //================================================================
    //================================================================
    //File Functions
    pub fn initFiles() {
        //Init the files req for Discord 2
        //Init the audio file that stores the audio settings
        //location = /applocaldata/audio.config

        if let Some(loc) = get_config_file() {  
            if (!loc.exists()) {
                if let Some(fileLoc) = loc.to_str() {
                    println!("[AUDIO DRIVER/FS] Creating Audio Config File: {}", fileLoc);
                } else { println!("[AUDIO DRIVER/FS] Creating Audio Config File Failed, Location Unavaliable." ); }

                File::create(loc);

                //Write to File Default Devices
                //Get Default Devices
                let default_input = AudioDriver::get_default_input_device_name();
                let default_output = AudioDriver::get_default_output_device_name();
                AudioDriver::writeAudioConfig(&default_input, &default_output)
            }
        }
    }

    pub fn writeAudioConfig(input_device: &str, output_device: &str) {
        if let Some(loc) = get_config_file() {
            let file_contents = json!({
                "input_device": input_device,
                "output_device": output_device
            });

            if let Ok(json_data) = serde_json::to_string_pretty(&file_contents) {
                println!("[AUDIO DRIVER/FS] Changing Devices: {}", json_data);

                if let Err(e) = write(loc, json_data) {
                    eprintln!("[AUDIO DRIVER/FS] Failed to write audio config to file: {}", e);
                }
            } else {
                eprintln!("[AUDIO DRIVER/FS] Failed to serialize JSON data.");
            }
        } else {
            eprintln!("[AUDIO DRIVER/FS] Failed to get config file location.");
        }
    }

    // [0] = input device
    // [1] = output device
    pub fn get_current_audio_devices() -> Vec<String> {
        // Attempt to open the file
        if let Some(loc) = get_config_file() {
            if let Ok(file) = File::open(loc) {
                let reader = BufReader::new(file);

                // Attempt to parse JSON
                if let Ok(json_data) = serde_json::from_reader::<_, Value>(reader) {
                    // Extract values from the JSON and collect into a Vec
                    let input_device = json_data["input_device"].as_str().unwrap_or("").to_string();
                    let output_device = json_data["output_device"].as_str().unwrap_or("").to_string();

                    return vec![input_device, output_device];
                }
            }
        }

        // Return an empty vector if anything goes wrong
        vec![]
    }

    // [0] = input volume
    // [1] = output volume
    pub fn get_current_audio_volumes() -> Vec<f32> {
        // Attempt to open the file
        if let Some(loc) = get_volume_config_file() {
            if let Ok(file) = File::open(loc) {
                let reader = BufReader::new(file);

                // Attempt to parse JSON
                if let Ok(json_data) = serde_json::from_reader::<_, Value>(reader) {
                    let input_volume: f32 = json_data["input"]
                    .as_f64()
                    .map(|v| v as f32)
                    .unwrap_or(100.0);
                
                    let output_volume: f32 = json_data["output"]
                        .as_f64()
                        .map(|v| v as f32)
                        .unwrap_or(100.0);

                    println!("[AUDIO DRVIER] Input Volume: {}, Output Volume: {}, Raw Data: {}", input_volume, output_volume, json_data["output"]);
                    return vec![input_volume, output_volume];
                } else { println!("FILE CANT BE READ"); }
            } else { println!("FILE NOT FOUND"); }
        }
        else {
            println!("CONFIG NOT FOUND");
        }

        vec![100.0, 100.0]
    }
}

pub fn get_app_dir() -> Option<PathBuf> {
    if let Some(mut base_dir) = dirs::data_local_dir() {
        base_dir.push("discord2");
        fs::create_dir_all(&base_dir);
        return Some(base_dir);
    } else { return None }
}

pub fn get_config_file() -> Option<PathBuf> {
    if let Some(mut base_dir) = dirs::data_local_dir() {
        base_dir.push("discord2");
        fs::create_dir_all(&base_dir);
        base_dir.push("audio");
        base_dir.set_extension("conf");
        return Some(base_dir);
    } else { return None }
}

pub fn get_volume_config_file() -> Option<PathBuf> {
    if let Some(mut base_dir) = dirs::data_local_dir() {
        base_dir.push("discord2");
        fs::create_dir_all(&base_dir);
        base_dir.push("audio-volume");
        base_dir.set_extension("conf");
        return Some(base_dir);
    } else { return None }
}