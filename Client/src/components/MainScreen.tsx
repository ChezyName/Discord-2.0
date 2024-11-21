import React, { useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

const MainScreen = () => {

  return (
    <div>
      <button onClick={() => {invoke('start_audio_loop')}}>Start Audio Loop</button>
      <button onClick={() => {invoke('stop_audio_loop');}}>End Audio Loop</button>
    </div>
  )
}

export default MainScreen