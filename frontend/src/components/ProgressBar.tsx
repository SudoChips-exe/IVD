import React, { FC } from 'react'
import '../styles/components.css'

interface ProgressBarProps {
  progress: number
  status?: string
}

const ProgressBar: FC<ProgressBarProps> = ({ progress, status }) => {
  const getStatusText = () => {
    if (status) return status
    if (progress < 25) return 'Preparing...'
    if (progress < 75) return 'Processing...'
    if (progress < 100) return 'Finalizing...'
    return 'Complete!'
  }

  return (
    <div className="progress-container">
      <div className="progress-info">
        <span className="progress-status">{getStatusText()}</span>
        <span className="progress-percent">{progress}%</span>
      </div>
      <div className="progress-bar">
        <div 
          className="progress-fill" 
          style={{ width: `${progress}%` }}
        ></div>
      </div>
    </div>
  )
}

export default ProgressBar
