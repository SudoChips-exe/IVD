import { useState } from 'react'
import './App.css'
import Header from './components/Header'
import URLInput from './components/URLInput'
import DownloadButton from './components/DownloadButton'
import ErrorMessage from './components/ErrorMessage'
import ProgressBar from './components/ProgressBar'
import Footer from './components/Footer'
import { CheckCircleIcon, PlatformsIcon, AudioIcon, FastIcon, MobileIcon } from './components/Icons'
import { useDownload } from './hooks/useDownload'
import { useScrollSpy } from './hooks/useScrollSpy'

function App() {
  useScrollSpy()
  const [url, setUrl] = useState('')
  const { download, loading, progress, error, success, retryCount } = useDownload()

  const handleDownload = async () => {
    if (url.trim()) {
      await download(url)
    }
  }

  return (
    <div className="app-container">
      <Header />

      <main className="main-content">
        <div className="hero-section" id="home">
          <h1>VIDCLAW</h1>
          <p>Extract and download high-quality videos from Instagram, TikTok, YouTube, Twitter, Facebook, and Snapchat — no sign-up required.</p>
        </div>

        <div className="download-section">
          <URLInput
            value={url}
            onChange={setUrl}
            onSubmit={handleDownload}
            disabled={loading}
          />

          <DownloadButton
            onClick={handleDownload}
            disabled={loading || !url.trim()}
            loading={loading}
          />

          {loading && (
            <ProgressBar
              progress={progress}
              status={retryCount > 0 ? `Retry attempt ${retryCount} of 3...` : undefined}
            />
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
              <p>Download from Instagram, TikTok, YouTube, Twitter, Facebook, and Snapchat without restrictions.</p>
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
              <h3>Direct Stream Speed</h3>
              <p>Streaming processing bypasses intermediate storage for instant delivery.</p>
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
              <p>Instagram, TikTok, YouTube, Twitter, Facebook, and Snapchat.</p>
            </div>
            <div className="faq-item">
              <h3>Is it free to use?</h3>
              <p>Yes — completely free with no account required.</p>
            </div>
            <div className="faq-item">
              <h3>What video quality is downloaded?</h3>
              <p>The highest quality available from the source platform.</p>
            </div>
            <div className="faq-item">
              <h3>Is downloading videos legal?</h3>
              <p>Only download content you own or have explicit permission to use. Respect platform terms of service and copyright law.</p>
            </div>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  )
}

export default App
