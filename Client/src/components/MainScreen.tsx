import React, { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import SidePanel from "./SidePanel";

const MainScreen = () => {
  const [isConnected, setIsConnected] = useState(false);
  const [serverIP, setServerIP] = useState('0.0.0.0');
  const [serverData, setServerData] = useState({});

  useEffect(() => {
    invoke('set_server_ip', {server_ip: serverIP});
    if(isConnected) invoke('start_audio_loop');
    else invoke('stop_audio_loop');
  }, [isConnected])

  return (
    <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'row'}}>
      <SidePanel setIsConnected={setIsConnected} setServerIP={setServerIP}/>

      <div style={{backgroundColor: '#222', color: '#FFF', width: '100%', borderLeft: '5px solid black',
        display: 'flex', flexDirection: 'column'}}>
        <div style={{width: '100%', height: '5%', borderBottom: '1px solid black',
        display: 'flex', alignItems: 'center', justifyContent: 'center'}}>
          FLAMING MANGO
          { isConnected ? <button onClick={() => {setIsConnected(false)}} style={{width: "auto", height: "100%"}}>Disconnect</button>
          : "" }
        </div>

        MESSAGE WINDOW
      </div>
    </div>
  )
}

export default MainScreen