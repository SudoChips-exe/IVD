import React, { FC } from 'react'
import { detectPlatform } from '../utils/urlDetection'
import { 
  LinkIcon, 
  ClipboardIcon, 
  InstagramLogo, 
  TikTokLogo, 
  YouTubeLogo, 
  TwitterLogo, 
  FacebookLogo, 
  SnapchatLogo 
} from './Icons'
import '../styles/components.css'

interface URLInputProps {
  value: string
  onChange: (value: string) => void
  onSubmit: () => void
  disabled?: boolean
}

const getPlatformConfig = (platform: string) => {
  switch (platform) {
    case 'Instagram':
      return { className: 'instagram', logo: <InstagramLogo /> }
    case 'TikTok':
      return { className: 'tiktok', logo: <TikTokLogo /> }
    case 'YouTube':
      return { className: 'youtube', logo: <YouTubeLogo /> }
    case 'Twitter':
      return { className: 'twitter', logo: <TwitterLogo /> }
    case 'Facebook':
      return { className: 'facebook', logo: <FacebookLogo /> }
    case 'Snapchat':
      return { className: 'snapchat', logo: <SnapchatLogo /> }
    default:
      return { className: 'unknown', logo: null }
  }
}

const URLInput: FC<URLInputProps> = ({ value, onChange, onSubmit, disabled }) => {
  const platform = value ? detectPlatform(value) : null
  
  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
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

  const platformConfig = platform ? getPlatformConfig(platform) : null

  return (
    <div className="url-input-container">
      <div className="input-wrapper">
        <LinkIcon className="input-icon-left" />
        <input
          type="text"
          placeholder="Paste video URL here (Instagram, TikTok, YouTube, etc.)"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          onKeyDown={handleKeyDown}
          disabled={disabled}
          className="url-input"
        />
        <button 
          className="paste-button"
          onClick={handlePaste}
          disabled={disabled}
          title="Paste from clipboard"
        >
          <ClipboardIcon size={14} /> Paste
        </button>
      </div>
      
      {platform && platform !== 'Unknown' && platformConfig && (
        <div className="platform-indicator">
          <span>Detected:</span>
          <span className={`platform-badge ${platformConfig.className}`}>
            {platformConfig.logo}
            {platform}
          </span>
        </div>
      )}
    </div>
  )
}

export default URLInput
