import { useEffect, useRef } from 'react'
import { io, Socket } from 'socket.io-client'
import DefaultEventsMap from 'socket.io-client'
import { getMessageGatewayFromAddress } from './FunctionLibrary';

const Messages = ({isConnected, serverIP}: any) => {
  let socket = useRef(io('http://localhost:3001', {autoConnect: false}));

  useEffect(() => {
    if(isConnected) {
      if(socket.current && socket.current.connected) socket.current.disconnect();

      console.log("[MSG] Joining Server @ " + getMessageGatewayFromAddress(serverIP).href)
      socket.current = io(getMessageGatewayFromAddress(serverIP).href, {autoConnect: false});
      socket.current.connect();
    }
    else {
      //disconnect socket
      if(socket.current && socket.current.connected) socket.current.disconnect();
      console.log("[MSG] Disconnecting Server - IsConnected?:" + socket.current.connected)
    }
  }, [isConnected])

  socket.current.on("debug", (data) => {
    console.log("Socket Returned: ", data);
  })

  return (
    <div>

    </div>
  )
}

export default Messages