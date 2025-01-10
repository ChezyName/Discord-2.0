import { useEffect, useState } from 'react'
import { getServerData, getServerList, removeFromServerList } from './FunctionLibrary';
import ServerMenu from './ServerMenu';
import { Button, Typography } from '@mui/material';
import DeleteOutlineIcon from '@mui/icons-material/DeleteOutline';

const SERVER_SEARCH_INTERVAL = 500;

export type ServerInformation = {
  serverIP: string;
  serverName: string;
  users: string[];
}

const SidePanel = ({setServerIP, setIsConnected ,setInitServerData, setServerName}: any) => {
  const [myServers, setMyServers] = useState<ServerInformation[]>([]);
  const [search, setSearch] = useState("");

  useEffect(() => {
    //Get Init Server Data
    let doServerGetData = async (list: string[]) => {
      let myServerList: ServerInformation[] = [];
      for(let i = 0; i < list.length; i++){
        console.log(list[i]);
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
    <div style={{width: '30%', height: '100%', backgroundColor: 'green', minWidth: "240px"}}>
      <ServerMenu setSearch={setSearch}/>
        {
          myServers.length > 0 ? (myServers.map((item) => {
            if(item.serverName.toLowerCase().includes(search.toLowerCase()) || item.serverIP.toLowerCase().includes(search.toLowerCase())) {
              return (<button className='serverJoinButton' style={{width: 'calc(100% - 10px)', height: "4.5em", marginBottom: "8px",
                marginLeft: "5px", marginRight: "5px", borderRadius: "4px", border: "2px solid black", display: 'flex', flexDirection: "column"}} onClick={() => {
                console.log("Connecting to " + item?.serverName + " @ " + item?.serverIP)
                if(setServerIP) setServerIP(item?.serverIP);
                if(setServerName) setServerName(item?.serverName);
                if(setIsConnected) setIsConnected(true);
                if(setInitServerData) setInitServerData(item);
              }}>
                <Typography sx={{textAlign: "left", fontSize: "1.5em", fontWeight: "bold", display: 'flex'}}> {item?.serverName}</Typography>
                <Typography sx={{textAlign: "left", fontSize: "1em", fontStyle: "italic", width: "100%", display: 'flex'}}>
                  {item?.serverIP}
                  <Button onClick={(e) => {e.stopPropagation(); removeFromServerList(item?.serverIP)}} sx={{marginLeft: 'auto', aspectRatio: "1", height: "1em", width: "1em"}}><DeleteOutlineIcon/></Button>
                </Typography>
              </button>)
            } else return "";
          })) : ""
        }
    </div>
  )
}

export default SidePanel