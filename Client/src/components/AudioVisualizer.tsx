import { useEffect, useState } from "react";
import Box from "@mui/material/Box";

interface AudioVisualizerProps {
  pcmData: Float32Array | null; // PCM audio data slice
}

const ProgressBarsCount = 20;
const Min_dB = -100;
const Max_dB = 0;

export default function AudioVisualizer({ pcmData }: AudioVisualizerProps) {
  const [amplitude, setAmplitude] = useState<number>(0); // Current amplitude percentage

  useEffect(() => {
    if (!pcmData) { setAmplitude(0); return; }
  
    const framesPerSlice = pcmData.length / 2; // Number of left-right pairs
    const frameDuration = 20 / framesPerSlice; // Duration per frame in ms
  
    let currentFrame = 0;
  
    const processFrames = () => {
      if (currentFrame >= framesPerSlice) return; // End of PCM slice
  
      // Calculate average amplitude for the current frame
      const left = pcmData[currentFrame * 2];
      const right = pcmData[currentFrame * 2 + 1];
      const averageAmplitude = (Math.abs(left) + Math.abs(right)) / 2;
  
      // Convert amplitude to dB, ensure non-zero to avoid log issues
      const dB = 20 * Math.log10(Math.max(averageAmplitude, 0.0001));

      const progress = (dB - Min_dB) / (Max_dB - Min_dB);
      setAmplitude(Math.round(ProgressBarsCount * progress));
  
      // Update amplitude (scale dB to match slider range)
      //setAmplitude(dB);
  
      // Move to the next frame
      currentFrame++;
      setTimeout(processFrames, frameDuration); // Process the next frame after the delay
    };
  
    processFrames(); // Start processing the PCM slice
  
    return () => {
      // Cleanup: Stop processing if the component unmounts
      currentFrame = framesPerSlice;
    };
  }, [pcmData]);
  

  return (
    <Box 
      sx={{ 
        width: '100%', 
        display: 'flex', 
        flexDirection: 'row',
        gap: '8px',
        alignItems: "stretch",
        marginLeft: "12px",
      }}
    >
      {Array.from({ length: ProgressBarsCount }).map((_, index) => (
        <Box 
          key={index} 
          className="progress-bar-box"
          sx={{
            backgroundColor: (amplitude >= (index + 1) ? "var(--Primary)" : "var(--Secondary)"),
            width: '100%',
            height: "100%",
            borderRadius: "25px",
            border: "2px solid var(--Outlines)"
          }}
        ></Box>
      ))}
    </Box>
  );
}
