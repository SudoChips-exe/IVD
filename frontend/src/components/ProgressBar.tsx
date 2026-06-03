import { FC } from 'react'
import '../styles/components.css'

interface ProgressBarProps {
  progress: number
  status?: string | null
  speed?: string | null
  eta?: string | null
}

const ProgressBar: FC<ProgressBarProps> = ({ progress, status, speed, eta }) => {
  const label = status ?? (progress === 0 ? 'Starting...' : `Downloading... ${progress}%`)

  return (
    <div className="progress-container">
      <div className="progress-info">
        <span className="progress-status">{label}</span>
        <div className="progress-meta">
          {speed && <span className="progress-speed">{speed}</span>}
          {eta && <span className="progress-eta">ETA {eta}</span>}
          {!status && <span className="progress-percent">{progress}%</span>}
        </div>
      </div>
      <div className="progress-bar">
        <div className="progress-fill" style={{ width: `${Math.max(2, progress)}%` }} />
      </div>
    </div>
  )
}

export default ProgressBar
