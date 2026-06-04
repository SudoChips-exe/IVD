import { FC } from 'react'
import { VideoInfo } from '../types'

interface VideoPreviewProps {
  info: VideoInfo | null
  loading: boolean
  error?: boolean
}

const VideoPreview: FC<VideoPreviewProps> = ({ info, loading, error }) => {
  if (loading) {
    return (
      <div className="video-preview video-preview--loading">
        <div className="preview-thumb-skeleton" />
        <div className="preview-body">
          <div className="preview-skeleton preview-skeleton--title" />
          <div className="preview-skeleton preview-skeleton--meta" />
          <span className="preview-loading-label">Fetching info...</span>
        </div>
      </div>
    )
  }

  if (!info) {
    if (error) {
      return (
        <div className="video-preview video-preview--error">
          <span className="preview-error-label">Could not fetch video info — you can still download.</span>
        </div>
      )
    }
    return null
  }

  return (
    <div className="video-preview">
      {info.thumbnail_url && (
        <img
          className="preview-thumb"
          src={info.thumbnail_url}
          alt=""
          loading="lazy"
          onError={(e) => { (e.target as HTMLImageElement).style.display = 'none' }}
        />
      )}
      <div className="preview-body">
        <p className="preview-title">{info.title}</p>
        <div className="preview-meta">
          <span className="preview-uploader">{info.uploader}</span>
          {info.duration_seconds != null && (
            <span className="preview-stat">{formatDuration(info.duration_seconds)}</span>
          )}
          {info.filesize_approx != null && (
            <span className="preview-stat">{formatSize(info.filesize_approx)}</span>
          )}
          <span className={`platform-badge ${info.platform.toLowerCase()}`}>
            {info.platform}
          </span>
        </div>
      </div>
    </div>
  )
}

function formatDuration(s: number): string {
  const h = Math.floor(s / 3600)
  const m = Math.floor((s % 3600) / 60)
  const sec = s % 60
  if (h > 0) return `${h}:${String(m).padStart(2, '0')}:${String(sec).padStart(2, '0')}`
  return `${m}:${String(sec).padStart(2, '0')}`
}

function formatSize(bytes: number): string {
  if (bytes >= 1e9) return `${(bytes / 1e9).toFixed(1)} GB`
  if (bytes >= 1e6) return `${(bytes / 1e6).toFixed(1)} MB`
  return `${Math.round(bytes / 1e3)} KB`
}

export default VideoPreview
