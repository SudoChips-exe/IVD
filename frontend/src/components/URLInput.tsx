import React, { FC } from 'react'
import { detectPlatform } from '../utils/urlDetection'
import '../styles/components.css'

interface URLInputProps {
  value: string
  onChange: (value: string) => void
  onSubmit: () => void
  disabled?: boolean
}

const URLInput: FC<URLInputProps> = ({ value, onChange, onSubmit, disabled }) => {
  const platform = value ? detectPlatform(value) : null
  
  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !disabled) {
      onSubmit()
    }
  }

  const handlePaste = async () => {
    try {
      const text = await navigator.clipboard.readText()
      onChange(text)
    } catch (err) {
      console.error('Failed to read clipboard:', err)
    }
  }

  return (
    <div className="url-input-container">
      <div className="input-wrapper">
        <input
          type="text"
          placeholder="Paste video URL here... (Instagram, TikTok, YouTube, etc.)"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          onKeyPress={handleKeyPress}
          disabled={disabled}
          className="url-input"
        />
        <button 
          className="paste-button"
          onClick={handlePaste}
          disabled={disabled}
          title="Paste from clipboard"
        >
          📋 Paste
        </button>
      </div>
      
      {platform && (
        <div className="platform-indicator">
          Detected: <strong>{platform}</strong>
        </div>
      )}
    </div>
  )
}

export default URLInput
