// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::thread;
use std::time::Duration;
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

//mod audiodriver;

fn main() {
    //audiodriver::run_audio_debugger();
    discord2_lib::run();
}