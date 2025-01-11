import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import MainScreen from "./components/MainScreen";
import './main.css'
import LoginScreen, { isLoggedIn as CheckIsLoggedIn } from "./components/LoginScreen";

const Main = () => {
  const [isLoggedIn, setIsLoggedIn] = useState(CheckIsLoggedIn());

  let onLoginChanged = () => { setIsLoggedIn(CheckIsLoggedIn()); }

  useEffect(onLoginChanged, []);

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