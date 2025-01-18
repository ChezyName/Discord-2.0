import { Button, MenuItem, Select, Slider, Typography } from '@mui/material'
import React, { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

const Voice = () => {
  const [isAudioTest, setAudioTest] = useState(false);
  
  const [InputDevices, setInputDevices] = useState<string[]>([]);
  const [OutputDevices, setOutputDevices] = useState<string[]>([]);

  useEffect(() => {
    setAudioTest(false);
    return () => {setAudioTest(false);}
  }, []);

  useEffect(() => {
    if(isAudioTest) invoke('start_audio_test');
    else invoke('stop_audio_test');

    return () => {
      invoke('stop_audio_test');
    }
  }, [isAudioTest]);

  //Device Init
  useEffect(() => {
    invoke('get_input_devices').then((data) => {
      console.log("Input Devices:", data)
      setInputDevices(data as string[]);
    })

    invoke('get_output_devices').then((data) => {
      console.log("Output Devices:", data)
      setOutputDevices(data as string[]);
    })
  }, [])

  return (
    <>
      {/** Input & Output Selection */}
      <div style={{width: "100%", height: "auto", display: "flex", marginBottom: "12px", borderBottom: "2px solid var(--Outlines)", gap: "4%"}}>
        <div style={{width: "48%", height: "auto", display: "flex", flexDirection: "column"}}>
          <Typography sx={{fontSize: "24px", width: "100%", textAlign: "center"}}>Input Device</Typography>
          <Select>
            {
              InputDevices.map((Device:string) => {
                return <MenuItem value={Device}>{Device}</MenuItem>
              })
            }
          </Select>
          <Slider />
        </div>

        <div style={{width: "48%", height: "auto", display: "flex", flexDirection: "column"}}>
          <Typography sx={{fontSize: "24px", width: "100%", textAlign: "center"}}>Output Device</Typography>
          <Select>
            {
              OutputDevices.map((Device:string) => {
                return <MenuItem value={Device}>{Device}</MenuItem>
              })
            }
          </Select>
          <Slider />
        </div>
      </div>

      <div style={{width: "100%", height: "auto", display: "flex"}}>
        <Button onClick={() => {setAudioTest(!isAudioTest)}} sx={{border: "1px solid var(--Outlines)"}}>
          {isAudioTest ? "Stop Checking" : "Let's Check"}
        </Button>
      </div>
    </>
  )
}

export default Voice