import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import SidePanel, { ServerInformation } from "./SidePanel";
import { getServerData } from './FunctionLibrary';
import Messages from './Messages';
import { Typography } from '@mui/material';

const MainScreen = () => {
  const [isConnected, setIsConnected] = useState(false);
  const [serverIP, setServerIP] = useState('0.0.0.0');
  const [serverName, setServerName] = useState('');
  const [serverData, setServerData] = useState<ServerInformation | null>();


  useEffect(() => {
    const interval = setInterval(async () => {
      if(!isConnected) return;
      let data = await getServerData(serverIP);
      setServerData(data);

      console.log(data);
    }, 1500);

    return () => clearInterval(interval);
  }, [isConnected])

  useEffect(() => {
    console.log(serverData);
  }, [serverData]);

  useEffect(() => {
    invoke('set_server_ip', {server_ip: serverIP});
    if(isConnected) invoke('start_audio_loop');
    else invoke('stop_audio_loop');
  }, [isConnected])

  return (
    <div style={{width: '100%', height: '100%', display: 'flex', flexDirection: 'row'}}>
      <SidePanel setServerName={setServerName} setIsConnected={setIsConnected}
        setServerIP={setServerIP} setInitServerData={setServerData}
      />

      <div style={{backgroundColor: '#222', color: '#FFF', width: '100%', borderLeft: '5px solid black',
        display: 'flex', flexDirection: 'column'}}>

        <div style={{width: '100%', height: '60px', borderBottom: '5px solid black',
          display: 'flex', alignItems: 'center', justifyContent: 'center'}}>
          <Typography sx={{marginLeft: "8px"}} variant='h4'>{serverName}</Typography>
          { isConnected ? <button onClick={() => {
            setIsConnected(false)
            setServerData(null);
            setServerIP('');
            setServerName('');
          }} style={{width: "auto", height: "100%", marginLeft: "auto"}}>Disconnect</button>
          : "" }
        </div>

        <div style={{marginTop: '0px', width: "100%", height: "calc(100% - 65px)"}}>
          <Messages isConnected={isConnected} serverIP={serverIP} serverName={serverName}/>
        </div>

      </div>
    </div>
  )
}

export default MainScreen