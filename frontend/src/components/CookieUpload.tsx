import { FC, useEffect, useRef, useState } from 'react'
import { api } from '../services/api'

const CookieUpload: FC = () => {
  const [active, setActive] = useState(false)
  const [uploading, setUploading] = useState(false)
  const [message, setMessage] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)
  const fileRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    api.getCookiesStatus().then(s => setActive(s.active)).catch(() => {})
  }, [])

  const handleFile = async (file: File) => {
    setUploading(true)
    setMessage(null)
    setError(null)
    try {
      const content = await file.text()
      await api.uploadCookies(content)
      setActive(true)
      setMessage('Cookies saved. Instagram and Facebook downloads enabled.')
    } catch {
      setError('Upload failed. Make sure you exported a valid cookies.txt file.')
    } finally {
      setUploading(false)
    }
  }

  const handleRemove = async () => {
    try {
      await api.deleteCookies()
      setActive(false)
      setMessage('Cookies removed.')
    } catch {
      setError('Failed to remove cookies.')
    }
  }

  return (
    <div className="cookie-upload">
      <div className="cookie-header">
        <span className="cookie-title">Auth Cookies</span>
        <span className={`cookie-status${active ? ' active' : ''}`}>
          {active ? '● Active' : '○ Not set'}
        </span>
      </div>
      <p className="cookie-desc">
        Required for Instagram &amp; Facebook. Export <code>cookies.txt</code> using the
        {' '}<em>Get cookies.txt LOCALLY</em> browser extension while logged in.
      </p>
      <div className="cookie-actions">
        <button
          className="cookie-btn"
          onClick={() => fileRef.current?.click()}
          disabled={uploading}
        >
          {uploading ? 'Uploading...' : 'Upload cookies.txt'}
        </button>
        {active && (
          <button className="cookie-btn danger" onClick={handleRemove}>
            Remove
          </button>
        )}
      </div>
      <input
        ref={fileRef}
        type="file"
        accept=".txt,text/plain"
        style={{ display: 'none' }}
        onChange={e => e.target.files?.[0] && handleFile(e.target.files[0])}
      />
      {message && <p className="cookie-msg success">{message}</p>}
      {error && <p className="cookie-msg error">{error}</p>}
    </div>
  )
}

export default CookieUpload
