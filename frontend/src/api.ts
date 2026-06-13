// VIDCLAW API layer.
// Talks to the backend that Vite proxies from /api to http://localhost:8080.
// Every call is wrapped by the caller so the UI can fall back to demo data
// when no backend is running (for example during design review).
import axios from 'axios';
import type { Quality, VideoInfo } from './types';

const client = axios.create({ baseURL: '/api', timeout: 20000 });

// Map a loose backend payload onto the shape the UI renders.
function normalizeInfo(data: Record<string, unknown>, url: string): VideoInfo {
  const get = (k: string) => (typeof data[k] === 'string' ? (data[k] as string) : '');
  let source = get('source') || get('extractor');
  if (!source) {
    try { source = new URL(url).hostname.replace(/^www\./, ''); } catch { source = ''; }
  }
  return {
    id: get('id') || String(Date.now()),
    title: get('title') || 'Untitled video',
    thumbnail: get('thumbnail') || get('thumbnail_url'),
    duration: get('duration') || get('duration_string'),
    author: get('author') || get('uploader') || get('channel'),
    meta: get('meta') || get('view_count_label') || '',
    source,
  };
}

export async function fetchVideoInfo(url: string): Promise<VideoInfo> {
  const { data } = await client.get('/info', { params: { url } });
  return normalizeInfo(data as Record<string, unknown>, url);
}

export async function startDownload(url: string, quality: Quality): Promise<{ id: string }> {
  const { data } = await client.post('/download', { url, quality });
  return data as { id: string };
}

export async function cancelDownload(id: string): Promise<void> {
  await client.post(`/download/${id}/cancel`);
}

export async function uploadCookies(file: File): Promise<void> {
  const form = new FormData();
  form.append('cookies', file);
  await client.post('/cookies', form, {
    headers: { 'Content-Type': 'multipart/form-data' },
  });
}
