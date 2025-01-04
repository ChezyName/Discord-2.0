import { useEffect, useState } from 'react'
import { Button, TextField, Modal, Box, Typography } from "@mui/material"
import { addServerToList, getServerData } from './FunctionLibrary';

import AddIcon from '@mui/icons-material/Add';


const LoadingThrobber = () => {
  return(
    <object style={{width: "auto", height: "100%"}} data="/LoadingBars.svg" type="image/svg+xml">
      <img src="/LoadingBars.svg" />
    </object>
  )
}

const ServerMenu = ({setSearch}:any) => {
  const [isModalOpen,setModalOpen] = useState(false);
  const [newServerIP, setServerIP] = useState("");
  const [serverCheck, setServerCheck] = useState(false);

  async function addServer() {
    setServerCheck(true);

    //Fetch Server Data
    let serverData = await getServerData(newServerIP);
    console.log("Adding to ServerList: ", serverData);

    if(serverData == null){
      setServerCheck(false);
    }
    else {
      //Add the server, close the modal
      addServerToList(newServerIP);

      setServerCheck(false);
      setModalOpen(false);
      setServerIP('');
    }
  }

  return (
    <>
      <div style={{backgroundColor: "transparent", width: "calc(100% - 4px)", height: "5%", marginBottom: "2%", minHeight: "40px",
        display: 'flex', flexDirection: "row", margin: "4px"}}>

          {/** Open Modal Window for adding Server via URL / IP */}
          <Button sx={{aspectRatio: "1", borderRadius: "4px", border: "1px solid black"}} onClick={() => {setModalOpen(true);}}><AddIcon/></Button>
          <TextField InputLabelProps={{shrink: true, style: {display: 'none'}}} sx={{marginRight: "2%", marginLeft: "2%",
              '& legend': { display: 'none' }, '& fieldset': { top: 0 }, width: "auto", display: 'flex', flexGrow: 1,
              '& .MuiInputBase-input': {height: 'calc(1.4375em/2)'},
            }}
            InputProps={{
              sx: {
                height: '100%',
                alignItems: 'start'
              }
            }}
            id="server-search" label="Search" type="search" variant='outlined' placeholder='Search Server'
            onChange={(event) => {
              if(setSearch) setSearch(event.target.value);
            }}/>
      </div>

      {/** Modal Window to add Server */}
      <Modal open={isModalOpen} onClose={() => {setModalOpen(false);}}
        aria-labelledby="modal-modal-title"
        aria-describedby="modal-modal-description">
        <Box sx={{display: 'flex', flexDirection: "column",
          backgroundColor: 'yellow', width: "auto", height: "auto",
          position: "absolute", left: "50%", top: "50%", transform: 'translate(-50%, -50%)',
          border: "4px solid blue", padding: "2%"
        }}>
          <Typography id="modal-modal-title" variant="h6" component="h2">
            Add New Server
          </Typography>

          <Typography id="modal-modal-description" sx={{ mt: 2, whiteSpace: "pre-wrap" }}>
            Please enter a Server IP Address like 'discord.com' or '127.0.0.1:3001'.{"\n"}
            <strong><i>port is needed</i></strong>
          </Typography>

          <TextField onChange={(event) => {setServerIP(event.target.value);}} 
            onKeyUp={(e) => {if(e.key == "Enter"){addServer();}}}
            label='Server Address' type='text' variant='standard'/>
          <Button sx={{marginTop: "2%"}} onClick={() => {addServer();}}>{serverCheck ? <LoadingThrobber /> : 'Add Server'}</Button>
        </Box>
      </Modal>
    </>
  )
}

export default ServerMenu