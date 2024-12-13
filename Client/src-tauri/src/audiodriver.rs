use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    InputCallbackInfo, OutputCallbackInfo, StreamConfig,
    Host, Device
};
use ringbuf::{
    traits::*,
    wrap::caching::Caching,
    storage::Heap,
    HeapRb,
    SharedRb
};
use samplerate::{convert, ConverterType};
use std::sync::Arc;

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
    let buffer_capacity = input_config.sample_rate.0 as usize; // Buffer for 1 second of audio
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
    prod: Caching<Arc<SharedRb<Heap<f32>>>, true, false>,
    cons: Caching<Arc<SharedRb<Heap<f32>>>, false, true>,

    host: Host,
    input_device: Device,
    output_device: Device,
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

        // Create a buffer for raw audio
        let buffer_capacity = input_config.sample_rate.0 as usize; // Buffer for 1 second of audio
        let ring = HeapRb::<f32>::new(buffer_capacity * 2);
        let (mut producer, mut consumer) = ring.split();

        Ok(Self {
            prod: producer,
            cons: consumer,

            host: host,
            input_device: input_device,
            output_device: output_device,
        })
    }

    /*
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

    pub fn start_audio_capture(&mut self) {
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
        input_stream.play()?;
    }

    //Returns the last 20ms of audio or sample_rate * 0.2;
    //If sample rate is 44800 then return 896 samples.
    //Additionally compresses using Opus.
    pub fn get_audio(&mut self) {

    }

    //changes the input device
    pub fn swap_audio_input(&mut self, input_device: &str) {

    }

    //changes the ouput device
    pub fn swap_audio_ouput(&mut self, input_device: &str) {

    }
    */
}