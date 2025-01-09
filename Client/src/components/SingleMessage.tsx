import React from 'react'
import Typography from '@mui/material/Typography'
import { getDisplayName } from './FunctionLibrary'

const SingleMessage = ({message, displayName, myName}: any) => {
  return (
    <div style={{width: "calc(100% - 24px)", height: "auto", paddingLeft: "12px", paddingRight: "12px"}}>
        <Typography style={{fontSize: "20px", fontWeight: "bold"}}>
            {displayName}
        </Typography>

        <Typography style={{fontSize: "16px"}}>
          {message}
        </Typography>
    </div>
  )
}

export default SingleMessage