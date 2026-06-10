interface HeaderProps {
  darkMode?: boolean
  onToggleTheme?: () => void
}

export default function Header({ darkMode = true, onToggleTheme }: HeaderProps) {
  return (
    <header className="header">
      <div className="header-left">
        <div className="logo-mark">
          <svg viewBox="0 0 24 24">
            <path d="M19 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z"/>
          </svg>
        </div>
        <div>
          <div className="logo-sub">Video Downloader</div>
          <div className="logo-text">VIDCLAW</div>
        </div>
      </div>
      {onToggleTheme && (
        <div className="toggle-wrap" onClick={onToggleTheme} role="button" aria-label="Toggle theme">
          <div className="toggle-track">
            <div className={`toggle-knob${darkMode ? '' : ' light'}`} />
          </div>
          <span className="toggle-icon">{darkMode ? '🌙' : '☀️'}</span>
          <span className="toggle-label">{darkMode ? 'Dark' : 'Light'}</span>
        </div>
      )}
    </header>
  )
}
