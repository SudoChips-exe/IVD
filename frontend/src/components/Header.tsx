import React from 'react'
import { LogoIcon } from './Icons'
import '../styles/components.css'

const scrollTo = (id: string) => (e: React.MouseEvent) => {
  e.preventDefault()
  document.getElementById(id)?.scrollIntoView({ behavior: 'smooth' })
}

const Header: React.FC = () => {
  return (
    <header className="header">
      <div className="header-content">
        <a onClick={scrollTo('home')} href="/" className="logo-container">
          <LogoIcon className="logo-icon" size={22} />
          <span className="logo logo-full">VIDCLAW</span>
          <span className="logo logo-short">UVD</span>
        </a>
        <nav className="nav-menu">
          <a onClick={scrollTo('home')} href="/">Home</a>
          <a onClick={scrollTo('features')} href="/">Features</a>
          <a onClick={scrollTo('faq')} href="/">FAQ</a>
          <a onClick={scrollTo('contact')} href="/">Contact</a>
        </nav>
      </div>
    </header>
  )
}

export default Header
