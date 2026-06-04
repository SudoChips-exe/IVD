import { useState, useEffect, useRef } from 'react'
import { api } from '../services/api'
import { VideoInfo } from '../types'

export const useVideoInfo = (url: string) => {
  const [info, setInfo] = useState<VideoInfo | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState(false)
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null)

  useEffect(() => {
    if (timerRef.current) clearTimeout(timerRef.current)

    const trimmed = url.trim()
    if (!trimmed || !trimmed.startsWith('http')) {
      setInfo(null)
      setError(false)
      setLoading(false)
      return
    }

    setLoading(true)
    setError(false)
    setInfo(null)

    timerRef.current = setTimeout(async () => {
      try {
        const result = await api.getVideoInfo(trimmed)
        setInfo(result)
        setError(false)
      } catch {
        setInfo(null)
        setError(true)
      } finally {
        setLoading(false)
      }
    }, 900)

    return () => {
      if (timerRef.current) clearTimeout(timerRef.current)
    }
  }, [url])

  const clear = () => {
    if (timerRef.current) clearTimeout(timerRef.current)
    setInfo(null)
    setError(false)
    setLoading(false)
  }

  return { info, loading, error, clear }
}
