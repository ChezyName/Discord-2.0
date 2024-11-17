import React from 'react'
import { invoke } from '@tauri-apps/api/core'

const MainScreen = () => {
  invoke('start_audio_loop');

  return (
    <div></div>
  )
}

export default MainScreen