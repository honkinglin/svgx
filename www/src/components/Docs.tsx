import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import './Docs.css';

// Import raw README content via Vite
import readmeContent from '../../../README.md?raw';

export function Docs() {
// In a real app we might fetch this, but bundling it is fine for now
  const content = readmeContent;

  return (
    <div className="container docs-container">
      <div className="docs-content">
        <ReactMarkdown 
            remarkPlugins={[remarkGfm]}
            components={{
                img: ({...props}) => {
                    return <img {...props} style={{maxWidth: '100%'}} />;
                }
            }}
        >
            {content}
        </ReactMarkdown>
      </div>
    </div>
  );
}
