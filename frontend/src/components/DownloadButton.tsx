import { FC } from 'react'
import { DownloadIcon } from './Icons'
import '../styles/components.css'

interface DownloadButtonProps {
  onClick: () => void
  disabled?: boolean
  loading?: boolean
}

const DownloadButton: FC<DownloadButtonProps> = ({ onClick, disabled, loading }) => {
  return (
    <button
      className={`download-button ${loading ? 'loading' : ''}`}
      onClick={onClick}
      disabled={disabled || loading}
    >
      {loading ? (
        <>
          <span className="spinner"></span> Downloading video...
        </>
      ) : (
        <>
          <DownloadIcon size={18} />
          Download Video
        </>
      )}
    </button>
  )
}

export default DownloadButton
