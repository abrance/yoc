import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './styles.css';

function App() {
  return (
    <main className="shell">
      <p className="eyebrow">yoc fixture</p>
      <h1>Hello from React</h1>
      <p>Minimal Vite app for testing reusable React workflows.</p>
    </main>
  );
}

const rootElement = document.getElementById('root');

if (rootElement === null) {
  throw new Error('Root element not found');
}

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
