/**
 * Flawwed Audio Driver That Holds Audio Functions For Getting and Recieving Audio,
 * Audio is created and destroyed frequently and not held in a global state
 */

#![no_std]
extern crate alloc;

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, OutputCallbackInfo, StreamConfig,
    Host, Device, BufferSize
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
use alloc::sync::Arc;

pub fn run_audio_debugger() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the default audio host
    let host = cpal::default_host();

    // Get default input and output devices
    let input_device = host.default_input_device().expect("Failed to find input device");
    let output_device = host.default_output_device().expect("Failed to find output device");

    // Get input and output configurations
    let input_config: StreamConfig = input_device.default_input_config()?.into();
    let output_config: StreamConfig = output_device.default_output_config()?.into();

    println!("Using input device: \"{}\"", input_device.name()?);
    println!("Using output device: \"{}\"", output_device.name()?);
    println!(
        "Using input config: sample rate: {}, channels: {}",
        input_config.sample_rate.0, input_config.channels
    );
    println!(
        "Using output config: sample rate: {}, channels: {}",
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
        |err| eprintln!("Error in input stream: {}", err),
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
        |err| eprintln!("Error in output stream: {}", err),
        None,
    )?;

    // Start streams
    input_stream.play()?;
    output_stream.play()?;

    println!("Audio loopback started. Press Enter to stop...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}

//Use this to connect with both the input and output
pub struct AudioDriver {
    host: Host,
    input_device: Option<Device>,
    output_device: Option<Device>,
}

impl Default for AudioDriver {
    fn default() -> Self {
        let host = cpal::default_host();

        // Attempt to retrieve input and output devices
        let input_device = host.default_input_device();
        let output_device = host.default_output_device();

        // Logging device information or fallback messages
        if let Some(ref device) = input_device {
            println!(
                "Using input device: \"{}\"",
                device.name().unwrap_or_else(|_| "Unknown device".to_string())
            );
        } else {
            println!("No default input device found.");
        }

        if let Some(ref device) = output_device {
            println!(
                "Using output device: \"{}\"",
                device.name().unwrap_or_else(|_| "Unknown device".to_string())
            );
        } else {
            println!("No default output device found.");
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
            "Input sample rate: {}, Output sample rate: {}",
            input_sample_rate, output_sample_rate
        );

        // Buffer configuration based on the input sample rate
        let buffer_capacity = input_sample_rate as usize; // 1 second of audio
        let ring = HeapRb::<f32>::new(buffer_capacity * 2);
        let (producer, consumer) = ring.split();

        // Return the constructed AudioDriver
        Self {
            host,
            input_device,
            output_device,
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

        println!("Using input device: \"{}\"", input_device.name()?);
        println!("Using output device: \"{}\"", output_device.name()?);
        println!(
            "Using input config: sample rate: {}, channels: {}",
            input_config.sample_rate.0, input_config.channels
        );
        println!(
            "Using output config: sample rate: {}, channels: {}",
            output_config.sample_rate.0, output_config.channels
        );

        Ok(Self {
            host: host,
            input_device: Some(input_device),
            output_device: Some(output_device),
        })
    }

    pub fn get_output_devices(&self) {
        // Iterate through available devices from the host
        for device in self.host.devices().unwrap() {
            // Check if the device supports output
            if let Ok(configs) = device.supported_output_configs() {
                println!("Output Device: {}", device.name().unwrap_or("Unknown Device".to_string()));
                
                // Optional: Print supported configurations
                for config in configs {
                    println!(
                        "  Channels: {}, Sample Rates: {}-{}, Sample Format: {:?}",
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
        for device in self.host.devices().unwrap() {
            // Check if the device supports output
            if let Ok(configs) = device.supported_input_configs() {
                println!("Input Device: {}", device.name().unwrap_or("Unknown Device".to_string()));
                
                // Optional: Print supported configurations
                for config in configs {
                    println!(
                        "  Channels: {}, Sample Rates: {}-{}, Sample Format: {:?}",
                        config.channels(),
                        config.min_sample_rate().0,
                        config.max_sample_rate().0,
                        config.sample_format()
                    );
                }
            }
        }
    }

    pub fn start_audio_capture(&mut self, socket: Option<tokio::net::UdpSocket>) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure input_device is available
        let input_device = self.input_device.as_ref().ok_or("Input device is not available")?;
        
        let input_config = input_device
            .default_input_config() // Call the method on the input_device
            .map_err(|_| "Failed to get input config")?; // Handle failure to get input config
        
        let input_sample_rate = input_config.sample_rate().0 as u32;
        let output_sample_rate = 44800; // Force Use 44.8kHz

        //Two Channel Audio Heap
        let ring = HeapRb::<f32>::new(input_sample_rate as usize * 2 as usize);
        let (mut producer, mut consumer) = ring.split();

        let stream_config = cpal::StreamConfig {
            channels: input_config.channels(),
            sample_rate: input_config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        // Build the input stream using the input_config
        let input_stream = input_device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &InputCallbackInfo| {
                let mut buffer = Vec::new();

                // Check if the input is mono or stereo
                let num_channels = input_config.channels() as usize; // Get the number of input channels

                // Handle mono to stereo conversion if necessary
                if num_channels == 1 {
                    // Mono input: duplicate the samples to create a stereo output (2 channels)
                    for sample in data {
                        // Duplicate the sample for both channels
                        buffer.push(*sample); // Left channel
                        buffer.push(*sample); // Right channel
                    }
                } else if num_channels == 2 {
                    // Stereo input: just copy the data as is
                    buffer.extend_from_slice(data);
                }

                buffer.extend_from_slice(data);

                // Resample if necessary
                if input_sample_rate != output_sample_rate {
                    if !buffer.is_empty() {
                        buffer = convert(
                            input_sample_rate,
                            output_sample_rate,
                            2, // Dual-channel audio (stereo)
                            ConverterType::SincBestQuality,
                            &buffer,
                        )
                        .expect("Resampling failed");
                    }
                }

                // Fill the ring buffer
                for sample in buffer.iter() {
                    producer.try_push(*sample).unwrap_or_else(|e| {
                        eprintln!("Failed to push sample to producer: {:?}", e);
                        consumer.try_pop();
                    });
                }

                //Send Audio Socket Every 20ms
                //Encode with Opus @ 44.8khz dual channel
            },
            |err| eprintln!("Error in input stream: {}", err),
            None,
        )?;

        print_type_of(&input_stream);

        // Start the input stream
        input_stream.play()?;
        Ok(())
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
}