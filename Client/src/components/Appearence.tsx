import { useEffect, useState } from 'react'
import { Box, Typography, FormControl, InputLabel, Select, MenuItem } from '@mui/material'

const Appearence = () => {
    useEffect(() => {
        if(localStorage.getItem('theme') == null || localStorage.getItem('theme') == undefined) {
            localStorage.setItem('theme','dark');
        }
        
        localStorage.setItem('themeReload', 'false');
     }, [])

    function changeTheme(theme:string|null) {
        if(theme == null) return;
        localStorage.setItem('themeReload', 'true');
        
        //Selecting Theme
        console.log('[NEW THEME]', theme)
        if(theme == 'dark' || theme == 'light'){
            localStorage.setItem('theme', theme);
        }
        else if(theme == 'custom') {
            //Custom Theme That File Must Be Loaded
        }
        else {
            //Custom Theme Here
        }

        window.location.reload();
    }

    return (
        <Box sx={{width: "100%", height: "auto", backgroundColor: "",
            display: 'flex', flexDirection: 'column', gap: '8px', marginTop: "8px"
        }}>
            <Box sx={{width: "100%", display: "flex"}}>
                <Typography variant='h5'>Theme</Typography>
                <FormControl sx={{width: "60%", marginLeft: "auto",
                    '*': {
                        color: "var(--Text)"
                    }
                }}>
                    <InputLabel id="demo-simple-select-label">Age</InputLabel>
                    <Select
                        labelId="demo-simple-select-label"
                        id="demo-simple-select"
                        value={localStorage.getItem('theme')}
                        label="Theme"
                        onChange={(e) => {changeTheme(e.target.value)}}
                        sx={{
                            color: "var(--Text)",
                            '.MuiOutlinedInput-notchedOutline': {
                              borderColor: 'var(--Text)',
                            },
                            '&.Mui-focused .MuiOutlinedInput-notchedOutline': {
                              borderColor: 'var(--Text)',
                            },
                            '&:hover .MuiOutlinedInput-notchedOutline': {
                              borderColor: 'var(--Text)',
                            },
                            '.MuiSvgIcon-root ': {
                              fill: "var(--Text) !important",
                            }
                        }}
                        MenuProps={{
                            sx: {
                                "&& .MuiList-root": {
                                    background: "var(--Interactable)",
                                    color: 'var(--Text)',
                                },
                                "&& .MuiButtonBase-root:hover": {
                                    background: "var(--Primary)",
                                    filter: 'alpha(opacity=5)',
                                },
                                "&& .Mui-selected": {
                                    backgroundColor: "var(--Primary)",
                                    color: 'var(--Text)',
                                }
                            }
                        }}
                    >
                        <MenuItem value={'dark'}>Dark</MenuItem>
                        <MenuItem value={'light'}>Light</MenuItem>
                        <MenuItem value={'custom'}>{"<Custom CSS File>"}</MenuItem>
                    </Select>
                </FormControl>
            </Box>
        </Box>
    )
}

export default Appearence