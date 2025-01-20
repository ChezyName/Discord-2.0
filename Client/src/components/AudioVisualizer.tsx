import React, { useEffect, useState } from "react";
import Box from "@mui/material/Box";

interface AudioVisualizerProps {
  pcmData: Float32Array | null; // PCM audio data slice
}

const AudioBarVisualizer: React.FC<AudioVisualizerProps> = ({ pcmData }) => {
  const [barHeights, setBarHeights] = useState<number[]>(Array(30).fill(0)); // 30 bars for visualization
  const [barColors, setBarColors] = useState<string[]>(Array(30).fill("#555")); // Initial color

  useEffect(() => {
    if (!pcmData) return;

    const framesPerBar = Math.floor(pcmData.length / barHeights.length); // Number of samples per bar
    const updatedHeights: number[] = [];
    const updatedColors: string[] = [];

    for (let i = 0; i < barHeights.length; i++) {
      const start = i * framesPerBar;
      const end = start + framesPerBar;

      const slice = pcmData.slice(start, end); // Slice of PCM data for this bar
      const avgAmplitude = slice.reduce((sum, val) => sum + Math.abs(val), 0) / slice.length;

      const dB = 20 * Math.log10(Math.max(avgAmplitude, 0.00001)); // Convert to dB
      const height = Math.max((dB + 100) / 100, 0); // Normalize to [0, 1] range
      updatedHeights.push(height);

      // Assign color based on dB range
      if (dB >= -20) {
        updatedColors.push("lime");
      } else if (dB >= -60) {
        updatedColors.push("yellow");
      } else {
        updatedColors.push("red");
      }
    }

    setBarHeights(updatedHeights);
    setBarColors(updatedColors);
  }, [pcmData]);

  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "center",
        alignItems: "flex-end",
        height: 100,
        width: "100%",
        background: "#2c2f33",
        padding: "10px 0",
        gap: "2px",
      }}
    >
      {barHeights.map((height, index) => (
        <Box
          key={index}
          sx={{
            width: "4px",
            height: `${height * 100}%`, // Scale height dynamically
            backgroundColor: barColors[index],
            borderRadius: "2px",
          }}
        />
      ))}
    </Box>
  );
};

export default AudioBarVisualizer;