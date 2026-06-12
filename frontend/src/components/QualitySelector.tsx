import type { Quality, QualityOption } from '../types';
import { Check } from './icons';

const OPTIONS: QualityOption[] = [
  { id: 'best', name: 'Best', sub: 'auto, highest' },
  { id: '1080p', name: '1080p', sub: 'full hd mp4' },
  { id: '720p', name: '720p', sub: 'hd mp4' },
  { id: '480p', name: '480p', sub: 'sd mp4' },
  { id: '360p', name: '360p', sub: 'low mp4' },
  { id: 'mp3', name: 'MP3', sub: '320 kbps', tag: 'AUDIO' },
];

interface QualitySelectorProps {
  value: Quality;
  onChange: (quality: Quality) => void;
}

export default function QualitySelector({ value, onChange }: QualitySelectorProps) {
  return (
    <div className="card url-card" data-screen-label="QualitySelector">
      <div className="section-label">Select quality</div>
      <div className="quality-grid">
        {OPTIONS.map((opt) => (
          <button
            key={opt.id}
            type="button"
            className={`q-opt${value === opt.id ? ' active' : ''}`}
            onClick={() => onChange(opt.id)}
          >
            <Check className="q-check" />
            <div className="q-name">
              {opt.name}
              {opt.tag && <span className="q-tag">{opt.tag}</span>}
            </div>
            <div className="q-sub">{opt.sub}</div>
          </button>
        ))}
      </div>
    </div>
  );
}
