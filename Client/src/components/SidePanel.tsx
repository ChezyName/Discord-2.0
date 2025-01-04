import { useEffect, useState } from 'react'
import { getServerData, getServerList } from './FunctionLibrary';
import ServerMenu from './ServerMenu';

const SERVER_SEARCH_INTERVAL = 500;

export type ServerInformation = {
  serverIP: string;
  serverName: string;
  users: string[];
}

const SidePanel = ({setServerIP, setIsConnected ,setInitServerData}: any) => {
  const [myServers, setMyServers] = useState<ServerInformation[]>([]);
  const [search, setSearch] = useState("");

  useEffect(() => {
    //Get Init Server Data
    let doServerGetData = async (list: string[]) => {
      let myServerList: ServerInformation[] = [];
      for(let i = 0; i < list.length; i++){
        console.log("Getting Data for " + list[i])
        let data: ServerInformation|null = await getServerData(list[i]);
        if(data !== null) myServerList.push(data);
      }

      setMyServers(myServerList);
    }

    let interval = setInterval(async () => {let d = await getServerList(); doServerGetData(d);}, SERVER_SEARCH_INTERVAL);

    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{width: '30%', height: '100%', backgroundColor: 'green', minWidth: "calc(480px * 0.3)"}}>
      <ServerMenu setSearch={setSearch}/>
        {
          myServers.length > 0 ? (myServers.map((item) => {
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