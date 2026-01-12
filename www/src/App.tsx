import { Suspense } from 'react';
import { Header } from './components/Header';
import { Hero } from './components/Hero';
import { Playground } from './components/Playground';

function App() {
  return (
    <>
      <Header />
      <main>
        <Hero />
        <Suspense fallback={<div className="container" style={{padding: '40px', textAlign: 'center'}}>Loading WASM module...</div>}>
          <Playground />
        </Suspense>
      </main>
      <footer style={{
        textAlign: 'center', 
        padding: '40px', 
        color: 'var(--color-text-muted)',
        borderTop: '1px solid var(--color-border)',
        marginTop: 'auto'
      }}>
        <div className="container">
          <p>© {new Date().getFullYear()} SvgTidy. MIT License.</p>
        </div>
      </footer>
    </>
  )
}

export default App
