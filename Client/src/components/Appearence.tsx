import { useState } from 'react'
import { Box, Typography, FormControl, InputLabel, Select, MenuItem } from '@mui/material'

const Appearence = () => {
    const [theme, setTheme] = useState('dark');

    function changeTheme(theme:string) {
        //Selecting Theme
        console.log('[NEW THEME]', theme)
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
                        value={theme}
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