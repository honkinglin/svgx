import { Github } from 'lucide-react';
import './Header.css';

export function Header() {
  return (
    <header className="header">
      <div className="container header-inner">
        <div className="logo-section">
          <img src="/logo.svg" alt="SvgTidy Logo" className="logo-img" />
        </div>
        <a 
          href="https://github.com/honkinglin/svgtidy" 
          target="_blank" 
          rel="noreferrer"
          className="github-link"
        >
          <Github size={24} />
        </a>
      </div>
    </header>
  );
}
