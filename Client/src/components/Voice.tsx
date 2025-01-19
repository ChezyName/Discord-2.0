import { Button, MenuItem, Select, Slider, Typography } from '@mui/material';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { readTextFile, writeTextFile, exists, BaseDirectory } from '@tauri-apps/plugin-fs';

function ApplyAudioChanged() {
  invoke('on_audio_settings_changed')
}

//Loads Devices into Memory (LocalStorage) for 'Faster' Load Times
export async function InitDevices() {
  //Place them in LocalStorage
    let currentDevice = await invoke('get_current_devices') as string[];
    let inputs = await invoke('get_input_devices') as string[];
    let outputs = await invoke('get_output_devices') as string[];

    //Get Slider Inputs
    let volume = {
      input: 100,
      output: 100,
    }

    const volumeSettingsExists = await exists('audio-volume.conf', {
      baseDir: BaseDirectory.AppLocalData,
    });

    if(volumeSettingsExists) {
      let vData =  await readTextFile('audio-volume.conf', {
        baseDir: BaseDirectory.AppLocalData,
      });
      volume = JSON.parse(vData);
      console.log("Read Audio-V.config with Data: ", volume, "  vs  ", vData)
    } else {
      await writeTextFile('audio-volume.conf', JSON.stringify(volume), {
        baseDir: BaseDirectory.AppLocalData,
      });
    }

    //save to local storage
    let json = {
      InputDevices: inputs,
      OutputDevices: outputs,
      InputDevice: currentDevice[0],
      OutputDevice: currentDevice[1],
      InputVolume: volume.input,
      OutputVolume: volume.output,
    }

    console.log("Loaded Audio Config:", json)

    localStorage.setItem("AudioDevices", JSON.stringify(json));
    return json;
}

const Voice = () => {
  const [isAudioTest, setAudioTest] = useState(false);
  
  const [InputDevices, setInputDevices] = useState<string[]>([]);
  const [OutputDevices, setOutputDevices] = useState<string[]>([]);

  const [CurrentInputDevice, setCurrentInputDevice] = useState<string>('');
  const [CurrentOutputDevice, setCurrentOutputDevice] = useState<string>('');

  const [InputVolume, setInputVolume] = useState(0);
  const [OutputVolume, setOutputVolume] = useState(0);

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
    /*
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
    */
    
    let init_and_load = async () => {
      let data = await InitDevices();
      setInputDevices(data.InputDevices)
      setOutputDevices(data.OutputDevices)
      setCurrentInputDevice(data.InputDevice)
      setCurrentOutputDevice(data.OutputDevice)
      setInputVolume(data.InputVolume)
      setOutputVolume(data.OutputVolume)
    }

    let jsonData = localStorage.getItem('AudioDevices');
    if(jsonData) {
      let data = JSON.parse(jsonData)
      setInputDevices(data.InputDevices)
      setOutputDevices(data.OutputDevices)
      setCurrentInputDevice(data.InputDevice)
      setCurrentOutputDevice(data.OutputDevice)
      setInputVolume(data.InputVolume)
      setOutputVolume(data.OutputVolume)
    }


    init_and_load();
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

    ApplyAudioChanged();
  }

  async function onVolumeChanged(value: number, isInputDevice: boolean) {
    let volume = {
      input: InputVolume,
      output: OutputVolume,
    }

    if(isInputDevice) {
      setInputVolume(value)
      volume.input = value;
    }
    else {
      setOutputVolume(value)
      volume.output = value;
    }

    console.log("Changing Audio Volume")

    //Write to file & Local Storage
    let currentData:any = JSON.parse(localStorage.getItem("AudioDevices") || '')
    currentData['InputVolume'] = volume.input;
    currentData['OutputVolume'] = volume.output;
    localStorage.setItem("AudioDevices", JSON.stringify(currentData))

    await writeTextFile('audio-volume.conf', JSON.stringify(volume), {
      baseDir: BaseDirectory.AppLocalData,
    });
    console.log("Wrote: ", JSON.stringify(volume), " to audio-volume.conf")
    ApplyAudioChanged();
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
          <Slider aria-label="Small" valueLabelDisplay="auto"
            min={0} max={100} value={InputVolume}
            onChange={(e,n) => {if(typeof n === 'number') onVolumeChanged(n, true)}}
          />
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
          <Slider aria-label="Small" valueLabelDisplay="auto"
            min={0} max={200} value={OutputVolume}
            onChange={(e,n) => {if(typeof n === 'number') onVolumeChanged(n, false)}}
          />
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
