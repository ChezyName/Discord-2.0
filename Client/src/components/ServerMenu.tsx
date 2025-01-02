import { useEffect, useState } from 'react'
import { Button } from "@mui/material"

import AddIcon from '@mui/icons-material/Add';

const ServerMenu = ({setSearch}:any) => {
  return (
    <div style={{backgroundColor: "purple", width: "100%", height: "5%", marginBottom: "2%", minHeight: "40px",
    display: 'flex', flexDirection: "row"}}>
        {/** Open Modal Window for adding Server via URL / IP */}
        <Button sx={{aspectRatio: "1"}}><AddIcon/></Button>
    </div>
  )
}

export default ServerMenu