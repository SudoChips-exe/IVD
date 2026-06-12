import type { Theme } from '../types';
import { ClawMark, Moon, Sun } from './icons';

interface HeaderProps {
  theme: Theme;
  onToggleTheme: () => void;
}

export default function Header({ theme, onToggleTheme }: HeaderProps) {
  return (
    <header className="site-header" data-screen-label="Header">
      <div className="brand">
        <div className="brand-mark">
          <ClawMark />
        </div>
        <div>
          <div className="wordmark">VID<b>CLAW</b></div>
          <div className="brand-sub">video downloader</div>
        </div>
      </div>

      <button className="theme-toggle" onClick={onToggleTheme} aria-label="Toggle color theme">
        {theme === 'dark' ? <Moon /> : <Sun />}
        <span>{theme === 'dark' ? 'Dark' : 'Light'}</span>
      </button>
    </header>
  );
}
