import { useState, useCallback } from 'react'
import { api } from '../services/api'

export const useDownload = () => {
  const [loading, setLoading] = useState(false)
  const [progress, setProgress] = useState(0)
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState(false)

  const download = useCallback(async (url: string) => {
    setLoading(true)
    setError(null)
    setSuccess(false)
    setProgress(0)

    try {
      // Validate URL
      if (!url.trim()) {
        throw new Error('Please enter a valid URL')
      }

      // Start download
      setProgress(25)
      const response = await api.downloadVideo(url)
      
      setProgress(75)

      // Extract filename from response headers
      const contentDisposition = response.headers['content-disposition']
      let filename = 'downloaded_video.mp4'
      
      if (contentDisposition) {
        const filenameMatch = contentDisposition.match(/filename="(.+?)"/)
        if (filenameMatch) {
          filename = filenameMatch[1]
        }
      }

      // Create blob and trigger download
      const blob = await response.data as Blob
      const downloadUrl = window.URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = downloadUrl
      link.download = filename
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      window.URL.revokeObjectURL(downloadUrl)

      setProgress(100)
      setSuccess(true)
      
      // Clear success message after 3 seconds
      setTimeout(() => setSuccess(false), 3000)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Download failed. Please try again.'
      setError(errorMessage)
      console.error('Download error:', err)
    } finally {
      setLoading(false)
    }
  }, [])

  return { download, loading, progress, error, success }
}
