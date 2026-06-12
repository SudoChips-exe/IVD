// VIDCLAW shared types

export type Quality = 'best' | '1080p' | '720p' | '480p' | '360p' | 'mp3';

export type DownloadStatus = 'queued' | 'downloading' | 'completed' | 'error';

export type Theme = 'dark' | 'light';

export interface Platform {
  id: string;
  label: string;
  color: string;
}

export interface QualityOption {
  id: Quality;
  name: string;
  sub: string;
  tag?: string;
}

export interface VideoInfo {
  id: string;
  title: string;
  thumbnail: string;
  duration: string;
  author: string;
  meta: string;
  source: string;
}

export interface DownloadJob {
  id: string;
  title: string;
  thumbnail: string;
  quality: Quality;
  status: DownloadStatus;
  progress: number;
  detail: string;
}
