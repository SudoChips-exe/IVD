import { useState, useCallback, useRef } from 'react'
import { api } from '../services/api'
import { VideoInfo } from '../types'
import { saveToHistory } from '../utils/history'

export interface QueueItem {
  id: string
  url: string
  quality: string
  audioOnly: boolean
  info?: VideoInfo
  state: 'downloading' | 'done' | 'error' | 'cancelled'
  progress: number
  speed: string | null
  eta: string | null
  status: string | null
  error: string | null
  filename?: string
}

export const useDownloadQueue = () => {
  const [items, setItems] = useState<QueueItem[]>([])
  const esRefs = useRef<Map<string, EventSource>>(new Map())
  const jobIdRefs = useRef<Map<string, string>>(new Map())

  const updateItem = useCallback((id: string, patch: Partial<QueueItem>) => {
    setItems(prev => prev.map(item => item.id === id ? { ...item, ...patch } : item))
  }, [])

  const addDownload = useCallback(async (
    url: string,
    quality: string,
    audioOnly: boolean,
    info?: VideoInfo,
  ) => {
    const id = crypto.randomUUID()
    setItems(prev => [{
      id, url, quality, audioOnly, info,
      state: 'downloading',
      progress: 0,
      speed: null,
      eta: null,
      status: 'Starting...',
      error: null,
    }, ...prev])

    try {
      const { job_id } = await api.startDownload(
        url,
        quality === 'best' ? undefined : quality,
        audioOnly,
      )
      jobIdRefs.current.set(id, job_id)

      await new Promise<void>((resolve, reject) => {
        const es = new EventSource(api.getProgressUrl(job_id))
        esRefs.current.set(id, es)

        es.onmessage = (e) => {
          try {
            const event = JSON.parse(e.data)
            switch (event.type) {
              case 'progress':
                updateItem(id, {
                  progress: Math.round(event.percent),
                  speed: event.speed ?? null,
                  eta: event.eta ?? null,
                  status: null,
                })
                break
              case 'authenticating':
                updateItem(id, { status: `Authenticating with ${event.method}...` })
                break
              case 'merging':
                updateItem(id, { progress: 95, status: 'Merging video and audio...' })
                break
              case 'done':
                es.close()
                esRefs.current.delete(id)
                resolve()
                break
              case 'cancelled':
                es.close()
                esRefs.current.delete(id)
                updateItem(id, { state: 'cancelled', status: null })
                jobIdRefs.current.delete(id)
                resolve()
                return
              case 'error':
                es.close()
                esRefs.current.delete(id)
                reject(new Error(event.message))
                break
            }
          } catch {
            // ignore parse errors
          }
        }

        es.onerror = () => {
          if (esRefs.current.has(id)) {
            es.close()
            esRefs.current.delete(id)
            reject(new Error('Connection lost. Please try again.'))
          }
        }
      })

      // Bail if cancelled during SSE
      if (!jobIdRefs.current.has(id)) return

      updateItem(id, { progress: 98, status: 'Fetching file...' })
      const response = await api.downloadFile(job_id)

      const contentDisposition = response.headers['content-disposition']
      let filename = audioOnly ? 'audio.mp3' : 'video.mp4'
      if (contentDisposition) {
        const match = contentDisposition.match(/filename="?([^"]+)"?/)
        if (match) filename = match[1]
      }

      const blob = response.data as Blob
      if (!blob || blob.size === 0) throw new Error('Downloaded file is empty.')

      const objectUrl = window.URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = objectUrl
      link.download = filename
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      window.URL.revokeObjectURL(objectUrl)

      jobIdRefs.current.delete(id)
      updateItem(id, { state: 'done', progress: 100, status: null, filename })

      if (info) {
        saveToHistory({
          url,
          title: info.title,
          thumbnail: info.thumbnail_url,
          platform: info.platform ?? 'Unknown',
          quality,
          audioOnly,
        })
      }
    } catch (err: unknown) {
      jobIdRefs.current.delete(id)
      esRefs.current.delete(id)
      const msg = err instanceof Error ? err.message : 'Download failed.'
      updateItem(id, { state: 'error', error: msg, status: null })
    }
  }, [updateItem])

  const cancelItem = useCallback(async (id: string) => {
    esRefs.current.get(id)?.close()
    esRefs.current.delete(id)
    const jobId = jobIdRefs.current.get(id)
    if (jobId) {
      api.cancelDownload(jobId).catch(() => {})
      jobIdRefs.current.delete(id)
    }
    updateItem(id, { state: 'cancelled', status: null })
  }, [updateItem])

  const removeItem = useCallback((id: string) => {
    setItems(prev => prev.filter(i => i.id !== id))
  }, [])

  return { items, addDownload, cancelItem, removeItem }
}
