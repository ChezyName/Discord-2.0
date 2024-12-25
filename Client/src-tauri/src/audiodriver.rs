/**
 * Flawwed Audio Driver That Holds Audio Functions For Getting and Recieving Audio,
 * Audio is created and destroyed frequently and not held in a global state
 */
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, OutputCallbackInfo, StreamConfig,
    Host, Device, BufferSize, Stream
};
use cpal::{SampleFormat, SupportedBufferSize};
use ringbuf::{
    traits::*,
    wrap::caching::Caching,
    storage::Heap,
    HeapRb,
    SharedRb
};
use samplerate::{convert, ConverterType};
use std::string::String;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use audiopus::{coder::Encoder, Application, Channels, SampleRate, Bandwidth};
use std::error::Error;

pub fn run_audio_debugger() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the default audio host
    let host = cpal::default_host();

    // Get default input and output devices
    let input_device = host.default_input_device().expect("Failed to find input device");
    let output_device = host.default_output_device().expect("Failed to find output device");

    // Get input and output configurations
    let input_config: StreamConfig = input_device.default_input_config()?.into();
    let output_config: StreamConfig = output_device.default_output_config()?.into();

    println!("[AUDIO DRIVER] Using input device: \"{}\"", input_device.name()?);
    println!("[AUDIO DRIVER] Using output device: \"{}\"", output_device.name()?);
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
                    if(input_config.channels == 1) { producer.try_push(sample).unwrap(); }
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
                        ConverterType::SincBestQuality,
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
    input_device: String,
    output_device: String,
    input_stream: Arc<AtomicBool>,
    output_stream: Arc<AtomicBool>,
}

impl Default for AudioDriver {
    fn default() -> Self {
        let host = cpal::default_host();

        // Attempt to retrieve input and output devices
        let input_device = host.default_input_device();
        let output_device = host.default_output_device();

        let mut input_device_name: String = String::from("Unknown device");
        let mut output_device_name: String = String::from("Unknown device");

        // Logging device information or fallback messages
        if let Some(ref device) = input_device {
            input_device_name = device.name().unwrap_or_else(|_| "Unknown device".to_string());
            println!(
                "[AUDIO DRIVER] Using input device: \"{}\"",
                device.name().unwrap_or_else(|_| "Unknown device".to_string())
            );
        } else {
            println!("[AUDIO DRIVER] No default input device found.");
        }

        if let Some(ref device) = output_device {
            output_device_name = device.name().unwrap_or_else(|_| "Unknown device".to_string());
            println!(
                "[AUDIO DRIVER] Using output device: \"{}\"",
                device.name().unwrap_or_else(|_| "Unknown device".to_string())
            );
        } else {
            println!("[AUDIO DRIVER] No default output device found.");
        }

        // Create default configurations if no devices are found
        let default_sample_rate = 48000;
        let default_channels = 2;

        let input_sample_rate = input_device
            .as_ref()
            .and_then(|device| device.default_input_config().ok())
            .map(|config| config.sample_rate().0) // Accessing the sample rate via the public method
            .unwrap_or(default_sample_rate);

        let output_sample_rate = output_device
            .as_ref()
            .and_then(|device| device.default_output_config().ok())
            .map(|config| config.sample_rate().0) // Accessing the sample rate via the public method
            .unwrap_or(default_sample_rate);

        println!(
            "[AUDIO DRIVER] Input sample rate: {}, Output sample rate: {}",
            input_sample_rate, output_sample_rate
        );

        // Buffer configuration based on the input sample rate
        let buffer_capacity = input_sample_rate as usize; // 1 second of audio
        let ring = HeapRb::<f32>::new(buffer_capacity * 2);
        let (producer, consumer) = ring.split();

        // Return the constructed AudioDriver
        Self {
            input_device: String::from(input_device_name),
            output_device: String::from(output_device_name),
            input_stream: Arc::new(AtomicBool::new(false)),
            output_stream: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl AudioDriver {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize the default audio host
        let host = cpal::default_host();

        // Get default input and output devices
        let input_device = host.default_input_device().expect("Failed to find input device");
        let output_device = host.default_output_device().expect("Failed to find output device");

        // Get input and output configurations
        let input_config: StreamConfig = input_device.default_input_config()?.into();
        let output_config: StreamConfig = output_device.default_output_config()?.into();

        println!("[AUDIO DRIVER] Using input device: \"{}\"", input_device.name()?);
        println!("[AUDIO DRIVER] Using output device: \"{}\"", output_device.name()?);
        println!(
            "[AUDIO DRIVER] Using input config: sample rate: {}, channels: {}",
            input_config.sample_rate.0, input_config.channels
        );
        println!(
            "[AUDIO DRIVER] Using output config: sample rate: {}, channels: {}",
            output_config.sample_rate.0, output_config.channels
        );

        Ok(Self {
            input_device: String::from(input_device.name().unwrap_or_else(|_| "Unknown device".to_string())),
            output_device: String::from(output_device.name().unwrap_or_else(|_| "Unknown device".to_string())),
            input_stream: Arc::new(AtomicBool::new(false)),
            output_stream: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn get_output_devices(&self) {
        // Iterate through available devices from the host
        let host = cpal::default_host();
        for device in host.devices().unwrap() {
            // Check if the device supports output
            if let Ok(configs) = device.supported_output_configs() {
                println!("[AUDIO DRIVER] Output Device: {}", device.name().unwrap_or("Unknown Device".to_string()));
                
                // Optional: Print supported configurations
                for config in configs {
                    println!(
                        "[AUDIO DRIVER]   Channels: {}, Sample Rates: {}-{}, Sample Format: {:?}",
                        config.channels(),
                        config.min_sample_rate().0,
                        config.max_sample_rate().0,
                        config.sample_format()
                    );
                }
            }
        }
    }

    pub fn get_input_devices(&mut self) {
        // Iterate through available devices from the host
        let host = cpal::default_host();
        for device in host.devices().unwrap() {
            // Check if the device supports output
            if let Ok(configs) = device.supported_input_configs() {
                println!("[AUDIO DRIVER] Input Device: {}", device.name().unwrap_or("Unknown Device".to_string()));
                
                // Optional: Print supported configurations
                for config in configs {
                    println!(
                        "[AUDIO DRIVER]   Channels: {}, Sample Rates: {}-{}, Sample Format: {:?}",
                        config.channels(),
                        config.min_sample_rate().0,
                        config.max_sample_rate().0,
                        config.sample_format()
                    );
                }
            }
        }
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
                    println!("[AUDIO DRIVER] Input device with name '{}' not found.", target);
                }
            }
            Err(err) => eprintln!("[AUDIO DRIVER] Failed to enumerate devices: {}", err),
        }

        return found_device
    }

    pub fn start_audio_capture(&mut self, socket: Arc<tokio::net::UdpSocket>, server_ip: Arc<String>) {
        let input_stream_active = self.input_stream.clone();
        
        tauri::async_runtime::spawn(async move {
            let host = cpal::default_host();

            println!("[AUDIO DRIVER] Started Audio Recording");
        
            // Get the input device
            let input_device = match host.default_input_device() {
                Some(device) => device,
                None => {
                    eprintln!("[AUDIO DRIVER] No input device available.");
                    return;
                }
            };
        
            // Get default input device
            let input_device = match host.default_input_device() {
                Some(device) => device,
                None => {
                    eprintln!("[AUDIO DRIVER : DEBUG] No input device found.");
                    return;
                }
            };

            // Get input config
            let input_config: StreamConfig = input_device.default_input_config().unwrap().into();
        
            // Extract input sample rate and format
            let input_sample_rate = input_config.sample_rate.0;
            let output_sample_rate = 48000; // Use 48kHz for output
        
            // Create a ring buffer for audio samples   {Force Dual Channel}
            let ring = HeapRb::<f32>::new(input_sample_rate as usize * 2);
            let (mut producer, mut consumer) = ring.split();
            
            //Encode the Audio with Opus
            let mut temp_encoder = Encoder::new(SampleRate::Hz48000, Channels::Stereo, Application::Voip);
            
            match temp_encoder {
                Ok(mut encoder) => {
                    // Set the bitrate and max bandwidth after successfully creating the encoder
                    //coder.set_bitrate(64000);
                    encoder.set_max_bandwidth(Bandwidth::Fullband);
                    println!("[AUDIO DRIVER] Encoder Successfully Initialized");

                    //0.02 = 20ms, 2 = 2 channels + extra 1.5
                    let frame_samples: usize = (output_sample_rate as f32 * 0.02 * 2.0 * 1.5) as usize;

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
                    
                                println!("-1");

                                // Handle mono or stereo input
                                let num_channels = input_config.channels as usize;
                                if num_channels == 1 {
                                    // Convert mono to stereo
                                    for &sample in data {
                                        buffer.push(sample); // Left channel
                                        buffer.push(sample); // Right channel
                                    }
                                } else if num_channels == 2 {
                                    // Directly copy stereo data
                                    buffer.extend_from_slice(data);
                                }
                    
                                // Resample if necessary
                                if input_sample_rate != output_sample_rate {
                                    if !buffer.is_empty() {
                                        buffer = convert(
                                            input_sample_rate,
                                            output_sample_rate,
                                            2, // Dual-channel audio
                                            ConverterType::SincBestQuality,
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

                                    println!("[AUDIO DRIVER/LIB] Sending Audio Data to Server");
                                        
                                    // Fill the PCM buffer with 20ms worth of samples
                                    pcm_buffer.clear();
                                    for _ in 0..frame_samples {
                                        if let Some(sample) = consumer.try_pop() {
                                            pcm_buffer.push(sample as f32); // Convert f32 to i16
                                        }
                                    }
                                
                                    // Compress the PCM data
                                    match encoder.encode_float(&pcm_buffer, &mut compressed_data) {
                                        Ok(compressed_size) => {
                                            compressed_data.truncate(compressed_size);

                                            socket.send_to(&compressed_data, &*server_ip);
                                        }
                                        Err(e) => {
                                            eprintln!("[AUDIO DRIVER/LIB] Failed to encode PCM data: {}", e);
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
                        eprintln!("[AUDIO DRIVER : DEBUG] Failed to start input stream: {}", err);
                        return;
                    }
                    else { input_stream_active.store(true, Ordering::SeqCst); }
                
                    println!("[AUDIO DRIVER] Audio capture started.");

                    while input_stream_active.load(Ordering::SeqCst)
                    {
                        // Allow other threads to work (wait 5 samples of time)
                        let sample_time = 60.0 / input_sample_rate as f32;
                        std::thread::sleep(std::time::Duration::from_millis((sample_time * 5.0) as u64));
                    }

                    println!("[AUDIO DRIVER] Audio capture ended.");
                }
                Err(e) => {
                    eprintln!("[AUDIO DRIVER] Failed to create encoder: {}\n[AUDIO DRIVER] Stopped Audio Capture", e);
                }
            }
        });
    }

    //Returns the last 20ms of audio or sample_rate * 0.2;
    //If sample rate is 44800 then return 896 samples.
    //Additionally compresses using Opus.
    pub fn get_audio(&mut self) {
        
    }

    //changes the input device and changes the input stream
    pub fn swap_audio_input(&mut self, input_device: &str) {

    }

    //changes the ouput device
    pub fn swap_audio_ouput(&mut self, input_device: &str) {

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
            let buffer_capacity = input_config.sample_rate.0 as usize * input_config.channels as usize;
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
                                if(input_config.channels == 1) { producer.try_push(sample).unwrap(); }
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
                                    ConverterType::SincBestQuality,
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
                eprintln!("[AUDIO DRIVER : DEBUG] Failed to start input stream: {}", err);
                return;
            }
            else { input_stream_active.store(true, Ordering::SeqCst); }
    
            if let Err(err) = output_stream.play() {
                eprintln!("[AUDIO DRIVER : DEBUG] Failed to start output stream: {}", err);
                return;
            }
            else { output_stream_active.store(true, Ordering::SeqCst); }
    
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
}