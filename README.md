# Discord2.0
A Full Recreation of Discord

# To do
- ~~**Heartbeat** to make sure sure user is still active (by server pov)~~
- ~~Opus Audio Sending~~
- ~~Opus Audio Decoding~~
- Audio Playing

- **MEMORY LEAK:** There is a memory leak from the Rodio Audio Driver since the stream is force dropped and not managed by Rust.
    - Solution is Global Audio Driver (Tauri State Management)
- Automatic Client-Side Diconnection

Audio goes from the following:
- Starts at Microphone Input
- Compressed into Opus Packet
- Sent to Server via UDP
- Sent to all clients (debug) or other clients (prod) connected via UDP
- Clients Decompress packet
- Opus packet is played

> All audio sent will always be dual channel @ 48khz for standardization

# Requirements
> I built this using Windows, so if you have a mac (osX) or Linux of any kind, good luck.

- cmake
- ASIO