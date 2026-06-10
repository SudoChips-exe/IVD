import { FC } from 'react'
import { QueueItem } from '../hooks/useDownloadQueue'

interface Props {
  item: QueueItem
  onCancel: () => void
  onRemove: () => void
}

const VideoIcon = () => (
  <svg viewBox="0 0 24 24" fill="currentColor" width="13" height="13">
    <path d="M15 10l4.553-2.277A1 1 0 0 1 21 8.618v6.764a1 1 0 0 1-1.447.894L15 14M3 8a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8z"/>
  </svg>
)

const DownloadQueueItem: FC<Props> = ({ item, onCancel, onRemove }) => {
  const { info, state, progress, speed, eta, status, error, audioOnly, quality } = item

  const statusEl = () => {
    if (state === 'downloading') return <span className="dl-status status-progress">{status ?? `${progress}%`}</span>
    if (state === 'done') return <span className="dl-status status-done">Done</span>
    if (state === 'error') return <span className="dl-status status-error">Failed</span>
    if (state === 'cancelled') return <span className="dl-status status-cancelled">Cancelled</span>
    return null
  }

  const meta = [
    audioOnly ? 'MP3' : quality.toUpperCase(),
    info?.platform,
    speed && state === 'downloading' ? speed : null,
    eta && state === 'downloading' ? `ETA ${eta}` : null,
  ].filter(Boolean).join(' · ')

  return (
    <div className="dl-item">
      {state === 'downloading' && (
        <div className="dl-progress-bg" style={{ width: `${progress}%` }} />
      )}
      <div className="dl-top">
        {info?.thumbnail_url ? (
          <img
            className="history-thumb"
            src={info.thumbnail_url}
            alt=""
            style={{ width: 42, height: 28, borderRadius: 6, objectFit: 'cover', flexShrink: 0 }}
            onError={(e) => { (e.target as HTMLImageElement).style.display = 'none' }}
          />
        ) : (
          <div className="dl-thumb" style={{ color: 'var(--hist-icon-fill)' }}>
            <VideoIcon />
          </div>
        )}
        <div className="dl-info">
          <div className="dl-title">{info?.title ?? item.url}</div>
          <div className="dl-meta">{meta}</div>
        </div>
        <div className="dl-actions">
          {statusEl()}
          {state === 'downloading' && (
            <button className="dl-cancel-btn" onClick={onCancel} type="button">✕</button>
          )}
          {state !== 'downloading' && (
            <button className="dl-remove-btn" onClick={onRemove} type="button" aria-label="Remove">✕</button>
          )}
        </div>
      </div>
      {state === 'downloading' && (
        <div className="dl-bar-track">
          <div
            className={`dl-bar-fill${progress < 99 ? ' dl-bar-pulse' : ''}`}
            style={{ width: `${progress}%` }}
          />
        </div>
      )}
      {state === 'error' && error && (
        <div className="error-message" style={{ marginTop: '0.5rem' }}>{error}</div>
      )}
    </div>
  )
}

export default DownloadQueueItem
