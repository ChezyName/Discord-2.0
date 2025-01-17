import { useEffect, useState } from 'react'
import { getServerData, getServerList, removeFromServerList } from './FunctionLibrary';
import ServerMenu from './ServerMenu';
import { Box, Button, Typography } from '@mui/material';
import DeleteOutlineIcon from '@mui/icons-material/DeleteOutline';
import UserInformation from './UserInformation';

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

    /* DEBUG
    let tempServers: ServerInformation[] = []
    for(let i = 0; i < 250; i++){
      let IP = ("" + i + "." + (i + 1) + '.' + i + 3);
      tempServers.push({
        serverIP: IP,
        serverName: IP,
        users: []
      })
    }
    setMyServers(tempServers);
    return;
    */

    //Get Init Server Data
    let doServerGetData = async (list: string[]) => {
      let myServerList: ServerInformation[] = [];
      for(let i = 0; i < list.length; i++){
        //console.log(list[i]);
        //console.log("Getting Data for " + list[i])
        let data: ServerInformation|null = await getServerData(list[i]);
        if(data !== null) myServerList.push(data);
      }

      setMyServers(myServerList);
    }

    let interval = setInterval(async () => {let d = await getServerList(); doServerGetData(d);}, SERVER_SEARCH_INTERVAL);

    return () => clearInterval(interval);
  }, []);

  return (
    <div style={{width: '30%', height: '100%', backgroundColor: 'var(--Background)', minWidth: "240px"}}>
      <ServerMenu setSearch={setSearch}/>
      <Box sx={{overflowY: 'auto', color: 'var(--Text)', backgroundColor: 'var(--Background)', marginTop: "0px", height: 'calc(100% - 66px - 82px)'}}>
        {
          myServers.length > 0 ? (myServers.map((item) => {
            if(item.serverName.toLowerCase().includes(search.toLowerCase()) || item.serverIP.toLowerCase().includes(search.toLowerCase())) {
              
              return (<Button className='serverJoinButton'
              sx={{width: 'calc(100% - 10px)', height: "4.5em", marginBottom: "8px",
                marginLeft: "5px", marginRight: "5px", borderRadius: "4px", 
                border: "2px solid var(--Outlines)", display: 'flex', flexDirection: "column",
                backgroundColor: 'var(--Interactable)', alignItems: 'left',
              }}
              
              onClick={() => {
                console.log("Connecting to " + item?.serverName + " @ " + item?.serverIP)
                if(setServerIP) setServerIP(item?.serverIP);
                if(setServerName) setServerName(item?.serverName);
                if(setIsConnected) setIsConnected(true);
                if(setInitServerData) setInitServerData(item);
              }}>
                <Typography color='var(--Text)' sx={{textAlign: "left", fontSize: "1.5em", fontWeight: "bold", display: 'flex', width: "100%"}}> {item?.serverName}</Typography>
                <Typography color='var(--Text)'sx={{textAlign: "left", fontSize: "1em", fontStyle: "italic", width: "100%", display: 'flex'}}>
                  {item?.serverIP}
                  <Button onClick={(e) => {e.stopPropagation(); removeFromServerList(item?.serverIP)}} sx={{marginLeft: 'auto', aspectRatio: "1", height: "1em", width: "1em"}}><DeleteOutlineIcon/></Button>
                </Typography>
              </Button>)

            } else return "";
          })) : ""
        }
      </Box>
      <Box sx={{
        width: "100%", height: "80px", backgroundColor: 'var(--Secondary)',
        borderTop: '2px solid var(--Outlines)', color: 'var(--Text)',
      }}> <UserInformation/> </Box>
    </div>
  )
}

export default SidePanel