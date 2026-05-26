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

function App() {
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
        <div className="hero-section">
          <h1>Universal Video Downloader</h1>
          <p>Extract and download high-quality videos from Instagram, TikTok, YouTube, Twitter, Facebook, and Snapchat with a single click.</p>
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
              <CheckCircleIcon size={18} />
              <span>Video file processed and downloaded successfully!</span>
            </div>
          )}
        </div>

        <div className="features-section" id="features">
          <div className="feature">
            <div className="feature-icon-wrapper">
              <PlatformsIcon size={22} />
            </div>
            <div className="feature-text">
              <h3>All Major Platforms</h3>
              <p>Download directly from Instagram, TikTok, YouTube, Twitter, Facebook, and Snapchat without restrictions.</p>
            </div>
          </div>
          
          <div className="feature">
            <div className="feature-icon-wrapper">
              <AudioIcon size={22} />
            </div>
            <div className="feature-text">
              <h3>Audio Preserved</h3>
              <p>Retain full sound fidelity, high-quality audio formats, and sync for every video download.</p>
            </div>
          </div>
          
          <div className="feature">
            <div className="feature-icon-wrapper">
              <FastIcon size={22} />
            </div>
            <div className="feature-text">
              <h3>Direct Stream Speed</h3>
              <p>Fast streaming processing bypassing intermediate storage, delivering your download instantly.</p>
            </div>
          </div>
          
          <div className="feature">
            <div className="feature-icon-wrapper">
              <MobileIcon size={22} />
            </div>
            <div className="feature-text">
              <h3>Mobile Friendly</h3>
              <p>Fully responsive viewport layout built to perform on Android, iOS, tablet, or desktop devices.</p>
            </div>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  )
}

export default App
