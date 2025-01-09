import { useEffect, useRef, useState } from 'react'
import { io } from 'socket.io-client'
import { getDisplayName, getMessageGatewayFromAddress } from './FunctionLibrary';
import SingleMessage from './SingleMessage';
import { TextField } from '@mui/material';

const Messages = ({isConnected, serverIP, serverName}: any) => {
  let socket = useRef(io('http://localhost:3001', {autoConnect: false}));
  const messageWindow = useRef<HTMLDivElement>(null);
  let [messages,setMessage] = useState([]);

  let onMessageLoad = () => {
    //Only scroll into view if this is the first load
    //or if User is already at the bottom
    console.log("[MSG] Scrolling Into View")
    messageWindow.current?.scrollIntoView({ behavior: "instant" });
  }

  useEffect(onMessageLoad,[messages]);

  useEffect(() => {
    if(isConnected) {
      if(socket.current && socket.current.connected) {
        socket.current.disconnect();
        setMessage([]);
      }

      console.log("[MSG] Joining Server @ " + getMessageGatewayFromAddress(serverIP).href)
      socket.current = io(getMessageGatewayFromAddress(serverIP).href, {autoConnect: false});
      socket.current.connect();

      let initSocket =async () => {
        let name = await getDisplayName();
        socket.current.emit("init", name);

        //==============================================================================
        // When the socket is loaded, create the return functions for all the data given
      
        socket.current.on("init", (initMessageData) => {
          //sort current messages
          initMessageData.sort((a:any,b:any) => {
            return a['TimeStamp'] - b['TimeStamp'];
          })

          //load the current messages
          console.log("[MSG] Loading Init Messages:", initMessageData);
          setMessage(initMessageData);
        })
      
        socket.current.on("message", (newMesasge) => {
          //load the new Message
        })
      }

      initSocket();
    }
    else {
      //disconnect socket
      if(socket.current && socket.current.connected) {
        socket.current.disconnect();
        setMessage([]);
      }
      console.log("[MSG] Disconnecting Server - IsConnected?:" + socket.current.connected);
    }

    return () => {
      if(socket.current) {
        socket.current.disconnect();
        setMessage([]);
        console.log("[MSG] Socket Disconnected by useEffect Return Statement.")
      }
    }
  }, [isConnected])

  return (
    <div style={{display: 'flex', flexDirection: 'column', width: "100%", height: "100%"}}>
      <div style={{backgroundColor: "transparent", width: "100%", height: "calc(100% - 65px)",
        overflowY: "auto", margin: "0", padding: "0", display: "flex", flexDirection: "column",
        gap: '16px'
      }}>
        {
          messages.map((msg: any) => {
            return <SingleMessage displayName={msg.user} message={msg.message}/>
          })
        }
        <div ref={messageWindow} />
      </div>

      <div style={{height: "40px", marginTop: "auto", paddingBottom: "12px"}}>
        <TextField InputLabelProps={{shrink: true, style: {display: 'none'}}} sx={{marginRight: "2%", marginLeft: "2%",
              '& legend': { display: 'none' }, '& fieldset': { top: 0 }, width: "auto", display: 'flex', flexGrow: 1,
              '& .MuiInputBase-input': {height: 'calc(1.4375em/2)'},
              backgroundColor: '#000 !important', borderRadius: '4px'
            }}
            InputProps={{
              sx: {
                height: '100%',
                alignItems: 'start',
                color: "white",
              }
            }}
            id="message" label="Search" type="text" variant='outlined' placeholder={'Message '+serverName}
            onChange={(event) => {
              
          }}/>
      </div>
    </div>
  )
}

export default Messages