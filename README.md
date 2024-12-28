# Discord2.0
A Full Recreation of Discord

# To do
- ~~**Heartbeat** to make sure sure user is still active (by server pov)~~
- ~~Opus Audio Sending~~
- ~~Opus Audio Decoding~~
- Audio Playing

- **MEMORY LEAK:** There is a memory leak from the Rodio Audio Driver since the stream is force dropped and not managed by Rust.
- Automatic Client-Side Diconnection

# Requirements
> I built this using Windows, so if you have a mac (osX) or Linux of any kind, good luck.

- cmake
- ASIO