import { renderHook, act } from '@testing-library/react'
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest'
import { useDownload } from '../hooks/useDownload'
import { api } from '../services/api'

vi.mock('../services/api', () => ({
  api: { downloadVideo: vi.fn() },
}))

const mockDownloadVideo = vi.mocked(api.downloadVideo)

function makeBlob(size = 100, type = 'video/mp4') {
  return new Blob([new Uint8Array(size)], { type })
}

describe('useDownload', () => {
  let lastAnchor: HTMLAnchorElement | null = null

  beforeEach(() => {
    vi.clearAllMocks()
    lastAnchor = null

    // Intercept createElement: return real element, but capture anchor + stub click
    const realCreate = document.createElement.bind(document)
    vi.spyOn(document, 'createElement').mockImplementation((tag: string, ...args: any[]) => {
      const el = realCreate(tag, ...args)
      if (tag === 'a') {
        lastAnchor = el as HTMLAnchorElement
        vi.spyOn(el as HTMLAnchorElement, 'click').mockImplementation(() => {})
      }
      return el
    })

    window.URL.createObjectURL = vi.fn().mockReturnValue('blob:fake-url')
    window.URL.revokeObjectURL = vi.fn()
  })

  afterEach(() => {
    vi.restoreAllMocks()
    vi.useRealTimers()
  })

  // ── Initial state ──────────────────────────────────────────────────────────

  it('starts with idle state', () => {
    const { result } = renderHook(() => useDownload())
    expect(result.current.loading).toBe(false)
    expect(result.current.error).toBeNull()
    expect(result.current.success).toBe(false)
    expect(result.current.progress).toBe(0)
    expect(result.current.retryCount).toBe(0)
  })

  // ── Validation ─────────────────────────────────────────────────────────────

  it('rejects empty URL without calling API', async () => {
    const { result } = renderHook(() => useDownload())
    await act(async () => { await result.current.download('') })
    expect(result.current.error).toBe('Please enter a valid URL')
    expect(result.current.loading).toBe(false)
    expect(mockDownloadVideo).not.toHaveBeenCalled()
  })

  it('rejects whitespace-only URL', async () => {
    const { result } = renderHook(() => useDownload())
    await act(async () => { await result.current.download('   ') })
    expect(result.current.error).toBe('Please enter a valid URL')
    expect(mockDownloadVideo).not.toHaveBeenCalled()
  })

  // ── Successful download ────────────────────────────────────────────────────

  it('triggers link click and sets success state', async () => {
    const blob = makeBlob()
    mockDownloadVideo.mockResolvedValueOnce({
      data: blob,
      headers: { 'content-disposition': 'attachment; filename="instagram_abc_downloaded.mp4"' },
    } as any)

    const { result } = renderHook(() => useDownload())
    await act(async () => {
      await result.current.download('https://www.instagram.com/reel/abc123')
    })

    expect(mockDownloadVideo).toHaveBeenCalledWith('https://www.instagram.com/reel/abc123')
    expect(window.URL.createObjectURL).toHaveBeenCalledWith(blob)
    expect(lastAnchor?.href).toBe('blob:fake-url')
    expect(lastAnchor?.download).toBe('instagram_abc_downloaded.mp4')
    expect(lastAnchor?.click).toHaveBeenCalledOnce()
    expect(window.URL.revokeObjectURL).toHaveBeenCalledWith('blob:fake-url')
    expect(result.current.success).toBe(true)
    expect(result.current.loading).toBe(false)
    expect(result.current.progress).toBe(100)
    expect(result.current.error).toBeNull()
  })

  it('uses default filename when content-disposition header is absent', async () => {
    mockDownloadVideo.mockResolvedValueOnce({ data: makeBlob(), headers: {} } as any)
    const { result } = renderHook(() => useDownload())
    await act(async () => {
      await result.current.download('https://www.youtube.com/watch?v=abc')
    })
    expect(lastAnchor?.download).toBe('downloaded_video.mp4')
    expect(result.current.success).toBe(true)
  })

  it('uses default filename when content-disposition has no filename match', async () => {
    mockDownloadVideo.mockResolvedValueOnce({
      data: makeBlob(),
      headers: { 'content-disposition': 'attachment' },
    } as any)
    const { result } = renderHook(() => useDownload())
    await act(async () => {
      await result.current.download('https://www.tiktok.com/@user/video/123')
    })
    expect(lastAnchor?.download).toBe('downloaded_video.mp4')
  })

  it('resets success after 5 seconds', async () => {
    vi.useFakeTimers()
    mockDownloadVideo.mockResolvedValueOnce({ data: makeBlob(), headers: {} } as any)
    const { result } = renderHook(() => useDownload())

    await act(async () => {
      await result.current.download('https://www.instagram.com/reel/abc')
    })
    expect(result.current.success).toBe(true)

    await act(async () => { vi.advanceTimersByTime(5000) })
    expect(result.current.success).toBe(false)
  })

  // ── Error — empty blob (retryable) ─────────────────────────────────────────

  it('errors on empty blob and retries 3 times', async () => {
    vi.useFakeTimers()
    mockDownloadVideo.mockResolvedValue({ data: makeBlob(0), headers: {} } as any)
    const { result } = renderHook(() => useDownload())

    await act(async () => {
      const p = result.current.download('https://www.instagram.com/reel/abc')
      await vi.runAllTimersAsync()
      await p
    })

    expect(result.current.loading).toBe(false)
    expect(result.current.error).toContain('Downloaded file is empty')
    expect(mockDownloadVideo).toHaveBeenCalledTimes(4) // initial + 3 retries
  })

  // ── Error — network failure (retryable) ────────────────────────────────────

  it('retries on network error and shows final error after max retries', async () => {
    vi.useFakeTimers()
    mockDownloadVideo.mockRejectedValue(new Error('Network timeout'))
    const { result } = renderHook(() => useDownload())

    await act(async () => {
      const p = result.current.download('https://www.tiktok.com/@user/video/123')
      await vi.runAllTimersAsync()
      await p
    })

    expect(result.current.loading).toBe(false)
    expect(result.current.error).toContain('Network timeout')
    expect(mockDownloadVideo).toHaveBeenCalledTimes(4) // initial + 3 retries
  })

  it('does not retry on non-retryable URL validation error', async () => {
    const { result } = renderHook(() => useDownload())
    await act(async () => { await result.current.download('') })
    expect(mockDownloadVideo).not.toHaveBeenCalled()
    expect(result.current.retryCount).toBe(0)
  })

  // ── Progress ───────────────────────────────────────────────────────────────

  it('reaches progress 100 on successful download', async () => {
    mockDownloadVideo.mockResolvedValueOnce({ data: makeBlob(), headers: {} } as any)
    const { result } = renderHook(() => useDownload())
    await act(async () => {
      await result.current.download('https://www.instagram.com/reel/abc')
    })
    expect(result.current.progress).toBe(100)
  })
})
