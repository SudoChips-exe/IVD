import { Link, Clipboard } from './Icons';

interface PlatformChip {
  id: string;
  label: string;
  color: string;
}

const PLATFORMS: PlatformChip[] = [
  { id: 'youtube', label: 'YouTube', color: '#ff4d4d' },
  { id: 'instagram', label: 'Instagram', color: '#e1306c' },
  { id: 'tiktok', label: 'TikTok', color: '#9aa6c2' },
  { id: 'twitter', label: 'X / Twitter', color: '#60a5fa' },
  { id: 'facebook', label: 'Facebook', color: '#3b82f6' },
  { id: 'vimeo', label: 'Vimeo', color: '#4ade80' },
];

interface URLInputProps {
  value: string;
  onChange: (value: string) => void;
  onSubmit: () => void;
  activePlatform: string;
  onSelectPlatform: (id: string) => void;
}

export default function URLInput({
  value,
  onChange,
  onSubmit,
  activePlatform,
  onSelectPlatform,
}: URLInputProps) {
  const handlePaste = async () => {
    try {
      const text = await navigator.clipboard.readText();
      if (text) onChange(text.trim());
    } catch {
      // Clipboard access can be blocked. Fail quietly and let the user type.
    }
  };

  return (
    <div className="card url-card" data-screen-label="URLInput">
      <div className="url-row">
        <Link className="link-ico" />
        <input
          type="text"
          value={value}
          spellCheck={false}
          placeholder="https://youtube.com/watch?v=..."
          onChange={(e) => onChange(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter') onSubmit(); }}
        />
        <button className="paste-btn" onClick={handlePaste} type="button">
          <Clipboard />
          Paste
        </button>
      </div>

      <div className="chips">
        {PLATFORMS.map((p) => (
          <button
            key={p.id}
            type="button"
            className={`chip${activePlatform === p.id ? ' active' : ''}`}
            onClick={() => onSelectPlatform(p.id)}
          >
            <span className="pdot" style={{ background: p.color }} />
            {p.label}
          </button>
        ))}
      </div>
    </div>
  );
}
