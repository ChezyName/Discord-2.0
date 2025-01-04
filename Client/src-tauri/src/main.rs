// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Device, Host, InputCallbackInfo, OutputCallbackInfo, StreamConfig,
};
use cpal::{SampleFormat, SupportedBufferSize};
use ringbuf::{storage::Heap, traits::*, wrap::caching::Caching, HeapRb, SharedRb};
use samplerate::{convert, ConverterType};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

//mod audiodriver;

fn main() {
    //audiodriver::run_audio_debugger();
    discord2_lib::run();
}
