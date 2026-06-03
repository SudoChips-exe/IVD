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
  disabled?: boolean
}

const QualitySelector: FC<QualitySelectorProps> = ({ value, onChange, disabled }) => (
  <div className="quality-selector">
    <span className="quality-label">Quality</span>
    <div className="quality-options">
      {OPTIONS.map(opt => (
        <button
          key={opt.value}
          className={`quality-btn${value === opt.value ? ' active' : ''}`}
          onClick={() => onChange(opt.value)}
          disabled={disabled}
          type="button"
        >
          {opt.label}
        </button>
      ))}
    </div>
  </div>
)

export default QualitySelector
