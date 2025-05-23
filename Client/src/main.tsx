import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import { invoke } from '@tauri-apps/api/core'
import MainScreen from "./components/MainScreen";
import LoginScreen, { isLoggedIn as CheckIsLoggedIn } from "./components/LoginScreen";

import './main.css'

import dark from "./dark.module.css?inline";
import light from "./light.module.css?inline";

import { exists, BaseDirectory, readTextFile } from '@tauri-apps/plugin-fs';
import { InitDevices } from "./components/Voice";

const Main = () => {
  const [isLoggedIn, setIsLoggedIn] = useState(CheckIsLoggedIn());

  let onLoginChanged = () => { setIsLoggedIn(CheckIsLoggedIn()); }

  useEffect(onLoginChanged, []);

  //Theme Selection
  useEffect(() => {
    //Auto load dark if not found
    if(localStorage.getItem('theme') == null || localStorage.getItem('theme') == undefined) {
      localStorage.setItem('theme','dark');
    }

    let theme = localStorage.getItem('theme')

    const styleElementCustom = document.createElement('style');
    document.head.append(styleElementCustom);

    if(theme == 'dark' || theme == 'light') {
      styleElementCustom.textContent = theme == 'dark' ? dark : light;
    }

    if(theme && theme !== 'dark' && theme !== 'light') {
      console.log("Opening Custom File @: ", theme)
      //Load Custom from Folder
      let loadThemeFile = async () => {
        if(theme == null) {
          localStorage.setItem('theme','dark');
          window.location.reload();
          return;
        }

        const exsists = await exists(theme, {
          baseDir: BaseDirectory.AppLocalData,
        });

        //Read file and append
        if(exsists){
          const fileContents = await readTextFile(theme, {
            baseDir: BaseDirectory.AppLocalData,
          });
          
          console.log("Loaded CSS File:", fileContents)
          styleElementCustom.textContent = fileContents;
        }
      }

      loadThemeFile();
    }

    //Make sure Voice Data is Loaded
    InitDevices();

    return () => {
      document.head.removeChild(styleElementCustom);
    }
  }, [])

  useEffect(() => {
    if(isLoggedIn) {
      //Tell Rust our Username
      invoke('set_username', {username: localStorage.getItem("displayName")});
    }
  }, [isLoggedIn])

  return (
    <div style={{width: '100%', height: '100%', display: 'flex'}}>
      {
        isLoggedIn ? <MainScreen /> : <LoginScreen setLoginChanged={onLoginChanged}/>
      }
  </div>
  )
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode> <Main/> </React.StrictMode>
);