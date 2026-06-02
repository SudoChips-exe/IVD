import { useState, useCallback } from 'react'
import { api } from '../services/api'

interface DownloadState {
  loading: boolean
  progress: number
  error: string | null
  success: boolean
  retryCount: number
}

const MAX_RETRIES = 3
const RETRY_DELAY = 1000 // 1 second

export const useDownload = () => {
  const [state, setState] = useState<DownloadState>({
    loading: false,
    progress: 0,
    error: null,
    success: false,
    retryCount: 0,
  })

  const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

  const download = useCallback(async (url: string, retryAttempt = 0): Promise<void> => {
    // Skip validation if retrying
    if (retryAttempt === 0) {
      setState({
        loading: true,
        progress: 0,
        error: null,
        success: false,
        retryCount: 0,
      })
    }

    try {
      // Validate URL only on first attempt
      if (!url.trim()) {
        throw new Error('Please enter a valid URL')
      }

      // Start download with progress indication
      setState(prev => ({ ...prev, progress: 25 }))
      
      const response = await api.downloadVideo(url.trim())
      
      setState(prev => ({ ...prev, progress: 75 }))

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
      
      // Validate blob
      if (!blob || blob.size === 0) {
        throw new Error('Downloaded file is empty')
      }

      const downloadUrl = window.URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = downloadUrl
      link.download = filename
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      window.URL.revokeObjectURL(downloadUrl)

      setState(prev => ({
        ...prev,
        progress: 100,
        success: true,
        loading: false,
      }))
      
      // Clear success message after 5 seconds
      setTimeout(() => setState(prev => ({ ...prev, success: false })), 5000)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Download failed. Please try again.'
      console.error('Download error:', err)

      // Determine if error is retryable
      const isRetryable = !errorMessage.includes('Invalid URL') && 
                         !errorMessage.includes('Please enter a valid URL') &&
                         retryAttempt < MAX_RETRIES

      if (isRetryable) {
        setState(prev => ({ 
          ...prev, 
          error: `${errorMessage} (Retrying... Attempt ${retryAttempt + 1}/${MAX_RETRIES})`,
          retryCount: retryAttempt + 1,
        }))
        
        await sleep(RETRY_DELAY * (retryAttempt + 1)) // Exponential backoff
        await download(url, retryAttempt + 1)
      } else {
        setState(prev => ({
          ...prev,
          error: errorMessage,
          loading: false,
          retryCount: retryAttempt,
        }))
      }
    }
  }, [])

  return { 
    download, 
    loading: state.loading, 
    progress: state.progress, 
    error: state.error, 
    success: state.success,
    retryCount: state.retryCount,
  }
}
