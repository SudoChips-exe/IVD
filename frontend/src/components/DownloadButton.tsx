import { DownloadCloud } from './icons';

interface DownloadButtonProps {
  onClick: () => void;
  disabled?: boolean;
  loading?: boolean;
  label?: string;
}

export default function DownloadButton({
  onClick,
  disabled = false,
  loading = false,
  label = 'Download now',
}: DownloadButtonProps) {
  return (
    <button
      type="button"
      className="download-btn"
      onClick={onClick}
      disabled={disabled || loading}
    >
      {loading ? (
        <>
          <span className="spinner" />
          Preparing...
        </>
      ) : (
        <>
          <DownloadCloud />
          {label}
        </>
      )}
    </button>
  );
}
