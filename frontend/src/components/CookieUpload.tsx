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
    setUploading(true); setMessage(null); setError(null)
    try {
      const content = await file.text()
      await api.uploadCookies(content)
      setActive(true)
      setMessage('Cookies saved. Instagram and Facebook downloads enabled.')
    } catch {
      setError('Upload failed. Make sure you exported a valid cookies.txt file.')
    } finally {
      setUploading(false) }
  }

  return (
    <div>
      <span className="input-label">Auth Cookies</span>
      <p style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', marginBottom: '0.75rem', lineHeight: 1.55 }}>
        Required for Instagram &amp; Facebook. Export <code style={{ fontFamily: 'JetBrains Mono', color: 'var(--accent-chip-text)' }}>cookies.txt</code> using
        the <em>Get cookies.txt LOCALLY</em> browser extension while logged in.
        {active && <span style={{ marginLeft: 8, color: 'var(--success-text, rgba(20,140,60,0.9))' }}>● Active</span>}
      </p>
      <div style={{ display: 'flex', gap: '0.6rem' }}>
        <button className="btn-fetch" type="button" onClick={() => fileRef.current?.click()} disabled={uploading}>
          {uploading ? 'Uploading…' : 'Upload cookies.txt'}
        </button>
        {active && (
          <button
            className="cookie-toggle"
            type="button"
            onClick={async () => { await api.deleteCookies().catch(() => {}); setActive(false); setMessage('Cookies removed.') }}
          >
            Remove
          </button>
        )}
      </div>
      <input ref={fileRef} type="file" accept=".txt,text/plain" style={{ display: 'none' }}
        onChange={e => e.target.files?.[0] && handleFile(e.target.files[0])} />
      {message && <p style={{ fontSize: '0.72rem', color: 'var(--success-text, rgba(20,140,60,0.9))', marginTop: '0.5rem', fontFamily: 'JetBrains Mono' }}>{message}</p>}
      {error && <div className="error-message">{error}</div>}
    </div>
  )
}

export default CookieUpload
