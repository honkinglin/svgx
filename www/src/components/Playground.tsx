import { useState, useMemo, useDeferredValue } from 'react';
import { optimize } from 'svgtidy';
import { Copy, Check, FileCode, Image as ImageIcon } from 'lucide-react';
import './Playground.css';

const DEFAULT_SVG = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <!-- This is a comment that will be removed -->
  <rect x="10" y="10" width="80" height="80" fill="red"/>
  <circle cx="50" cy="50" r="20" fill="blue" />
</svg>`;

export function Playground() {
  const [input, setInput] = useState(DEFAULT_SVG);
  const deferredInput = useDeferredValue(input);
  const [copied, setCopied] = useState(false);
  const [viewMode, setViewMode] = useState<'preview' | 'code'>('preview');

  const { output, error } = useMemo(() => {
    if (!deferredInput.trim()) {
      return { output: '', error: null };
    }
    try {
      const result = optimize(deferredInput);
      return { output: result, error: null };
    } catch (err) {
      console.error(err);
      return { output: '', error: "Failed to optimize SVG. Ensure input is valid XML." };
    }
  }, [deferredInput]);

  const handleCopy = () => {
    navigator.clipboard.writeText(output);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const stats = {
    original: new Blob([deferredInput]).size,
    optimized: new Blob([output]).size,
  };
  const savings = stats.original > 0 
    ? ((stats.original - stats.optimized) / stats.original * 100).toFixed(1) 
    : 0;

  return (
    <section id="playground" className="playground-section">
      <div className="container">
        <div className="section-header">
           <h2>Interactive Playground</h2>
           <p>Paste your SVG below and see the WASM optimizer in action.</p>
        </div>

        <div className="playground-grid">
          {/* Input Panel */}
          <div className="panel input-panel">
            <div className="panel-header">
              <span className="label">Input SVG</span>
              <span className="badge">{stats.original} bytes</span>
            </div>
            <textarea 
              className="editor" 
              value={input} 
              onChange={(e) => setInput(e.target.value)}
              placeholder="Paste SVG code here..."
              spellCheck={false}
            />
          </div>

          {/* Output Panel */}
          <div className="panel output-panel">
            <div className="panel-header">
              <span className="label">Optimized SVG</span>
              <div className="actions">
                 <span className="badge success">
                   {stats.optimized} bytes (-{savings}%)
                 </span>
                 <div className="view-toggles">
                    <button 
                        className={`action-btn ${viewMode === 'preview' ? 'active' : ''}`}
                        onClick={() => setViewMode('preview')}
                        title="Preview"
                    >
                        <ImageIcon size={16} />
                    </button>
                    <button 
                        className={`action-btn ${viewMode === 'code' ? 'active' : ''}`}
                        onClick={() => setViewMode('code')}
                        title="View Code"
                    >
                        <FileCode size={16} />
                    </button>
                 </div>
                 <button className="action-btn copy-btn" onClick={handleCopy} title="Copy to Clipboard">
                   {copied ? <Check size={16} /> : <Copy size={16} />}
                 </button>
              </div>
            </div>
            
            <div className="preview-area">
                {error ? (
                    <div className="error-message">{error}</div>
                ) : (
                    viewMode === 'preview' ? (
                        <div 
                            className="preview-canvas"
                            dangerouslySetInnerHTML={{ __html: output }} 
                        />
                    ) : (
                        <textarea 
                            className="editor readonly" 
                            value={output} 
                            readOnly 
                        />
                    )
                )}
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
