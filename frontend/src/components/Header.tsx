import React from 'react'
import { LogoIcon } from './Icons'
import '../styles/components.css'

const Header: React.FC = () => {
  return (
    <header className="header">
      <div className="header-content">
        <a href="#home" className="logo-container">
          <LogoIcon className="logo-icon" size={22} />
          <span className="logo logo-full">Universal Video Downloader</span>
          <span className="logo logo-short">UVD</span>
        </a>
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
