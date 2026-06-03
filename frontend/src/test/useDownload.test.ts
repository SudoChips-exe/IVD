import { renderHook, act } from '@testing-library/react'
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest'
import { useDownload } from '../hooks/useDownload'
import { api } from '../services/api'

vi.mock('../services/api', () => ({
  api: {
    startDownload: vi.fn(),
    getProgressUrl: vi.fn(),
    downloadFile: vi.fn(),
  },
}))

const mockStart = vi.mocked(api.startDownload)
const mockGetUrl = vi.mocked(api.getProgressUrl)
const mockFile = vi.mocked(api.downloadFile)

// ── EventSource mock ───────────────────────────────────────────────────────────

let lastES: MockES | null = null

class MockES {
  onmessage: ((e: { data: string }) => void) | null = null
  onerror: ((e: Event) => void) | null = null
  close = vi.fn()
  constructor(_url: string) { lastES = this }
  emit(payload: object) { this.onmessage?.({ data: JSON.stringify(payload) }) }
  fail() { this.onerror?.(new Event('error')) }
}

// Flush microtasks (works regardless of fake timers — Promise is not intercepted)
const flushMicrotasks = () => act(async () => {
  await Promise.resolve()
  await Promise.resolve()
  await Promise.resolve()
})

function makeBlob(size = 100) {
  return new Blob([new Uint8Array(size)], { type: 'video/mp4' })
}

describe('useDownload', () => {
  let lastAnchor: HTMLAnchorElement | null = null

  beforeEach(() => {
    vi.clearAllMocks()
    lastES = null
    lastAnchor = null

    // @ts-expect-error replace browser EventSource with mock
    global.EventSource = MockES
    mockGetUrl.mockReturnValue('http://localhost/progress/job-1')

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
    expect(result.current.progress).toBe(0)
    expect(result.current.error).toBeNull()
    expect(result.current.success).toBe(false)
    expect(result.current.speed).toBeNull()
    expect(result.current.eta).toBeNull()
    expect(result.current.status).toBeNull()
  })

  // ── Validation ─────────────────────────────────────────────────────────────

  it('rejects empty URL without calling API', async () => {
    const { result } = renderHook(() => useDownload())
    await act(async () => { await result.current.download('') })
    expect(result.current.error).toBe('Please enter a valid URL')
    expect(result.current.loading).toBe(false)
    expect(mockStart).not.toHaveBeenCalled()
  })

  it('rejects whitespace-only URL', async () => {
    const { result } = renderHook(() => useDownload())
    await act(async () => { await result.current.download('   ') })
    expect(result.current.error).toBe('Please enter a valid URL')
    expect(mockStart).not.toHaveBeenCalled()
  })

  // ── Successful download ────────────────────────────────────────────────────

  it('full success flow: progress → done → file download', async () => {
    const blob = makeBlob()
    mockStart.mockResolvedValueOnce({ job_id: 'job-1' })
    mockFile.mockResolvedValueOnce({
      data: blob,
      headers: { 'content-disposition': 'attachment; filename="video.mp4"' },
    } as any)

    const { result } = renderHook(() => useDownload())

    let p!: Promise<void>
    act(() => { p = result.current.download('https://youtube.com/watch?v=abc') })

    await flushMicrotasks()
    expect(lastES).not.toBeNull()

    act(() => { lastES!.emit({ type: 'progress', percent: 60, speed: '2MiB/s', eta: '00:05' }) })
    expect(result.current.progress).toBe(60)
    expect(result.current.speed).toBe('2MiB/s')
    expect(result.current.eta).toBe('00:05')

    act(() => { lastES!.emit({ type: 'done', filename: 'video.mp4' }) })
    await act(async () => { await p })

    expect(result.current.success).toBe(true)
    expect(result.current.progress).toBe(100)
    expect(result.current.loading).toBe(false)
    expect(result.current.error).toBeNull()
    expect(mockFile).toHaveBeenCalledWith('job-1')
    expect(lastAnchor?.download).toBe('video.mp4')
    expect(lastAnchor?.click).toHaveBeenCalledOnce()
  })

  it('shows authenticating status event', async () => {
    mockStart.mockResolvedValueOnce({ job_id: 'job-1' })
    mockFile.mockResolvedValueOnce({ data: makeBlob(), headers: {} } as any)

    const { result } = renderHook(() => useDownload())
    let p!: Promise<void>
    act(() => { p = result.current.download('https://instagram.com/reel/abc') })
    await flushMicrotasks()

    act(() => { lastES!.emit({ type: 'authenticating', method: 'chromium' }) })
    expect(result.current.status).toContain('chromium')

    act(() => { lastES!.emit({ type: 'done', filename: 'v.mp4' }) })
    await act(async () => { await p })
    expect(result.current.success).toBe(true)
  })

  it('shows merging status at 95%', async () => {
    mockStart.mockResolvedValueOnce({ job_id: 'job-1' })
    mockFile.mockResolvedValueOnce({ data: makeBlob(), headers: {} } as any)

    const { result } = renderHook(() => useDownload())
    let p!: Promise<void>
    act(() => { p = result.current.download('https://youtube.com/watch?v=abc') })
    await flushMicrotasks()

    act(() => { lastES!.emit({ type: 'merging' }) })
    expect(result.current.progress).toBe(95)
    expect(result.current.status).toContain('Merging')

    act(() => { lastES!.emit({ type: 'done', filename: 'v.mp4' }) })
    await act(async () => { await p })
    expect(result.current.success).toBe(true)
  })

  it('uses default filename when header absent', async () => {
    mockStart.mockResolvedValueOnce({ job_id: 'job-1' })
    mockFile.mockResolvedValueOnce({ data: makeBlob(), headers: {} } as any)

    const { result } = renderHook(() => useDownload())
    let p!: Promise<void>
    act(() => { p = result.current.download('https://youtube.com/watch?v=abc') })
    await flushMicrotasks()
    act(() => { lastES!.emit({ type: 'done', filename: 'v.mp4' }) })
    await act(async () => { await p })

    expect(lastAnchor?.download).toBe('downloaded_video.mp4')
  })

  it('resets success after 5 seconds', async () => {
    vi.useFakeTimers()  // before everything so hook's setTimeout uses fake timer
    mockStart.mockResolvedValueOnce({ job_id: 'job-1' })
    mockFile.mockResolvedValueOnce({ data: makeBlob(), headers: {} } as any)

    const { result } = renderHook(() => useDownload())
    let p!: Promise<void>
    act(() => { p = result.current.download('https://youtube.com/watch?v=abc') })
    // flushMicrotasks uses Promise.resolve() — safe with fake timers (only setTimeout is mocked)
    await flushMicrotasks()

    act(() => { lastES!.emit({ type: 'done', filename: 'v.mp4' }) })
    await act(async () => { await p })
    expect(result.current.success).toBe(true)

    await act(async () => { vi.advanceTimersByTime(5000) })
    expect(result.current.success).toBe(false)
  })

  // ── Error cases ────────────────────────────────────────────────────────────

  it('SSE error event sets error state', async () => {
    mockStart.mockResolvedValueOnce({ job_id: 'job-1' })

    const { result } = renderHook(() => useDownload())
    let p!: Promise<void>
    act(() => { p = result.current.download('https://instagram.com/reel/abc') })
    await flushMicrotasks()

    act(() => { lastES!.emit({ type: 'error', message: 'This video requires authentication.' }) })
    await act(async () => { await p })

    expect(result.current.error).toContain('authentication')
    expect(result.current.loading).toBe(false)
  })

  it('EventSource connection error sets error state', async () => {
    mockStart.mockResolvedValueOnce({ job_id: 'job-1' })

    const { result } = renderHook(() => useDownload())
    let p!: Promise<void>
    act(() => { p = result.current.download('https://youtube.com/watch?v=abc') })
    await flushMicrotasks()

    act(() => { lastES!.fail() })
    await act(async () => { await p })

    expect(result.current.error).toContain('Connection lost')
    expect(result.current.loading).toBe(false)
  })

  it('startDownload API failure sets error', async () => {
    mockStart.mockRejectedValueOnce(new Error('Network error'))

    const { result } = renderHook(() => useDownload())
    await act(async () => { await result.current.download('https://youtube.com/watch?v=abc') })

    expect(result.current.error).toContain('Network error')
    expect(result.current.loading).toBe(false)
  })

  // ── Progress ───────────────────────────────────────────────────────────────

  it('progress updates correctly from SSE events', async () => {
    mockStart.mockResolvedValueOnce({ job_id: 'job-1' })
    mockFile.mockResolvedValueOnce({ data: makeBlob(), headers: {} } as any)

    const { result } = renderHook(() => useDownload())
    let p!: Promise<void>
    act(() => { p = result.current.download('https://youtube.com/watch?v=abc') })
    await flushMicrotasks()

    act(() => { lastES!.emit({ type: 'progress', percent: 33 }) })
    expect(result.current.progress).toBe(33)

    act(() => { lastES!.emit({ type: 'progress', percent: 66 }) })
    expect(result.current.progress).toBe(66)

    act(() => { lastES!.emit({ type: 'done', filename: 'v.mp4' }) })
    await act(async () => { await p })
    expect(result.current.progress).toBe(100)
  })
})
