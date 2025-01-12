import { Modal, Box, Tabs, Tab as TabNative, TabsProps, styled, Button, Typography, TypographyProps } from '@mui/material'
import React, { useState } from 'react'

let Tab = styled(TabNative)<TabsProps> (({ theme }) => ({ 
    color: 'var(--Text)',
}));

let Title = styled(Typography)<TypographyProps> (({ theme }) => ({
    fontSize: '32px', fontWeight: 'bold', width: '100%',
    color: 'var(--Text)', borderBottom: '2px solid var(--Outlines)'
}));

interface TabPanelProps {
    children?: React.ReactNode;
    index: number;
    value: number;
}

function TabPanel(props: TabPanelProps) {
    const { children, value, index, ...other } = props;
  
    return (
      <div
        role="tabpanel"
        hidden={value !== index}
        id={`vertical-tabpanel-${index}`}
        aria-labelledby={`vertical-tab-${index}`}
        {...other}
        style={{width: "100%", height: "100%"}}
      >
        {value === index && (
          <Box sx={{ p: 3 }}>
            {children}
          </Box>
        )}
      </div>
    );
  }

const Settings = () => {
    const [currentTab, setTab] = useState(0);

    return (
        <Box sx={{
            display: 'flex', flexDirection: 'row', width: "calc(100% - 10px)",
            height: 'calc(100% - 10px)',
        }}>
            <Tabs
                orientation="vertical"
                variant="scrollable"
                value={currentTab}
                onChange={(e,v) => {setTab(v)}}
                aria-label="Vertical tabs example"
                sx={{ borderRight: 1, borderColor: 'divider',
                    "& .MuiTabs-flexContainer": {
                        width: '100%', height: '100%'
                    }
                }}
            >
                <Tab label="Appearence"/>
                <Tab label="Voice & Video"/>
                <Tab label="Copyright & Credits"/>
                
                <Button
                    sx={{
                        width: "100%", padding: "16px",
                        height: "24px", borderRadius: "0px",
                        color: "var(--Text)", marginTop: "auto",
                        '*': { borderRadius: '0px' },
                        border: "0px solid var(--Outlines)",
                    }}
                    onClick={() => {
                        localStorage.setItem('displayName','');
                        window.location.reload();
                    }}
                >
                    Log out
                </Button>
            </Tabs>
            <TabPanel value={currentTab} index={0}>
                <Title>Appearence</Title>
            </TabPanel>
            <TabPanel value={currentTab} index={1}>
                <Title>Voice & Video</Title>
            </TabPanel>
            <TabPanel value={currentTab} index={2}>
                <Title>Copyright & Credits</Title>
            </TabPanel>
        </Box>
    )
}

const SettingsParent = ({isModalOpen, setModalOpen}:any) => {
  return (
    <Modal open={isModalOpen} onClose={() => {if(setModalOpen) setModalOpen(false);}}
    aria-labelledby="modal-modal-title"
    aria-describedby="modal-modal-description">
        <Box sx={{display: 'flex', flexDirection: "column",
        backgroundColor: 'black', width: "80%", height: "80%",
        position: "absolute", left: "50%", top: "50%", transform: 'translate(-50%, -50%)',
        border: "4px solid var(--Outlines)", padding: "0", color: 'var(--Text)', borderRadius: '8px',
        opacity: '1',
        }}>
            {/** Settings Window UI Here */}
            <Box sx={{display: 'flex', flexDirection: "column",
                backgroundColor: 'var(--Primary)', width: "calc(100% - 24px)", height: "calc(100% - 24px)",
                padding: "12px"
            }}>
                {/** Settings Window UI Here */}
                <Settings />
            </Box>
        </Box>
    </Modal>
  )
}

export default SettingsParent