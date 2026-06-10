import React, { FC } from 'react'
import { detectPlatform } from '../utils/urlDetection'

interface URLInputProps {
  value: string
  onChange: (value: string) => void
  onSubmit: () => void
  disabled?: boolean
}

const URLInput: FC<URLInputProps> = ({ value, onChange, onSubmit, disabled }) => {
  const platform = value ? detectPlatform(value) : null

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !disabled) onSubmit()
  }

  const handlePaste = async () => {
    try {
      const text = await navigator.clipboard.readText()
      onChange(text)
    } catch {
      // clipboard access denied
    }
  }

  const platforms = ['YouTube', 'TikTok', 'Instagram', 'Twitter', 'Facebook']

  return (
    <div>
      <span className="input-label">Paste URL</span>
      <div className="url-bar">
        <input
          type="text"
          className="url-input"
          placeholder="https://youtube.com/watch?v=..."
          value={value}
          onChange={(e) => onChange(e.target.value)}
          onKeyDown={handleKeyDown}
          disabled={disabled}
        />
        <button className="btn-fetch" onClick={onSubmit} disabled={disabled}>
          <svg viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.92)" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" width="14" height="14">
            <circle cx="11" cy="11" r="6"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
          </svg>
          Fetch
        </button>
      </div>
      <div className="platform-row">
        {platforms.map(p => (
          <span key={p} className={`chip${platform === p ? ' active' : ''}`}>{p}</span>
        ))}
      </div>
      <button className="cookie-toggle" type="button" onClick={handlePaste} title="Paste from clipboard">
        ⎘ Paste from clipboard
      </button>
    </div>
  )
}

export default URLInput
