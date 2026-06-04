export type Platform =
  | 'Instagram'
  | 'TikTok'
  | 'YouTube'
  | 'Twitter'
  | 'Facebook'
  | 'Unknown'

export interface VideoInfo {
  title: string
  uploader: string
  duration_seconds?: number
  thumbnail_url?: string
  filesize_approx?: number
  platform: string
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
