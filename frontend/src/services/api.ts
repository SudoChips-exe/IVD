import axios, { AxiosInstance } from 'axios'

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080'

const client: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  responseType: 'blob',
})

export const api = {
  async downloadVideo(url: string) {
    return client.post('/api/download', { url })
  },

  async checkHealth() {
    return client.get('/api/health')
  },
}
