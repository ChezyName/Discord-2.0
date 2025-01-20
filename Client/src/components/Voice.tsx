import { Button, MenuItem, Select, Slider, Typography } from '@mui/material';
import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { readTextFile, writeTextFile, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
import { listen } from '@tauri-apps/api/event';
import { AudioVisualizer } from 'react-audio-visualize';

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

  const [audioBlob, setAudioBlob] = useState(new Blob([], {type: "audio/wav"}))
  const visualizerRef = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    setAudioTest(false);
    return () => { setAudioTest(false); };
  }, []);

  useEffect(() => {
    if (isAudioTest) {
      invoke('stop_audio_loop');
      invoke('start_audio_test');
    }
    else {
      invoke('start_audio_loop');
      invoke('stop_audio_test');
    }

    return () => {
      invoke('start_audio_loop');
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

  listen<any>('audio-sample', (event) => {
    //pcm data in 20ms @ 4800khz
    let blob = createWavBlob(event.payload as number[])
    console.log("Audio Debbuger: ", event.payload, typeof event.payload, blob);
    setAudioBlob(blob);
  });

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

        {audioBlob && (
          <AudioVisualizer
            ref={visualizerRef}
            blob={audioBlob}
            width={500}
            height={75}
            barWidth={1}
            gap={0}
            barColor={'#f76565'}
          />
        )}
      </div>
    </>
  );
};

export default Voice;

function createWavBlob(pcmData: number[], sampleRate = 48000, numChannels = 2) {
  const byteRate = sampleRate * numChannels * 2; // 16-bit audio (2 bytes per sample)
  const blockAlign = numChannels * 2;
  const wavHeaderSize = 44;

  // WAV Header
  const wavHeader = new ArrayBuffer(wavHeaderSize);
  const view = new DataView(wavHeader);

  // "RIFF" chunk descriptor
  view.setUint32(0, 0x52494646, false); // "RIFF"
  view.setUint32(4, 36 + pcmData.length * 2, true); // File size - 8 bytes
  view.setUint32(8, 0x57415645, false); // "WAVE"

  // "fmt " sub-chunk
  view.setUint32(12, 0x666d7420, false); // "fmt "
  view.setUint32(16, 16, true); // Subchunk1Size (16 for PCM)
  view.setUint16(20, 1, true); // AudioFormat (1 = PCM)
  view.setUint16(22, numChannels, true); // NumChannels
  view.setUint32(24, sampleRate, true); // SampleRate
  view.setUint32(28, byteRate, true); // ByteRate
  view.setUint16(32, blockAlign, true); // BlockAlign
  view.setUint16(34, 16, true); // BitsPerSample

  // "data" sub-chunk
  view.setUint32(36, 0x64617461, false); // "data"
  view.setUint32(40, pcmData.length * 2, true); // Subchunk2Size

  // PCM Data (convert from Float32Array to Int16Array)
  const pcm16 = new Int16Array(pcmData.length);
  for (let i = 0; i < pcmData.length; i++) {
      // Clamp PCM data to [-1, 1] and scale to 16-bit range
      pcm16[i] = Math.max(-1, Math.min(1, pcmData[i])) * 0x7FFF;
  }

  // Combine header and PCM data
  const wavData = new Uint8Array(wavHeaderSize + pcm16.length * 2);
  wavData.set(new Uint8Array(wavHeader), 0);
  wavData.set(new Uint8Array(pcm16.buffer), wavHeaderSize);

  // Create Blob
  return new Blob([wavData], { type: "audio/wav" });
}