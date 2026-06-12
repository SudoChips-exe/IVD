//ADEOLA THE DEPRESSED WAS HERE AS THE FRONTEND DEVELOPER FOR THIS PROJECT, I SWEAR
import { useEffect, useRef, useState } from 'react';
import Header from './components/Header';
import URLInput from './components/URLInput';
import VideoPreview from './components/VideoPreview';
import QualitySelector from './components/QualitySelector';
import DownloadButton from './components/DownloadButton';
import DownloadQueueItem from './components/DownloadQueueItem';
import CookieUpload from './components/CookieUpload';
import Footer from './components/Footer';
import { Inbox } from './components/icons';
import Onboarding from './components/Onboarding';
import * as api from './api';
import type { DownloadJob, Quality, Theme, VideoInfo } from './types';

const QUALITY_DETAIL: Record<Quality, string> = {
  best: 'Best mp4',
  '1080p': '1080p mp4',
  '720p': '720p mp4',
  '480p': '480p mp4',
  '360p': '360p mp4',
  mp3: 'MP3 320kbps',
};

function getInitialTheme(): Theme {
  const saved = localStorage.getItem('vidclaw-theme');
  return saved === 'light' ? 'light' : 'dark';
}

export default function App() {
  const [theme, setTheme] = useState<Theme>(getInitialTheme);
  const [url, setUrl] = useState('');
  const [platform, setPlatform] = useState('youtube');
  const [quality, setQuality] = useState<Quality>('best');
  const [info, setInfo] = useState<VideoInfo | null>(null);
  const [fetching, setFetching] = useState(false);
  const [jobs, setJobs] = useState<DownloadJob[]>([]);

  // Keep one simulated-progress timer per job so demo mode animates cleanly.
  const timers = useRef<Record<string, number>>({});

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('vidclaw-theme', theme);
  }, [theme]);

  useEffect(() => {
    const saved = timers.current;
    return () => { Object.values(saved).forEach((t) => window.clearInterval(t)); };
  }, []);

  const toggleTheme = () => setTheme((t) => (t === 'dark' ? 'light' : 'dark'));

  // Build a believable preview without a backend so the design stays reviewable.
  const demoInfo = (link: string): VideoInfo => {
    let source = '';
    try { source = new URL(link).hostname.replace(/^www\./, ''); } catch { source = `${platform}.com`; }
    return {
      id: String(Date.now()),
      title: 'Building a premium dark UI from scratch',
      thumbnail: 'https://images.unsplash.com/photo-1492619375914-88005aa9e8fb?q=80&w=600&auto=format&fit=crop',
      duration: '12:48',
      author: 'Polished Studio',
      meta: '1.2M views, 2 weeks ago',
      source,
    };
  };

  const handleFetch = async () => {
    if (!url.trim() || fetching) return;
    setFetching(true);
    try {
      setInfo(await api.fetchVideoInfo(url.trim()));
    } catch {
      setInfo(demoInfo(url.trim()));
    } finally {
      setFetching(false);
    }
  };

  const simulateProgress = (id: string) => {
    timers.current[id] = window.setInterval(() => {
      setJobs((prev) => prev.map((j) => {
        if (j.id !== id) return j;
        const next = Math.min(100, j.progress + Math.random() * 14 + 4);
        if (next >= 100) {
          window.clearInterval(timers.current[id]);
          delete timers.current[id];
          return { ...j, progress: 100, status: 'completed', detail: `${QUALITY_DETAIL[j.quality]}, done` };
        }
        const speed = (12 + Math.random() * 10).toFixed(1);
        return { ...j, progress: next, status: 'downloading', detail: `${Math.round(next)}% at ${speed} MB/s` };
      }));
    }, 600);
  };

  const handleDownload = async () => {
    const current = info ?? (url.trim() ? demoInfo(url.trim()) : null);
    if (!current) { handleFetch(); return; }

    const id = `${Date.now()}`;
    const job: DownloadJob = {
      id,
      title: current.title,
      thumbnail: current.thumbnail,
      quality,
      status: 'queued',
      progress: 0,
      detail: `${QUALITY_DETAIL[quality]}, waiting`,
    };
    setJobs((prev) => [job, ...prev]);

    try {
      await api.startDownload(url.trim(), quality);
    } catch {
      // No backend in demo mode. The simulated progress below stands in.
    }
    simulateProgress(id);
  };

  const handleCancel = (id: string) => {
    window.clearInterval(timers.current[id]);
    delete timers.current[id];
    api.cancelDownload(id).catch(() => undefined);
    setJobs((prev) => prev.map((j) => (
      j.id === id ? { ...j, status: 'error', detail: 'Cancelled' } : j
    )));
  };

  const handleRemove = (id: string) => {
    window.clearInterval(timers.current[id]);
    delete timers.current[id];
    setJobs((prev) => prev.filter((j) => j.id !== id));
  };

  const handleCookies = (file: File) => {
    api.uploadCookies(file).catch(() => undefined);
  };

  return (
    <>
      <Onboarding />
      <div className="aurora" />
      <div className="grid-bg" />

      <div className="shell">
        <Header theme={theme} onToggleTheme={toggleTheme} />

        <section className="hero">
          <span className="eyebrow"><span className="dot" /> Fast. Private. No watermarks.</span>
          <h1>Download any video.<br /><span className="grad">Effortlessly.</span></h1>
          <p>Paste a link from YouTube, TikTok, Instagram and more. Pick your quality. Done.</p>
        </section>

        <div className="stack">
          <URLInput
            value={url}
            onChange={setUrl}
            onSubmit={handleFetch}
            activePlatform={platform}
            onSelectPlatform={setPlatform}
          />

          {info && <VideoPreview info={info} />}

          <QualitySelector value={quality} onChange={setQuality} />

          <DownloadButton
            onClick={handleDownload}
            loading={fetching}
            label={info ? 'Download now' : 'Fetch and download'}
          />

          <div data-screen-label="DownloadQueue">
            <div className="section-label">Download queue</div>
            {jobs.length === 0 ? (
              <div className="card queue-empty">
                <Inbox />
                <div>Your downloads will appear here.</div>
              </div>
            ) : (
              <div className="queue">
                {jobs.map((job) => (
                  <DownloadQueueItem
                    key={job.id}
                    job={job}
                    onCancel={handleCancel}
                    onRemove={handleRemove}
                  />
                ))}
              </div>
            )}
          </div>

          <CookieUpload onUpload={handleCookies} />
        </div>

        <Footer />
      </div>
    </>
  );
}
