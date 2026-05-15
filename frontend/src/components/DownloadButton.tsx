import React, { FC } from 'react'
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
          <span className="spinner"></span> Downloading...
        </>
      ) : (
        <>
          ⬇️ Download Video
        </>
      )}
    </button>
  )
}

export default DownloadButton
