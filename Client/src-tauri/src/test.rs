/*  MUST BE IN 1 FUNCTION TO BE CALLED VIA `cargo.run test.rs``
    Class to test Server,
        Start Timer
    Get Audio (20ms)
        20ms of Audio
    Encode to Opus
        Encode Time
    Send to Server
        Audio INPUT Completed Time

    Wait for Audio Input
        Audio Recieve Time (Network Round Trip Time RTT)
    Decode Opus
        Audio Decode Time
    Play Audio (For Testing Purposes)
*/