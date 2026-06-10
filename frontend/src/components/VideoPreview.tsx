import { FC } from 'react'
import { VideoInfo } from '../types'

interface VideoPreviewProps {
  info: VideoInfo | null
  loading: boolean
  error?: boolean
}

function formatDuration(s: number): string {
  const h = Math.floor(s / 3600)
  const m = Math.floor((s % 3600) / 60)
  const sec = s % 60
  if (h > 0) return `${h}:${String(m).padStart(2,'0')}:${String(sec).padStart(2,'0')}`
  return `${m}:${String(sec).padStart(2,'0')}`
}

function formatSize(bytes: number): string {
  if (bytes >= 1e9) return `${(bytes/1e9).toFixed(1)} GB`
  if (bytes >= 1e6) return `${(bytes/1e6).toFixed(1)} MB`
  return `${Math.round(bytes/1e3)} KB`
}

const VideoPreview: FC<VideoPreviewProps> = ({ info, loading, error }) => {
  if (loading) return (
    <div className="video-preview" style={{ marginTop: '0.8rem' }}>
      <div style={{ width: 80, height: 52, borderRadius: 7, background: 'var(--hist-icon-bg)', flexShrink: 0 }} />
      <div className="video-preview-info">
        <div style={{ height: 12, width: '60%', background: 'var(--hist-icon-bg)', borderRadius: 4, marginBottom: 6 }} />
        <div style={{ height: 10, width: '40%', background: 'var(--hist-icon-bg)', borderRadius: 4 }} />
      </div>
    </div>
  )

  if (!info) {
    if (error) return (
      <div className="error-message" style={{ marginTop: '0.8rem' }}>Could not fetch video info — you can still download.</div>
    )
    return null
  }

  const meta = [
    info.uploader,
    info.duration_seconds != null ? formatDuration(info.duration_seconds) : null,
    info.filesize_approx != null ? formatSize(info.filesize_approx) : null,
    info.platform,
  ].filter(Boolean).join(' · ')

  return (
    <div className="video-preview">
      {info.thumbnail_url && (
        <img className="video-preview-thumb" src={info.thumbnail_url} alt="" loading="lazy"
          onError={(e) => { (e.target as HTMLImageElement).style.display = 'none' }} />
      )}
      <div className="video-preview-info">
        <div className="video-preview-title">{info.title}</div>
        <div className="video-preview-meta">{meta}</div>
      </div>
    </div>
  )
}

export default VideoPreview
