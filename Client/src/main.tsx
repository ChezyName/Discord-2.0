import React from "react";
import ReactDOM from "react-dom/client";
import MainScreen from "./components/MainScreen";
import './main.css'
import LoginScreen, { isLoggedIn } from "./components/LoginScreen";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <div style={{width: '100%', height: '100%', display: 'flex'}}>
      {
        isLoggedIn() ? <MainScreen /> : <LoginScreen/>
      }
    </div>
  </React.StrictMode>
);
