import { useState } from 'react'
import './App.css'
import Header from './components/Header'
import URLInput from './components/URLInput'
import DownloadButton from './components/DownloadButton'
import ErrorMessage from './components/ErrorMessage'
import ProgressBar from './components/ProgressBar'
import Footer from './components/Footer'
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
          <p>Download videos from Instagram, TikTok, YouTube, Twitter, Facebook & Snapchat with one click</p>
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

          {loading && <ProgressBar progress={progress} status={retryCount > 0 ? `Retry attempt ${retryCount}...` : undefined} />}
          
          {error && <ErrorMessage message={error} />}
          
          {success && (
            <div className="success-message">
              ✓ Video downloaded successfully!
            </div>
          )}
        </div>

        <div className="features-section">
          <div className="feature">
            <h3>🎬 All Platforms</h3>
            <p>Instagram, TikTok, YouTube & more</p>
          </div>
          <div className="feature">
            <h3>🔊 Audio Preserved</h3>
            <p>Download with original audio quality</p>
          </div>
          <div className="feature">
            <h3>⚡ Fast Download</h3>
            <p>Direct streaming, no intermediate storage</p>
          </div>
          <div className="feature">
            <h3>📱 Mobile Friendly</h3>
            <p>Works from any device, any browser</p>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  )
}

export default App
