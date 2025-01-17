import { Box } from "@mui/material"

//Icons
import GitHubIcon from '@mui/icons-material/GitHub';
import YouTubeIcon from '@mui/icons-material/YouTube';
import TwitterIcon from '@mui/icons-material/Twitter';

import { IconButton } from "./UserInformation";
import { openInNewTab } from "./FunctionLibrary";

const Credits = {
    DisplayName: 'ChezyName',
    GitHub: 'https://github.com/ChezyName/Discord-2.0',
    YouTube: 'https://www.youtube.com/@chezyname',
    Twitter: 'https://x.com/ChezyName',
}

const CopyrightNotice = () => {
  return (
    <div style={{display: 'flex', flexDirection: 'column', width: "100%", height: "calc(100% - 24px)"}}>
        <Box sx={{marginTop: "8px", fontSize: "16px", textAlign: 'left'}}>
            Discord 2 is an independent project and is not affiliated, associated, authorized, endorsed by,
            or in any way officially connected with Discord Inc.
            The name 'Discord' and its logo are trademarks of Discord Inc.
        </Box>

        <Box sx={{marginTop: "8px", fontSize: "16px", textAlign: '', whiteSpace: 'pre-wrap'}}>
        Copyright 2024-2025 {Credits.DisplayName}.{'\n'}
        Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation
        files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify,
        merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished
        to do so, subject to the following conditions:
        {'\n\n'}
        The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
        {'\n\n'}
        THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
        INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
        PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
        DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH
        THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
        </Box>

        <Box sx={{background: "", width: "100%", height: "auto", marginTop: "auto",
            display: "flex", flexDirection: 'row', alignItems: "center", marginBottom: "8px", gap: '8px',
            justifyContent: 'center'
        }}>
            {Credits.GitHub ? <IconButton onClick={()=>{openInNewTab(Credits.GitHub)}} sx={{margin: '0'}}><GitHubIcon/></IconButton> : ""}
            {Credits.YouTube ? <IconButton onClick={()=>{openInNewTab(Credits.YouTube)}} sx={{margin: '0'}}><YouTubeIcon/></IconButton> : ""}
            {Credits.Twitter ? <IconButton onClick={()=>{openInNewTab(Credits.Twitter)}} sx={{margin: '0'}}><TwitterIcon/></IconButton> : ""}
        </Box>
    </div>
  )
}

export default CopyrightNotice