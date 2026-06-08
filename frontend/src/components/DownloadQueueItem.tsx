import { FC } from 'react'
import { QueueItem } from '../hooks/useDownloadQueue'
import ProgressBar from './ProgressBar'

interface Props {
  item: QueueItem
  onCancel: () => void
  onRemove: () => void
}

const DownloadQueueItem: FC<Props> = ({ item, onCancel, onRemove }) => {
  const { info, state, progress, speed, eta, status, error, audioOnly, quality } = item

  return (
    <div className={`queue-item queue-item--${state}`}>
      <div className="queue-item-header">
        {info?.thumbnail_url && (
          <img
            className="queue-thumb"
            src={info.thumbnail_url}
            alt=""
            onError={(e) => { (e.target as HTMLImageElement).style.display = 'none' }}
          />
        )}
        <div className="queue-info">
          <p className="queue-title">{info?.title ?? item.url}</p>
          <div className="queue-meta">
            {info?.uploader && <span className="queue-uploader">{info.uploader}</span>}
            <span className="queue-badge">{audioOnly ? 'MP3' : quality.toUpperCase()}</span>
            {state === 'done' && <span className="queue-state-badge queue-state-badge--done">Done</span>}
            {state === 'error' && <span className="queue-state-badge queue-state-badge--error">Failed</span>}
            {state === 'cancelled' && <span className="queue-state-badge queue-state-badge--cancelled">Cancelled</span>}
          </div>
        </div>
        <div className="queue-actions" style={{ display: 'flex', gap: '0.35rem', alignItems: 'center' }}>
          {state === 'downloading' && (
            <button className="queue-cancel-btn" onClick={onCancel} type="button">
              Cancel
            </button>
          )}
          {state === 'done' && (
            <button
              className="queue-remove-btn"
              type="button"
              title="Copy source URL"
              onClick={() => navigator.clipboard.writeText(item.url).catch(() => {})}
              aria-label="Copy URL"
            >
              ⎘
            </button>
          )}
          {state !== 'downloading' && (
            <button className="queue-remove-btn" onClick={onRemove} type="button" aria-label="Remove">
              ✕
            </button>
          )}
        </div>
      </div>

      {state === 'downloading' && (
        <ProgressBar progress={progress} status={status} speed={speed} eta={eta} />
      )}

      {state === 'error' && error && (
        <p className="queue-error-msg">{error}</p>
      )}
    </div>
  )
}

export default DownloadQueueItem
