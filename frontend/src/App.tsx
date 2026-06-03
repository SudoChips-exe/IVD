import { useState } from 'react'
import './App.css'
import Header from './components/Header'
import URLInput from './components/URLInput'
import QualitySelector, { Quality } from './components/QualitySelector'
import DownloadButton from './components/DownloadButton'
import ProgressBar from './components/ProgressBar'
import ErrorMessage from './components/ErrorMessage'
import Footer from './components/Footer'
import CookieUpload from './components/CookieUpload'
import { CheckCircleIcon, PlatformsIcon, AudioIcon, FastIcon, MobileIcon } from './components/Icons'
import { useDownload } from './hooks/useDownload'
import { useScrollSpy } from './hooks/useScrollSpy'

function App() {
  useScrollSpy()
  const [url, setUrl] = useState('')
  const [quality, setQuality] = useState<Quality>('best')
  const { download, cancel, loading, progress, speed, eta, status, error, success } = useDownload()

  const handleDownload = async () => {
    if (url.trim()) {
      await download(url, quality === 'best' ? undefined : quality)
    }
  }

  return (
    <div className="app-container">
      <Header />

      <main className="main-content">
        <div className="hero-section" id="home">
          <h1>VIDCLAW</h1>
          <p>Extract and download high-quality videos from Instagram, TikTok, YouTube, Twitter, and Facebook — no sign-up required.</p>
        </div>

        <div className="download-section">
          <URLInput
            value={url}
            onChange={setUrl}
            onSubmit={handleDownload}
            disabled={loading}
          />

          <QualitySelector value={quality} onChange={setQuality} disabled={loading} />

          <DownloadButton
            onClick={handleDownload}
            disabled={loading || !url.trim()}
            loading={loading}
          />

          {loading && (
            <>
              <ProgressBar
                progress={progress}
                status={status}
                speed={speed}
                eta={eta}
              />
              <button className="cancel-btn" onClick={cancel}>
                Cancel
              </button>
            </>
          )}

          {error && <ErrorMessage message={error} />}

          {success && (
            <div className="success-message">
              <CheckCircleIcon size={16} />
              <span>Video downloaded successfully.</span>
            </div>
          )}
        </div>

        <div className="section-label" id="features">Features</div>
        <div className="features-section">
          <div className="feature">
            <div className="feature-icon-wrapper">
              <PlatformsIcon size={20} />
            </div>
            <div className="feature-text">
              <h3>All Major Platforms</h3>
              <p>Download from Instagram, TikTok, YouTube, Twitter, and Facebook without restrictions.</p>
            </div>
          </div>

          <div className="feature">
            <div className="feature-icon-wrapper">
              <AudioIcon size={20} />
            </div>
            <div className="feature-text">
              <h3>Audio Preserved</h3>
              <p>Full sound fidelity and audio sync retained for every download.</p>
            </div>
          </div>

          <div className="feature">
            <div className="feature-icon-wrapper">
              <FastIcon size={20} />
            </div>
            <div className="feature-text">
              <h3>Real-Time Progress</h3>
              <p>Live download speed, ETA, and progress streamed directly from the server.</p>
            </div>
          </div>

          <div className="feature">
            <div className="feature-icon-wrapper">
              <MobileIcon size={20} />
            </div>
            <div className="feature-text">
              <h3>Mobile Friendly</h3>
              <p>Fully responsive across Android, iOS, tablet, and desktop.</p>
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
              <p>Choose from Best, 1080p, 720p, 480p, or 360p. "Best" downloads the highest quality available from the source platform.</p>
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
