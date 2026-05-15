import React, { FC } from 'react'
import '../styles/components.css'

interface ProgressBarProps {
  progress: number
}

const ProgressBar: FC<ProgressBarProps> = ({ progress }) => {
  return (
    <div className="progress-container">
      <div className="progress-bar">
        <div 
          className="progress-fill" 
          style={{ width: `${progress}%` }}
        ></div>
      </div>
      <span className="progress-text">{progress}%</span>
    </div>
  )
}

export default ProgressBar
