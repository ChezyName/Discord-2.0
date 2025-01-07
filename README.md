# Discord2.0
A Full Recreation of Discord
> Currently Being Built

# To do
- ~~**Heartbeat** to make sure sure user is still active (by server pov)~~
- ~~Opus Audio Sending~~
- ~~Opus Audio Decoding~~
- ~~Audio Playing~~
- Text Messages

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


# Running The Server
> Currently the Data Server runs on port `3001` and the UDP voice server runs on port `3000`, meaning that you need to open these ports.

To run the server, just simply execute the executable or using source code run `go run .` or `go run main.go`.

to change the name just attach `-name=YOUR_NAME_HERE`, for example `-name=DiscordTwoo`
to debug the server, just attach `-debug=true`