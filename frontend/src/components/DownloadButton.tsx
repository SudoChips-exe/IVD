import { FC } from 'react'

interface DownloadButtonProps {
  onClick: () => void
  disabled?: boolean
  loading?: boolean
}

const DownloadButton: FC<DownloadButtonProps> = ({ onClick, disabled, loading }) => {
  return (
    <button
      className="btn-fetch"
      style={{ width: '100%', marginTop: '1rem', justifyContent: 'center', padding: '0.85rem' }}
      onClick={onClick}
      disabled={disabled || loading}
      type="button"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.92)" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" width="16" height="16">
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
        <polyline points="7 10 12 15 17 10"/>
        <line x1="12" y1="15" x2="12" y2="3"/>
      </svg>
      {loading ? 'Downloading…' : 'Download'}
    </button>
  )
}

export default DownloadButton
