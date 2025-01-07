import { useEffect, useRef } from 'react'
import { io, Socket } from 'socket.io-client'
import DefaultEventsMap from 'socket.io-client'
import { getMessageGatewayFromAddress } from './FunctionLibrary';

const Messages = ({isConnected, serverIP}: any) => {
  let socket = io('http://localhost:3001');

  socket.on("debug", (data) => {
    console.log("Socket Returned: ", data);
  })

  return (
    <div>

    </div>
  )
}

export default Messages