export type Platform = 
  | 'Instagram' 
  | 'TikTok' 
  | 'YouTube' 
  | 'Twitter' 
  | 'Facebook' 
  | 'Unknown'

export interface DownloadRequest {
  url: string
}

export interface VideoMetadata {
  title: string
  duration_seconds: number
  author: string
  video_url: string
  audio_url?: string
  thumbnail_url: string
  original_platform: string
  file_size_bytes?: number
}

export interface ErrorResponse {
  error: string
  message: string
  retry_after?: number
}
