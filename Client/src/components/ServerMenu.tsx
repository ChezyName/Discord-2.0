import { useEffect, useState } from 'react'
import { Button, TextField, Modal, Box, Typography } from "@mui/material"

import AddIcon from '@mui/icons-material/Add';

const ServerMenu = ({setSearch}:any) => {
  const [isModalOpen,setModalOpen] = useState(false);

  return (
    <>
      <div style={{backgroundColor: "purple", width: "100%", height: "5%", marginBottom: "2%", minHeight: "40px",
        display: 'flex', flexDirection: "row"}}>

          {/** Open Modal Window for adding Server via URL / IP */}
          <Button sx={{aspectRatio: "1"}} onClick={() => {setModalOpen(true);}}><AddIcon/></Button>
          <TextField sx={{marginRight: "2%", marginLeft: "2%"}}
            id="server-search" label="Search" type="search" variant='standard'
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
          border: "4px solid blue", padding: "5px"
        }}>
          <Typography id="modal-modal-title" variant="h6" component="h2">
            Add New Server
          </Typography>
          <TextField label='Server Address' type='text' variant='standard'/>
          <Button>Add Server</Button>
        </Box>
      </Modal>
    </>
  )
}

export default ServerMenu