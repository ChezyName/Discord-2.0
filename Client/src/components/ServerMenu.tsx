import { useState } from 'react'
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
      <div style={{width: "calc(100% - 4px)", height: "52px", marginBottom: "0%", minHeight: "40px",
        display: 'flex', flexDirection: "row", padding: "4px"}}>

          {/** Open Modal Window for adding Server via URL / IP */}
          <Button sx={{aspectRatio: "1", borderRadius: "4px",
            border: "1px solid var(--Outlines)", color: 'var(--Text)',
            backgroundColor: 'var(--Interactable)'
          }} onClick={() => {setModalOpen(true);}}><AddIcon/></Button>
          <TextField InputLabelProps={{shrink: true, style: {display: 'none'}}} sx={{marginRight: "2%", marginLeft: "2%",
              '& legend': { display: 'none' }, '& fieldset': { top: 0 }, width: "auto", display: 'flex', flexGrow: 1,
              '& .MuiInputBase-input': {height: 'calc(1.4375em/2)'},
              backgroundColor: 'var(--Interactable) !important', borderRadius: '8px', border: '2px solid var(--Outlines)',
            }}
            InputProps={{
              sx: {
                height: '100%',
                alignItems: 'start',
                color: "var(--Text)",
              }
            }}
            id="server-search" label="Search" type="search" variant='outlined' placeholder='Search Server'
            onChange={(event) => {
              if(setSearch) setSearch(event.target.value);
            }}/>
      </div>
      <div style={{borderBottom: "4px solid var(--Outlines)", width: "100%", marginLeft: 'auto', marginRight: "auto", marginBottom: "2px"}}/>

      {/** Modal Window to add Server */}
      <Modal open={isModalOpen} onClose={() => {setModalOpen(false);}}
        aria-labelledby="modal-modal-title"
        aria-describedby="modal-modal-description">
        <Box sx={{display: 'flex', flexDirection: "column",
          backgroundColor: 'var(--Primary)', width: "auto", height: "auto",
          position: "absolute", left: "50%", top: "50%", transform: 'translate(-50%, -50%)',
          border: "4px solid var(--Outlines)", padding: "2%", color: 'var(--Text)', borderRadius: '8px'
        }}>
          <Typography fontWeight={'bold'} id="modal-modal-title" variant="h4" component="h2">
            Add New Server
          </Typography>

          <Typography id="modal-modal-description" sx={{ mt: 2, whiteSpace: "pre-wrap" }}>
            Please enter a Server IP Address like 'discord.com' or 'localhost:3001'.{"\n"}
            <strong>DO NOT USE LOCALHOST, USE 127.0.0.1</strong>
            <strong><i>port is needed</i></strong>
          </Typography>

          <TextField onChange={(event) => {setServerIP(event.target.value);}} 
            onKeyUp={(e) => {if(e.key == "Enter"){addServer();}}}
            label='Server Address' type='text' variant='outlined'
            InputLabelProps={{shrink: true, style: {display: 'none'}}} sx={{marginRight: "2%", marginLeft: "0%",
            marginTop: '12px',
              '& legend': { display: 'none' }, '& fieldset': { top: 0 }, width: "100%", display: 'flex', flexGrow: 1,
              '& .MuiInputBase-input': {height: 'calc(1.4375em/2)'},
              backgroundColor: 'var(--Interactable) !important', borderRadius: '8px'
            }}
            InputProps={{
              sx: {
                height: '100%',
                alignItems: 'start',
                color: "white",
              }
            }}
            placeholder={'Server IP Address'}
          />
          <Button sx={{marginTop: "16px", width: "100%",
              height: "42px", borderRadius: "8px", backgroundColor: 'var(--Interactable)',
              transition: '0.25s ease-in-out border',
              '*': { borderRadius: '8px' },
            }}
            onClick={() => {addServer();}}>
              {serverCheck ? <LoadingThrobber /> : 'Add Server'}
          </Button>
        </Box>
      </Modal>
    </>
  )
}

export default ServerMenu