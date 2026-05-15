import React from 'react'
import '../styles/components.css'

const Header: React.FC = () => {
  return (
    <header className="header">
      <div className="header-content">
        <h1 className="logo">🎬 Video Downloader</h1>
        <nav className="nav-menu">
          <a href="#home">Home</a>
          <a href="#features">Features</a>
          <a href="#faq">FAQ</a>
          <a href="#contact">Contact</a>
        </nav>
      </div>
    </header>
  )
}

export default Header
