import { FC } from 'react'

interface ProgressBarProps {
  progress: number
  status?: string | null
  speed?: string | null
  eta?: string | null
}

const ProgressBar: FC<ProgressBarProps> = ({ progress }) => (
  <div className="progress-bar-wrap">
    <div className="progress-bar-track">
      <div className="progress-bar-fill" style={{ width: `${Math.max(2, progress)}%` }} />
    </div>
  </div>
)

export default ProgressBar
