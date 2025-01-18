import { Button, MenuItem, Select, Slider, Typography } from '@mui/material';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

//Loads Devices into Memory (LocalStorage) for 'Faster' Load Times
export function InitDevices() {

}

const Voice = () => {
  const [isAudioTest, setAudioTest] = useState(false);
  
  const [InputDevices, setInputDevices] = useState<string[]>([]);
  const [OutputDevices, setOutputDevices] = useState<string[]>([]);

  const [CurrentInputDevice, setCurrentInputDevice] = useState<string>('');
  const [CurrentOutputDevice, setCurrentOutputDevice] = useState<string>('');

  useEffect(() => {
    setAudioTest(false);
    return () => { setAudioTest(false); };
  }, []);

  useEffect(() => {
    if (isAudioTest) invoke('start_audio_test');
    else invoke('stop_audio_test');

    return () => {
      invoke('stop_audio_test');
    };
  }, [isAudioTest]);

  // Device Init
  useEffect(() => {
    // Getting Default Devices (based on FS)
    invoke('get_current_devices').then((returnData) => {
      console.log("Returned Devices:", returnData);
      
      let data = returnData as string[];
      if (data.length > 0) setCurrentInputDevice(data[0] || ''); // Default to empty string if undefined
      if (data.length > 1) setCurrentOutputDevice(data[1] || ''); // Default to empty string if undefined
    });

    // Getting all current input devices
    invoke('get_input_devices').then((data) => {
      console.log("Input Devices:", data);
      setInputDevices(data as string[]);
    });

    invoke('get_output_devices').then((data) => {
      console.log("Output Devices:", data);
      setOutputDevices(data as string[]);
    });
  }, []);

  // Logs whenever CurrentInputDevice or CurrentOutputDevice changes
  useEffect(() => {
    console.log("Updated Input/Output Devices", CurrentInputDevice, CurrentOutputDevice);
  }, [CurrentInputDevice, CurrentOutputDevice]);

  function onDeviceChanged(value: string, isInputDevice: boolean) {
    console.log("Changing device to:", value, "isInputDevice:", isInputDevice);
    if (isInputDevice) {
      //Input Device
      setCurrentInputDevice(value);
      invoke('change_current_input_device', {inputDevice: value});
      console.log("Changing Input Device.");
    }
    else {
      //Output Device
      setCurrentOutputDevice(value);
      invoke('change_current_output_device', {outputDevice: value});
      console.log("Changing Output Device.");
    }
  }

  return (
    <>
      {/** Input & Output Selection */}
      <div style={{ width: "100%", height: "auto", display: "flex", marginBottom: "12px", borderBottom: "2px solid var(--Outlines)", gap: "4%", color: "var(--Text)" }}>
        <div style={{ width: "48%", height: "auto", display: "flex", flexDirection: "column" }}>
          <Typography sx={{ fontSize: "24px", width: "100%", textAlign: "center" }}>Input Device</Typography>
          <Select
            defaultValue='LOADING'
            value={CurrentInputDevice} // No need for || "" since it's already initialized as an empty string
            onChange={(e) => { onDeviceChanged(e.target.value, true); }}
          >
            {InputDevices.map((Device: string) => {
              return <MenuItem key={Device} value={Device}>{Device}</MenuItem>;
            })}
          </Select>
          <Slider />
        </div>

        <div style={{ width: "48%", height: "auto", display: "flex", flexDirection: "column" }}>
          <Typography sx={{ fontSize: "24px", width: "100%", textAlign: "center" }}>Output Device</Typography>
          <Select
            defaultValue='LOADING'
            value={CurrentOutputDevice} // Same here, no need for || ""
            onChange={(e) => { onDeviceChanged(e.target.value, false); }}
          >
            {OutputDevices.map((Device: string) => {
              return <MenuItem key={Device} value={Device}>{Device}</MenuItem>;
            })}
          </Select>
          <Slider />
        </div>
      </div>

      <div style={{ width: "100%", height: "auto", display: "flex" }}>
        <Button onClick={() => { setAudioTest(!isAudioTest); }} sx={{ border: "1px solid var(--Outlines)" }}>
          {isAudioTest ? "Stop Checking" : "Let's Check"}
        </Button>
      </div>
    </>
  );
};

export default Voice;
