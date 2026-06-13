export type Platform =
  | 'Instagram'
  | 'TikTok'
  | 'YouTube'
  | 'Twitter'
  | 'Facebook'
  | 'Unknown'

export type Theme = 'dark' | 'light'

export type Quality = 'best' | '1080p' | '720p' | '480p' | '360p' | 'mp3'

export type DownloadStatus = 'queued' | 'downloading' | 'completed' | 'error'

export interface VideoInfo {
  id?: string
  title: string
  thumbnail: string
  thumbnail_url?: string
  duration?: string
  duration_seconds?: number
  author?: string
  uploader?: string
  meta?: string
  source?: string
  platform?: string
  filesize_approx?: number
  is_image?: boolean
}

export interface DownloadJob {
  id: string
  title: string
  thumbnail: string
  quality: Quality
  status: DownloadStatus
  progress: number
  detail: string
}

export interface QualityOption {
  id: Quality
  name: string
  sub: string
  tag?: string
}

export interface PlaylistEntry {
  url: string
  title: string
  thumbnail?: string
  duration_seconds?: number
}

export interface PlaylistInfo {
  title: string
  entries: PlaylistEntry[]
  total: number
}

export interface HistoryEntry {
  id: string
  url: string
  title: string
  thumbnail?: string
  platform: string
  quality: string
  audioOnly: boolean
  timestamp: number
}

export interface ErrorResponse {
  error: string
  message: string
  retry_after?: number
}
