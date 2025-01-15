import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import SidePanel, { ServerInformation } from "./SidePanel";
import { getServerData } from './FunctionLibrary';
import Messages from './Messages';
import { Button, Typography } from '@mui/material';

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

      <div style={{backgroundColor: 'var(--Background)', color: '#FFF', width: '100%', borderLeft: '5px solid var(--Outlines)',
        display: 'flex', flexDirection: 'column'}}>

        <div style={{width: '100%', height: '60px', borderBottom: '5px solid var(--Outlines)',
          display: (isConnected ? 'flex' : 'none'), alignItems: 'center', justifyContent: 'center'}}>
          <Typography fontWeight='bold' color='var(--Text)' sx={{marginLeft: "8px", height: '100%',
            display: 'flex', alignItems: "center", justifyContent: 'center',
          }} variant='h4'>{serverData?.serverName}</Typography>
          { isConnected ? 
            <Button onClick={() => {
              setIsConnected(false)
              setServerData(null);
              setServerIP('');
              setServerName('');
            }} 
            sx={{width: "auto",
                height: "80%", borderRadius: "8px", backgroundColor: 'var(--Interactable)',
                transition: '0.25s ease-in-out border', color: "var(--Text)", marginLeft: "auto",
                '*': { borderRadius: '8px' }, marginBottom: 'auto', marginTop: 'auto', marginRight: '8px',
                border: "1px solid var(--Outlines)",
            }}>Disconnect</Button>
          : "" }
        </div>

        <div style={{ width: "100%", height: (isConnected ? 'calc(100% - 65px)' : '100%')}}>
          {/** Allow for Commands when NOT Connected to Server - Minecraft Style?*/}
          <Messages isConnected={isConnected} serverIP={serverIP} serverName={serverName}/>
        </div>

      </div>
    </div>
  )
}

export default MainScreen