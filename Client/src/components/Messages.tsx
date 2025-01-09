import { useEffect, useRef } from 'react'
import { io, Socket } from 'socket.io-client'
import DefaultEventsMap from 'socket.io-client'
import { getDisplayName, getMessageGatewayFromAddress } from './FunctionLibrary';

const Messages = ({isConnected, serverIP}: any) => {
  let socket = useRef(io('http://localhost:3001', {autoConnect: false}));

  useEffect(() => {
    if(isConnected) {
      if(socket.current && socket.current.connected) socket.current.disconnect();

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
        })
      
        socket.current.on("message", (newMesasge) => {
          //load the new Message
        })
      }

      initSocket();
    }
    else {
      //disconnect socket
      if(socket.current && socket.current.connected) socket.current.disconnect();
      console.log("[MSG] Disconnecting Server - IsConnected?:" + socket.current.connected)
    }

    return () => {
      if(socket.current) {
        socket.current.disconnect();
        console.log("[MSG] Socket Disconnected by useEffect Return Statement.")
      }
    }
  }, [isConnected])

  return (
    <div>

    </div>
  )
}

export default Messages