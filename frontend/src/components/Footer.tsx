import { ClawMark } from './icons';

export default function Footer() {
  const year = new Date().getFullYear();
  return (
    <footer className="site-footer">
      <div className="fl">
        <span className="mini"><ClawMark /></span>
        <span>VIDCLAW &copy; {year}. Built for speed</span>
      </div>
      <nav>
        <a href="#privacy">Privacy</a>
        <a href="#terms">Terms</a>
        <a href="#github">GitHub</a>
      </nav>
    </footer>
  );
}
