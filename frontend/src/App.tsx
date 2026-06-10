import { useState, useEffect } from 'react'
import './App.css'
import Header from './components/Header'
import URLInput from './components/URLInput'
import QualitySelector, { Quality } from './components/QualitySelector'
import DownloadButton from './components/DownloadButton'
import Footer from './components/Footer'
import CookieUpload from './components/CookieUpload'
import VideoPreview from './components/VideoPreview'
import DownloadQueueItem from './components/DownloadQueueItem'
import { PlatformsIcon, AudioIcon, FastIcon, MobileIcon } from './components/Icons'
import { useDownloadQueue } from './hooks/useDownloadQueue'
import { useVideoInfo } from './hooks/useVideoInfo'
import { useScrollSpy } from './hooks/useScrollSpy'
import { getHistory, clearHistory } from './utils/history'
import { isPlaylistUrl } from './utils/urlDetection'
import { HistoryEntry, PlaylistInfo } from './types'
import { api } from './services/api'

function App() {
  useScrollSpy()
  const [url, setUrl] = useState('')
  const [quality, setQuality] = useState<Quality>('best')
  const [audioOnly, setAudioOnly] = useState(false)
  const [history, setHistory] = useState<HistoryEntry[]>([])
  const [playlistInfo, setPlaylistInfo] = useState<PlaylistInfo | null>(null)
  const [playlistLoading, setPlaylistLoading] = useState(false)
  const [darkMode, setDarkMode] = useState(true)

  // Apply theme to <html> element
  useEffect(() => {
    document.documentElement.setAttribute('data-theme', darkMode ? 'dark' : 'light')
  }, [darkMode])

  const isPlaylist = isPlaylistUrl(url)
  const { info, loading: infoLoading, error: infoError, clear: clearInfo } = useVideoInfo(isPlaylist ? '' : url)
  const { items, addDownload, cancelItem, removeItem } = useDownloadQueue()

  useEffect(() => {
    setHistory(getHistory())
  }, [])

  useEffect(() => {
    if (items.some(i => i.state === 'done')) {
      setHistory(getHistory())
    }
  }, [items])

  useEffect(() => {
    if (!isPlaylist) {
      setPlaylistInfo(null)
      return
    }
    const trimmed = url.trim()
    setPlaylistInfo(null)
    setPlaylistLoading(true)
    let cancelled = false
    api.getPlaylistInfo(trimmed)
      .then(data => { if (!cancelled) setPlaylistInfo(data) })
      .catch(() => {})
      .finally(() => { if (!cancelled) setPlaylistLoading(false) })
    return () => { cancelled = true }
  }, [url, isPlaylist])

  const handleDownload = () => {
    const trimmed = url.trim()
    if (!trimmed) return
    clearInfo()
    setUrl('')
    addDownload(trimmed, quality, audioOnly, info ?? undefined)
  }

  const handleDownloadAll = () => {
    if (!playlistInfo) return
    setUrl('')
    setPlaylistInfo(null)
    playlistInfo.entries.forEach(entry => {
      addDownload(entry.url, quality, audioOnly)
    })
  }

  const activeItems = items.filter(i => i.state === 'downloading')
  const completedItems = items.filter(i => i.state !== 'downloading')

  return (
    <div className="app-container">
      {/* Animated background */}
      <div className="bg-mesh" />
      <div className="bg-orb" />
      <div className="grain" />

      <Header darkMode={darkMode} onToggleTheme={() => setDarkMode(d => !d)} />

      <main className="main-content">
        <div className="hero-section" id="home">
          <div className="hero-eyebrow">
            <span className="hero-status-dot" />
            Free &middot; No account required
          </div>
          <h1>VIDCLAW</h1>
          <p>Download high-quality video and audio from any major social platform. Instant, private, no limits.</p>
          <div className="hero-chips">
            <span>Instagram</span>
            <span>TikTok</span>
            <span>YouTube</span>
            <span>Twitter</span>
            <span>Facebook</span>
          </div>
        </div>

        <div className="download-section">
          <URLInput
            value={url}
            onChange={setUrl}
            onSubmit={isPlaylist ? handleDownloadAll : handleDownload}
            disabled={false}
          />

          {isPlaylist && (
            <div className="playlist-banner">
              <div className="playlist-banner-info">
                {playlistLoading ? (
                  <span className="playlist-loading">Loading playlist…</span>
                ) : playlistInfo ? (
                  <>
                    <span className="playlist-title">{playlistInfo.title}</span>
                    <span className="playlist-count">{playlistInfo.total} video{playlistInfo.total !== 1 ? 's' : ''}</span>
                  </>
                ) : null}
              </div>
              <button
                className="playlist-download-all-btn"
                type="button"
                onClick={handleDownloadAll}
                disabled={!playlistInfo || playlistLoading}
              >
                Download all
              </button>
            </div>
          )}

          {!isPlaylist && <VideoPreview info={info} loading={infoLoading} error={infoError} />}

          <QualitySelector
            value={quality}
            onChange={setQuality}
            audioOnly={audioOnly}
            onAudioOnlyChange={setAudioOnly}
            isImage={info?.is_image}
          />

          <DownloadButton
            onClick={isPlaylist ? handleDownloadAll : handleDownload}
            disabled={!url.trim() || (isPlaylist && (!playlistInfo || playlistLoading))}
            loading={false}
          />
        </div>

        {activeItems.length > 0 && (
          <div className="queue-section-wrapper">
            <div className="queue-header">
              <span className="queue-title">Downloading</span>
              <span className="queue-count">{activeItems.length} running</span>
            </div>
            {activeItems.map(item => (
              <DownloadQueueItem
                key={item.id}
                item={item}
                onCancel={() => cancelItem(item.id)}
                onRemove={() => removeItem(item.id)}
              />
            ))}
          </div>
        )}

        {completedItems.length > 0 && (
          <div className="queue-section-wrapper">
            <div className="queue-header">
              <span className="queue-title">Completed</span>
              <span className="queue-count">{completedItems.length}</span>
            </div>
            {completedItems.map(item => (
              <DownloadQueueItem
                key={item.id}
                item={item}
                onCancel={() => cancelItem(item.id)}
                onRemove={() => removeItem(item.id)}
              />
            ))}
          </div>
        )}

        {history.length > 0 && (
          <div className="queue-section-wrapper" id="history">
            <div className="queue-header">
              <span className="queue-title">History</span>
              <button
                className="clear-history-btn"
                type="button"
                onClick={() => { clearHistory(); setHistory([]) }}
              >
                Clear all
              </button>
            </div>
            <div className="history-section" style={{padding: 0}}>
              {history.slice(0, 20).map(entry => (
                <div key={entry.id} className="history-item">
                  {entry.thumbnail ? (
                    <img
                      className="history-thumb"
                      src={entry.thumbnail}
                      alt=""
                      loading="lazy"
                      onError={(e) => { (e.target as HTMLImageElement).style.display = 'none' }}
                    />
                  ) : (
                    <div className="hist-icon">
                      <svg viewBox="0 0 24 24"><path d="M15 10l4.553-2.277A1 1 0 0 1 21 8.618v6.764a1 1 0 0 1-1.447.894L15 14M3 8a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8z"/></svg>
                    </div>
                  )}
                  <div className="history-info">
                    <p className="history-title">{entry.title}</p>
                    <div className="history-meta">
                      <span className="platform-badge">{entry.platform}</span>
                      <span className="history-quality">{entry.audioOnly ? 'MP3' : entry.quality.toUpperCase()}</span>
                      <span className="history-date">{new Date(entry.timestamp).toLocaleDateString()}</span>
                    </div>
                  </div>
                  <div className="history-actions">
                    <button
                      className="history-action-btn"
                      type="button"
                      title="Re-download"
                      onClick={() => addDownload(entry.url, entry.quality as Quality, entry.audioOnly)}
                      aria-label="Re-download"
                    >↺</button>
                    <button
                      className="history-action-btn"
                      type="button"
                      title="Copy URL"
                      onClick={() => navigator.clipboard.writeText(entry.url).catch(() => {})}
                      aria-label="Copy URL"
                    >⎘</button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        <div className="section-label" id="features">Features</div>
        <div className="features-section">
          <div className="feature">
            <div className="feature-icon-wrapper"><PlatformsIcon size={20} /></div>
            <div className="feature-text">
              <h3>All Major Platforms</h3>
              <p>Download from Instagram, TikTok, YouTube, Twitter, and Facebook without restrictions.</p>
            </div>
          </div>
          <div className="feature">
            <div className="feature-icon-wrapper"><AudioIcon size={20} /></div>
            <div className="feature-text">
              <h3>Audio &amp; Video</h3>
              <p>Download full video or extract audio as MP3 — your choice, every time.</p>
            </div>
          </div>
          <div className="feature">
            <div className="feature-icon-wrapper"><FastIcon size={20} /></div>
            <div className="feature-text">
              <h3>Real-Time Progress</h3>
              <p>Live download speed, ETA, and progress streamed directly from the server.</p>
            </div>
          </div>
          <div className="feature">
            <div className="feature-icon-wrapper"><MobileIcon size={20} /></div>
            <div className="feature-text">
              <h3>Queue Multiple Downloads</h3>
              <p>Start several downloads simultaneously — paste a URL and queue another while the first runs.</p>
            </div>
          </div>
        </div>

        <div className="section-label" id="faq">FAQ</div>
        <div className="faq-section">
          <div className="faq-list">
            <div className="faq-item">
              <h3>Which platforms are supported?</h3>
              <p>Instagram, TikTok, YouTube, Twitter, and Facebook.</p>
            </div>
            <div className="faq-item">
              <h3>Is it free to use?</h3>
              <p>Yes — completely free with no account required.</p>
            </div>
            <div className="faq-item">
              <h3>What video quality is downloaded?</h3>
              <p>Choose from Best, 1080p, 720p, 480p, or 360p. Select MP3 to extract audio only.</p>
            </div>
            <div className="faq-item">
              <h3>Why do Instagram downloads fail?</h3>
              <p>Instagram requires authentication. Upload your cookies.txt file using the settings below.</p>
            </div>
            <div className="faq-item">
              <h3>Is downloading videos legal?</h3>
              <p>Only download content you own or have explicit permission to use. Respect platform terms of service and copyright law.</p>
            </div>
          </div>
        </div>

        <div className="section-label" id="contact">Settings</div>
        <div className="settings-section">
          <CookieUpload />
        </div>
      </main>

      <Footer />
    </div>
  )
}

export default App
