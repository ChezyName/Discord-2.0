import React from 'react'
import Typography from '@mui/material/Typography'

const SingleMessage = ({message}: any) => {
  return (
    <div style={{width: "calc(100% - 24px)", height: "auto", paddingLeft: "12px", paddingRight: "12px"}}>
        <Typography style={{fontSize: "20px", fontWeight: "bold"}}>
            Username
        </Typography>

        <Typography style={{fontSize: "16px"}}>
            That's why I need a one dance
            Got a Hennessy in my hand
            One more time 'fore I go
            Higher powers taking a hold on me
            I need a one dance
            Got a Hennessy in my hand
            One more time 'fore I go
            Higher powers taking a hold on me
        </Typography>
    </div>
  )
}

export default SingleMessage