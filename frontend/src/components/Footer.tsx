import React from 'react'
import '../styles/components.css'

const Footer: React.FC = () => {
  return (
    <footer className="footer">
      <div className="footer-content">
        <p>&copy; 2026 Universal Video Downloader. All rights reserved.</p>
        <p>
          <span>⚖️ Legal Disclaimer:</span> Download only content you have permission to download. 
          Respect copyright and platform terms of service.
        </p>
        <div className="footer-links">
          <a href="#privacy">Privacy Policy</a>
          <a href="#terms">Terms of Service</a>
          <a href="#contact">Contact Us</a>
        </div>
      </div>
    </footer>
  )
}

export default Footer
