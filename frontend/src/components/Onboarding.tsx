import { useEffect, useState } from 'react';

interface Step {
  title: string;
  body: string;
  emoji: string;
}

const STEPS: Step[] = [
  {
    emoji: '📋',
    title: 'Paste your link',
    body: 'Copy any video URL from YouTube, TikTok, Instagram, Twitter or Facebook and paste it in the bar.',
  },
  {
    emoji: '🎯',
    title: 'Pick your quality',
    body: 'Choose from Best, 1080p, 720p, 480p or 360p. Want audio only? Hit MP3.',
  },
  {
    emoji: '⚡',
    title: 'Hit Download',
    body: 'Your download starts instantly and shows live progress in the queue below.',
  },
  {
    emoji: '🍪',
    title: 'Instagram not working?',
    body: 'Upload a cookies.txt file from your browser. VIDCLAW uses it to authenticate on your behalf.',
  },
];

const KEY = 'vidclaw-onboarded';

export default function Onboarding() {
  const [visible, setVisible] = useState(false);
  const [step, setStep] = useState(0);
  const [exiting, setExiting] = useState(false);

  useEffect(() => {
    if (!localStorage.getItem(KEY)) {
      const t = setTimeout(() => setVisible(true), 700);
      return () => clearTimeout(t);
    }
  }, []);

  const dismiss = () => {
    setExiting(true);
    setTimeout(() => {
      localStorage.setItem(KEY, '1');
      setVisible(false);
      setExiting(false);
    }, 300);
  };

  const next = () => {
    if (step < STEPS.length - 1) {
      setStep((s) => s + 1);
    } else {
      dismiss();
    }
  };

  const prev = () => setStep((s) => Math.max(0, s - 1));

  if (!visible) return null;

  const current = STEPS[step];
  const isLast = step === STEPS.length - 1;

  return (
    <div
      className={`ob-backdrop${exiting ? ' ob-exit' : ''}`}
      onClick={(e) => { if (e.target === e.currentTarget) dismiss(); }}
    >
      <div className="ob-card">
        <button className="ob-close" onClick={dismiss} aria-label="Skip guide">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>

        <div className="ob-pips">
          {STEPS.map((_, i) => (
            <div
              key={i}
              className={`ob-pip${i === step ? ' ob-pip-active' : i < step ? ' ob-pip-done' : ''}`}
            />
          ))}
        </div>

        <div className="ob-body">
          <div className="ob-emoji" key={step}>{current.emoji}</div>
          <h2 className="ob-title">{current.title}</h2>
          <p className="ob-text">{current.body}</p>
        </div>

        <div className="ob-actions">
          {step > 0 && (
            <button className="ob-btn-ghost" onClick={prev}>Back</button>
          )}
          <button className="ob-btn-primary" onClick={next}>
            {isLast ? 'Got it' : 'Next'}
          </button>
        </div>

        <div className="ob-counter">{step + 1} of {STEPS.length}</div>
      </div>
    </div>
  );
}
