import { Box, Button, TextField, Typography } from "@mui/material";
import { useState } from "react";

export function isLoggedIn() : boolean {
  return localStorage.getItem('displayName') !== null
  && localStorage.getItem('displayName') !== undefined
  && localStorage.getItem('displayName') !== "";
}

function Login(displayName: string) {
  if(displayName === "") return
  //Login
  //Browsers use LocalStorage, Standalone use Filesystem via FunctionLib
  console.log("Logingin as: " + displayName)
  localStorage.setItem('displayName', displayName);
}

const LoginScreen = ({setLoginChanged}:any) => {
  const [displayName, setDisplayName] = useState('');

  return (
    <Box sx={{width: '100vw', height: '100vh',
        display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center',
        minHeight: '100%',
        backgroundImage:
        'radial-gradient(at 50% 50%, var(--Primary), var(--Secondary))',
        backgroundRepeat: 'no-repeat', backgroundColor: 'var(--Background)',
    }}>
        
        <div style={{display: 'flex', flexDirection: 'column', alignItems: 'center',
          backgroundColor: "rgba(5, 7, 10, 0.4)", borderRadius: '8px', border: '2px solid var(--Outlines)', padding: "24px",
          paddingTop: "24px", paddingBottom: "24px", width: "40vw", height: 'auto', color: 'white'
        }}>
          {/* <Typography variant="h4" sx={{width: "100%", textAlign: "center"}}>Discord 2</Typography> */}
          
          <Typography color='var(--Text)' fontWeight={'bold'} variant="h4" sx={{marginBottom: "0px"}}>Welcome back!</Typography>
          <Typography color='var(--Text)' variant="h6" sx={{marginBottom: "42px"}}>We're so excited to see you again!</Typography>

          <TextField value={displayName} InputLabelProps={{shrink: true, style: {display: 'none'}}} sx={{marginRight: "2%", marginLeft: "2%",
                '& legend': { display: 'none' }, '& fieldset': { top: 0 }, width: "80%", display: 'flex', flexGrow: 1,
                '& .MuiInputBase-input': {height: 'calc(1.4375em/2)'},
                backgroundColor: 'var(--Interactable) !important', borderRadius: '8px'
              }}
              InputProps={{
                sx: {
                  height: '100%',
                  alignItems: 'start',
                  color: "var(--Text)",
                }
              }}
              id="message" label="Message" variant='outlined' placeholder={'Display Name'}
              onChange={(event) => {
                setDisplayName(event.target.value);
              }}
              onKeyUp={(e) => {
                if(e.key === 'Enter') {
                  Login(displayName);
                  setLoginChanged();
                }
              }}
            />

            <Button onClick={() => {Login(displayName); setLoginChanged();}} sx={{marginTop: "16px", width: "80%",
              height: "42px", borderRadius: "8px", backgroundColor: 'var(--Interactable)',
              transition: '0.25s ease-in-out border',
              '*': { borderRadius: '8px' },
            }}>
              <Typography color='var(--Text)' fontWeight={'bold'}>Login</Typography>
            </Button>

            <div style={{width: "80%", height: "100%", borderBottom: "1px solid var(--Text)", marginTop: "15px"}}></div>

            <Typography color='var(--Text)' width={'80%'} textAlign={'center'} marginTop={'8px'} fontSize={'8px'}>
            Discord 2 is an independent project and is not affiliated, associated, authorized,
            endorsed by, or in any way officially connected with Discord Inc.
            The name 'Discord' and its logo are trademarks of Discord Inc.

            This was for fun, please dont sue me Discord.
            </Typography>
        </div>
    </Box>
  )
}

export default LoginScreen