import React, { useEffect, useState } from 'react'
import { Box, Button, ButtonProps, Typography, styled } from '@mui/material';
import { getTauriVersion } from '@tauri-apps/api/app';

import { getDisplayName } from './FunctionLibrary';

//TEMP ICONS UNTIL FUNCTIONALITY IS COMPLETED
import MicIcon from '@mui/icons-material/Mic';
import HeadsetOffIcon from '@mui/icons-material/HeadsetOff';
import SettingsIcon from '@mui/icons-material/Settings';
import Settings from './Settings';

export const IconButton = styled(Button)<ButtonProps> (({ theme }) => ({
    marginTop: "16px", width: "auto", aspectRatio: '1', minWidth: "16px",
    height: "auto", borderRadius: "8px", backgroundColor: 'var(--Interactable)',
    transition: '0.25s ease-in-out border', color: "var(--Text)", marginLeft: "auto",
    '*': { borderRadius: '8px' }, marginBottom: 'auto', marginTop: 'auto', marginRight: '0px',
    border: "1px solid var(--Outlines)",
}));

const UserInformation = () => {
    const [versionNumber, setVersion] = useState('0.0.0');
    const [settingsOpen, setSettingsOpen] = useState(false);

    //Init
    useEffect(() => {
        let getVersion = async () => {
            let version = await getTauriVersion();
            setVersion(version)
        }

        getVersion();
    }, [])

    return (
        <>
            <Box sx={{width: "100%", height: "100%", display: 'flex', flexDirection: 'row'}}>
                <Box sx={{width: "60%", height: 'calc(100% - 8px)', display: 'flex', flexDirection: 'column',
                    alignItems: 'left', padding: "4px", paddingLeft: '8px',
                    verticalAlign: 'center', justifyContent: 'center',
                }}>
                    <Typography>
                        {getDisplayName().length > 15 ? getDisplayName().slice(0, -3) + '...'
                        : getDisplayName()}
                    </Typography>
                    <Typography fontSize={'0.6em'}>Discord v{versionNumber}</Typography>
                </Box>
                <Box sx={{
                    width: "40%", height: "100%", marginLeft: "2px", display: 'flex', flexDirection: 'row',
                    gap: "4px", marginRight: "4px"
                }}>
                    {/*
                    <IconButton> <MicIcon/> </IconButton>
                    <IconButton> <HeadsetOffIcon/> </IconButton>
                    */}
                    <IconButton onClick={() => {setSettingsOpen(true);}} sx={{
                        "&:hover .MuiSvgIcon-root": {
                            animation: "spin 1.25s ease-in-out",
                        },
                        "@keyframes spin": {
                            "0%": {
                            transform: "rotate(0deg)",
                            },
                            "100%": {
                            transform: "rotate(360deg)",
                            },
                        },
                    }}> <SettingsIcon/> </IconButton>
                </Box>
            </Box>
            <Settings isModalOpen={settingsOpen} setModalOpen={setSettingsOpen} />
        </>
    )
}

export default UserInformation