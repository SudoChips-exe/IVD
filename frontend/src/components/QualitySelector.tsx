import { FC } from 'react'

export type Quality = 'best' | '1080p' | '720p' | '480p' | '360p'

const OPTIONS: { value: Quality; label: string }[] = [
  { value: 'best', label: 'Best' },
  { value: '1080p', label: '1080p' },
  { value: '720p', label: '720p' },
  { value: '480p', label: '480p' },
  { value: '360p', label: '360p' },
]

interface QualitySelectorProps {
  value: Quality
  onChange: (q: Quality) => void
  audioOnly: boolean
  onAudioOnlyChange: (v: boolean) => void
  disabled?: boolean
  isImage?: boolean
}

const QualitySelector: FC<QualitySelectorProps> = ({ value, onChange, audioOnly, onAudioOnlyChange, disabled, isImage }) => {
  if (isImage) return null
  return (
    <div style={{ marginTop: '1rem' }}>
      <span className="input-label">Quality</span>
      <div className="quality-row">
        {OPTIONS.map(opt => (
          <button
            key={opt.value}
            className={`q-btn${value === opt.value && !audioOnly ? ' sel' : ''}`}
            onClick={() => { onAudioOnlyChange(false); onChange(opt.value) }}
            disabled={disabled}
            type="button"
          >
            {opt.label}
          </button>
        ))}
        <button
          className={`q-btn${audioOnly ? ' sel' : ''}`}
          onClick={() => onAudioOnlyChange(!audioOnly)}
          disabled={disabled}
          type="button"
        >
          MP3
        </button>
      </div>
    </div>
  )
}

export default QualitySelector
