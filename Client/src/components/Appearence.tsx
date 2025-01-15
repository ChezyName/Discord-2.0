import { useEffect, useState } from 'react'
import { Box, Typography, FormControl, InputLabel, Select, MenuItem } from '@mui/material'
import { open } from '@tauri-apps/plugin-dialog';
import { readFile, writeFile, BaseDirectory, readDir, DirEntry } from '@tauri-apps/plugin-fs';

const Appearence = () => {
    const [customThemes, setCustomThemes] = useState<string[]>([]);
    useEffect(() => {
        if(localStorage.getItem('theme') == null || localStorage.getItem('theme') == undefined) {
            localStorage.setItem('theme','dark');
        }
        
        localStorage.setItem('themeReload', 'false');
    }, [])

    async function changeTheme(theme:string|null) {
        if(theme == null) return;
        localStorage.setItem('themeReload', 'true');
        
        //Selecting Theme
        console.log('[NEW THEME]', theme)
        if(theme == 'dark' || theme == 'light'){
            localStorage.setItem('theme', theme);
        }
        else if(theme == 'custom') {
            //Custom Theme That File Must Be Loaded
            const fileLocation = await open({
                multiple: false,
                directory: false,
                filters: [{
                    name: 'Theme File (.css)',
                    extensions: ['css']
                }]
            });

            console.log("[THEME] Loading File:", fileLocation)
            if(fileLocation !== null && fileLocation !== undefined) {
                let url = fileLocation.replace(/^.*[\\/]/, '')
                if(url !== undefined) {
                    const data = await readFile(fileLocation);
                    console.log("File Contents: " + data);

                    await writeFile(url, data, {
                        baseDir: BaseDirectory.AppLocalData,
                    });

                    console.log("File Saved: " + url)

                    //Select File as Current Theme
                    localStorage.setItem('theme', url);
                }
            }
        }
        else {
            //Custom Theme Here
            localStorage.setItem('theme', theme);
        }

        window.location.reload();
    }

    async function InitCustomThemes() {
        const entries: DirEntry[] = await readDir('', { baseDir: BaseDirectory.AppLocalData });
        console.log("Files in AppLocalData: ", entries);

        let files = [];
        for(let i = 0; i < entries.length; i++) {
            const file = entries[i];
            if(file.isFile == true && file.name.includes('.css')) {
                files.push(file.name);
            }
        }

        //Set Themes Files Found
        console.log("All CSS Files in AppLocalData:", files);
        setCustomThemes(files);
    }

    useEffect(() => { InitCustomThemes(); }, [])

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
                        {
                            customThemes.map((item:string) => {
                                return <MenuItem value={item}>{item.replace('.css','')}</MenuItem>
                            })
                        }
                        <MenuItem value={'custom'}>{"<Custom CSS File>"}</MenuItem>
                    </Select>
                </FormControl>
            </Box>
        </Box>
    )
}

export default Appearence