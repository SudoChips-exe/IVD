import { useState, useCallback, useRef } from 'react'
import { api } from '../services/api'

interface DownloadState {
  loading: boolean
  progress: number
  speed: string | null
  eta: string | null
  status: string | null
  error: string | null
  success: boolean
}

export const useDownload = () => {
  const [state, setState] = useState<DownloadState>({
    loading: false,
    progress: 0,
    speed: null,
    eta: null,
    status: null,
    error: null,
    success: false,
  })

  const esRef = useRef<EventSource | null>(null)
  const jobIdRef = useRef<string | null>(null)

  const cancel = useCallback(async () => {
    esRef.current?.close()
    esRef.current = null

    if (jobIdRef.current) {
      api.cancelDownload(jobIdRef.current).catch(() => {})
      jobIdRef.current = null
    }

    setState({ loading: false, progress: 0, speed: null, eta: null, status: null, error: null, success: false })
  }, [])

  const download = useCallback(async (url: string, quality?: string): Promise<void> => {
    if (!url.trim()) {
      setState(prev => ({ ...prev, error: 'Please enter a valid URL' }))
      return
    }

    setState({ loading: true, progress: 0, speed: null, eta: null, status: 'Starting...', error: null, success: false })

    try {
      const { job_id } = await api.startDownload(url.trim(), quality)
      jobIdRef.current = job_id

      await new Promise<void>((resolve, reject) => {
        const es = new EventSource(api.getProgressUrl(job_id))
        esRef.current = es

        es.onmessage = (e) => {
          try {
            const event = JSON.parse(e.data)

            switch (event.type) {
              case 'progress':
                setState(prev => ({
                  ...prev,
                  progress: Math.round(event.percent),
                  speed: event.speed ?? null,
                  eta: event.eta ?? null,
                  status: null,
                }))
                break
              case 'authenticating':
                setState(prev => ({ ...prev, progress: 0, status: `Authenticating with ${event.method}...` }))
                break
              case 'merging':
                setState(prev => ({ ...prev, progress: 95, status: 'Merging video and audio...' }))
                break
              case 'done':
                es.close()
                esRef.current = null
                resolve()
                break
              case 'cancelled':
                es.close()
                esRef.current = null
                resolve()
                return
              case 'error':
                es.close()
                esRef.current = null
                reject(new Error(event.message))
                break
            }
          } catch {
            // ignore parse errors
          }
        }

        es.onerror = () => {
          es.close()
          esRef.current = null
          reject(new Error('Connection lost. Please try again.'))
        }
      })

      // If cancelled during the SSE phase, state was already reset by cancel()
      if (!jobIdRef.current) return

      setState(prev => ({ ...prev, progress: 98, status: 'Fetching file...' }))
      const response = await api.downloadFile(job_id)

      const contentDisposition = response.headers['content-disposition']
      let filename = 'downloaded_video.mp4'
      if (contentDisposition) {
        const match = contentDisposition.match(/filename="(.+?)"/)
        if (match) filename = match[1]
      }

      const blob = response.data as Blob
      if (!blob || blob.size === 0) throw new Error('Downloaded file is empty')

      const downloadUrl = window.URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = downloadUrl
      link.download = filename
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      window.URL.revokeObjectURL(downloadUrl)

      jobIdRef.current = null
      setState({ loading: false, progress: 100, speed: null, eta: null, status: null, error: null, success: true })
      setTimeout(() => setState(prev => ({ ...prev, success: false })), 5000)
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Download failed. Please try again.'
      jobIdRef.current = null
      setState(prev => ({ ...prev, loading: false, error: message, status: null }))
    }
  }, [])

  return {
    download,
    cancel,
    loading: state.loading,
    progress: state.progress,
    speed: state.speed,
    eta: state.eta,
    status: state.status,
    error: state.error,
    success: state.success,
  }
}
