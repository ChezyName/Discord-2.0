import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import MainScreen from "./components/MainScreen";
import LoginScreen, { isLoggedIn as CheckIsLoggedIn } from "./components/LoginScreen";

import './main.css'

import dark from './dark.css'
import light from './light.css'
import { BaseDirectory } from "@tauri-apps/plugin-fs";

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
    let themeHref = './dark.css'

    if(theme == 'light') themeHref = './light.css'


    let relative = window.__TAURI__ ? BaseDirectory.AppLocalData : '/src/';

    //Apply Theme
    const styleElement = document.createElement('link');
    styleElement.href = relative + (theme || '') + '.css';
    styleElement.rel = 'stylesheet'
    styleElement.type = 'text/css'
    document.head.append(styleElement);

    console.log(styleElement);

    return () => {
      document.head.removeChild(styleElement);
    }
  }, [])

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