import { useEffect, useState } from 'react'
import { getServerData, getServerList } from './FunctionLibrary';

const SERVER_SEARCH_INTERVAL = 500;

export type ServerInformation = {
  serverIP: string;
  serverName: string;
  users: string[];
}

const SidePanel = ({setServerIP, setIsConnected ,setInitServerData}: any) => {
  const [myServers, setMyServers] = useState<ServerInformation[]>([]);

  useEffect(() => {
    //Get Init Server Data
    let doServerGetData = async (list: string[]) => {
      let myServerList: ServerInformation[] = [];
      for(let i = 0; i < list.length; i++){
        console.log("Getting Data for " + list[i])
        let data: ServerInformation = await getServerData(list[i]);
        myServerList.push(data);
      }

      setMyServers(myServerList);
    }

    let interval = setInterval(() => {doServerGetData(getServerList());}, SERVER_SEARCH_INTERVAL);

    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{width: '30%', height: '100%', backgroundColor: 'green'}}>
        {
          myServers.length > 0 ? (myServers.map((item) => {
            console.log("Adding: ", item)
            return (<button style={{width: '100%', height: "25px"}} onClick={() => {
              console.log("Connecting to " + item?.serverName + " @ " + item?.serverIP)
              if(setServerIP) setServerIP(item?.serverIP);
              if(setIsConnected) setIsConnected(true);
              if(setInitServerData) setInitServerData(item);
            }}>{item?.serverName}</button>)
          })) : ""
        }
    </div>
  )
}

export default SidePanel