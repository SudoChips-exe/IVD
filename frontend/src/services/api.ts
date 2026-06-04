import axios, { AxiosInstance } from 'axios'
import { VideoInfo } from '../types'

const API_BASE_URL = import.meta.env.VITE_API_URL ?? 'http://localhost:8080'

const client: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  headers: { 'Content-Type': 'application/json' },
})

const blobClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  responseType: 'blob',
})

export const api = {
  async getVideoInfo(url: string): Promise<VideoInfo> {
    const res = await client.post('/api/info', { url })
    return res.data
  },

  async startDownload(url: string, quality?: string, audioOnly?: boolean): Promise<{ job_id: string }> {
    const res = await client.post('/api/download', {
      url,
      quality: quality ?? null,
      audio_only: audioOnly ?? false,
    })
    return res.data
  },

  getProgressUrl(jobId: string): string {
    return `${API_BASE_URL}/api/progress/${jobId}`
  },

  async downloadFile(jobId: string) {
    return blobClient.get(`/api/file/${jobId}`)
  },

  async cancelDownload(jobId: string) {
    return client.post(`/api/cancel/${jobId}`)
  },

  async uploadCookies(content: string) {
    return client.post('/api/cookies', content, {
      headers: { 'Content-Type': 'text/plain' },
    })
  },

  async deleteCookies() {
    return client.delete('/api/cookies')
  },

  async getCookiesStatus(): Promise<{ active: boolean }> {
    const res = await client.get('/api/cookies/status')
    return res.data
  },

  async checkHealth() {
    return client.get('/api/health')
  },
}
