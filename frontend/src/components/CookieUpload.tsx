import { useRef, useState } from 'react';
import { Cookie, Chevron, Upload, Check } from './icons';

interface CookieUploadProps {
  onUpload: (file: File) => void;
}

export default function CookieUpload({ onUpload }: CookieUploadProps) {
  const [open, setOpen] = useState(false);
  const [drag, setDrag] = useState(false);
  const [fileName, setFileName] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleFile = (file: File | undefined) => {
    if (!file) return;
    setFileName(file.name);
    onUpload(file);
  };

  return (
    <div className={`card cookie-card${open ? ' open' : ''}`} data-screen-label="CookieUpload">
      <div className="cookie-head" onClick={() => setOpen((v) => !v)}>
        <div className="cookie-ico"><Cookie /></div>
        <div className="ct">
          <h4>Instagram authentication</h4>
          <p>Upload cookies.txt to download private or age-gated content</p>
        </div>
        <Chevron className="chev" />
      </div>

      <div className="cookie-body">
        <div>
          <div
            className={`dropzone${drag ? ' drag' : ''}${fileName ? ' loaded' : ''}`}
            onClick={() => inputRef.current?.click()}
            onDragOver={(e) => { e.preventDefault(); setDrag(true); }}
            onDragLeave={() => setDrag(false)}
            onDrop={(e) => {
              e.preventDefault();
              setDrag(false);
              handleFile(e.dataTransfer.files[0]);
            }}
          >
            {fileName ? <Check /> : <Upload />}
            <div className="dz-t">
              {fileName ? (
                <>Loaded <b>{fileName}</b></>
              ) : (
                <>Drop <b>cookies.txt</b> or click to browse</>
              )}
            </div>
            <div className="dz-s">netscape format, stored locally only</div>
          </div>
          <input
            ref={inputRef}
            type="file"
            accept=".txt"
            hidden
            onChange={(e) => handleFile(e.target.files?.[0])}
          />
        </div>
      </div>
    </div>
  );
}
