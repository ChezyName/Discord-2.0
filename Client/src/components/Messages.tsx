import { useEffect, useRef, useState } from 'react'
import { io, Socket } from 'socket.io-client'
import DefaultEventsMap from 'socket.io-client'
import { getDisplayName, getMessageGatewayFromAddress } from './FunctionLibrary';
import SingleMessage from './SingleMessage';

const Messages = ({isConnected, serverIP}: any) => {
  let socket = useRef(io('http://localhost:3001', {autoConnect: false}));
  let [messages,setMessage] = useState([]);

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
    <div style={{backgroundColor: "transparent", width: "100%", height: "100%",
      overflowY: "auto", margin: "0", padding: "0", display: "flex", flexDirection: "column",
      gap: '16px'
    }}>
      {
        messages.map((msg) => {
          return <SingleMessage />
        })
      }
    </div>
  )
}

export default Messages