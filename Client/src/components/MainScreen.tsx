import React, { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import SidePanel from "./SidePanel";

const MainScreen = () => {
  const [isConnected, setIsConnected] = useState(false);
  const [serverIP, setServerIP] = useState('0.0.0.0');
  const [serverData, setServerData] = useState({});

  return (
    <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'row'}}>
      <SidePanel />
      <div style={{backgroundColor: '#222', color: '#FFF', width: '100%', borderLeft: '5px solid black',
        display: 'flex', flexDirection: 'column'}}>
        <div style={{width: '100%', height: '5%', borderBottom: '1px solid black',
        display: 'flex', alignItems: 'center', justifyContent: 'center'}}> FLAMING MANGO </div>

        MESSAGE WINDOW
      </div>
    </div>
  )
}

export default MainScreen