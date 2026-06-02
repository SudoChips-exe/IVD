import React from 'react'
import '../styles/components.css'

const Footer: React.FC = () => {
  return (
    <footer className="footer" id="contact">
      <div className="footer-content">
        <p>&copy; {new Date().getFullYear()} Universal Video Downloader. All rights reserved.</p>
        <p className="legal-disclaimer">
          <span>Disclaimer:</span> Please download videos only for personal offline viewing or if you have explicit permission from the copyright owner. Respect all platform terms of service.
        </p>
        <div className="footer-links">
          <a href="#privacy">Privacy Policy</a>
          <a href="#terms">Terms & Conditions</a>
          <a href="#contact">Contact</a>
        </div>
      </div>
    </footer>
  )
}

export default Footer
