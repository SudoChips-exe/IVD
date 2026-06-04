# VIDCLAW — Setup & Project Reference

Full-stack video downloader. Paste a URL → preview → download MP4 or MP3.
Built with Rust + Actix-web backend and React + TypeScript frontend.

## Project Status: Deployed ✅

**Last Updated**: June 4, 2026

---

## ✅ Completed

### Backend
- Rust + Actix-web 4.x server
- yt-dlp subprocess for all platform downloads (no API keys needed)
- `POST /api/info` — fetch video metadata before download (title, thumbnail, duration, filesize, platform)
- Job-based download system with SSE progress streaming
- Real-time progress: percent, speed, ETA streamed from yt-dlp stdout
- Audio-only download — extracts MP3 via yt-dlp `--extract-audio`
- Download cancellation — kills yt-dlp child process on request
- Quality selector — 1080p / 720p / 480p / 360p / Best
- Browser cookie fallback chain (Chrome → Chromium → Firefox → Brave → Edge)
- Cookie file support (`~/.config/vidclaw/cookies.txt`) for Instagram/Facebook
- Cookie upload endpoint (`POST /api/cookies`) — upload via browser UI
- Session cookie env vars (`INSTAGRAM_SESSION_ID`, `FACEBOOK_SESSION_COOKIES`)
- Proxy support via `YTDLP_PROXY` env var
- Per-IP + global rate limiting middleware
- Smart error classification (private, geo-blocked, copyright, auth, no-video)
- Docker deployment (nginx + Rust binary + yt-dlp + ffmpeg in single container)
- Render deployment config (`render.yaml`)

### Frontend
- React + TypeScript + Vite
- Video preview card — thumbnail, title, uploader, duration, filesize, platform badge (debounced 900ms after URL paste)
- Multi-download queue — start multiple downloads simultaneously; each has independent SSE progress
- Audio-only toggle (MP3) — next to quality selector
- Download history — localStorage, persists across sessions, with clear button
- Platform detection badge (Instagram, TikTok, YouTube, Twitter, Facebook)
- Real SSE progress bar with speed + ETA per download item
- Cancel button per download item
- Quality selector pill buttons
- Cookie upload UI (Settings section)
- PWA — installable, service worker, offline UI
- Scroll-spy URL updates as user scrolls
- Responsive — mobile first

### Tests
- Backend: 57 tests (22 unit + 22 unit-bin + 13 integration) — all passing
- Frontend: 33 tests (urlDetection + useDownload SSE flow + PWA manifest) — all passing

---

## Quick Start

### Local Development

```bash
# Both backend + frontend from root
bun install
bun run dev
# Backend: http://localhost:8080
# Frontend: http://localhost:5173
```

Or separately:

```bash
# Terminal 1 — backend
cd backend && cargo run

# Terminal 2 — frontend
cd frontend && bun install && bun run dev
```

### yt-dlp venv (recommended — needed for TikTok locally)

```bash
bash scripts/setup_venv.sh
```

Creates venv at `~/.local/share/vidclaw/venv` with `curl-cffi` for TikTok JS challenge impersonation. Backend auto-detects it.

### Run Tests

```bash
bun run test                  # frontend (33 tests)
cd backend && cargo test      # backend (57 tests)
```

### Build for Production

```bash
bun run build                 # frontend → frontend/dist/
cd backend && cargo build --release
```

---

## Project Structure

```
IVD/
├── SETUP.md              ← this file
├── TODO.md               ← pending tasks
├── CONTEXT.md            ← full context for resuming on another machine
├── Dockerfile            ← single container: nginx + Rust + yt-dlp + ffmpeg
├── render.yaml           ← Render deployment config
├── nginx.conf            ← nginx template ($PORT substituted at runtime)
├── entrypoint.sh         ← startup: decode cookies, configure nginx, start server
├── docker-compose.yml    ← local Docker testing
│
├── backend/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs           # server setup, app state
│       ├── jobs.rs           # job store, JobEvent enum, cancel flag
│       ├── config.rs         # env var config
│       ├── error.rs          # AppError → HTTP status
│       ├── models.rs         # DownloadRequest, VideoInfo, Platform enum
│       ├── util.rs           # validate_url, detect_platform (test-only)
│       ├── cache.rs          # metadata cache (reserved)
│       ├── api/ytdlp.rs      # yt-dlp subprocess, progress parsing, get_info
│       ├── handlers/
│       │   ├── info.rs       # POST /api/info → VideoInfo
│       │   ├── download.rs   # POST /api/download → { job_id }
│       │   ├── progress.rs   # GET /api/progress/{job_id} — SSE
│       │   ├── file_delivery.rs # GET /api/file/{job_id} — binary stream
│       │   ├── cancel.rs     # POST /api/cancel/{job_id}
│       │   ├── cookies.rs    # POST/DELETE /api/cookies, GET /api/cookies/status
│       │   └── health.rs     # GET /api/health
│       └── middleware/rate_limit.rs
│
└── frontend/
    ├── package.json
    ├── vite.config.ts
    └── src/
        ├── App.tsx
        ├── types/index.ts        # VideoInfo, HistoryEntry, Platform
        ├── services/api.ts       # axios client, all endpoints
        ├── hooks/
        │   ├── useDownloadQueue.ts  # multi-download queue with SSE per item
        │   ├── useVideoInfo.ts      # debounced info fetch + error state
        │   ├── useDownload.ts       # legacy single-download hook (kept for tests)
        │   └── useScrollSpy.ts
        ├── components/
        │   ├── VideoPreview.tsx     # thumbnail + title + meta before download
        │   ├── DownloadQueueItem.tsx # single item in download queue
        │   ├── QualitySelector.tsx  # quality pills + MP3 toggle
        │   ├── URLInput.tsx
        │   ├── DownloadButton.tsx
        │   ├── ProgressBar.tsx
        │   ├── CookieUpload.tsx
        │   ├── Header.tsx
        │   ├── Footer.tsx
        │   ├── ErrorMessage.tsx
        │   └── Icons.tsx
        ├── utils/
        │   ├── history.ts       # localStorage: get/save/clear download history
        │   └── urlDetection.ts  # detectPlatform, isValidUrl
        ├── styles/
        │   ├── App.css          # layout, hero, features, FAQ sections
        │   └── components.css   # all component styles
        └── test/
```

---

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/info` | Fetch video metadata → `VideoInfo` |
| POST | `/api/download` | Start download → `{ job_id }` |
| GET | `/api/progress/{job_id}` | SSE stream (progress / done / error / cancelled) |
| GET | `/api/file/{job_id}` | Download binary after `done` event |
| POST | `/api/cancel/{job_id}` | Cancel running download |
| POST | `/api/cookies` | Upload cookies.txt (plain text body) |
| DELETE | `/api/cookies` | Remove stored cookies |
| GET | `/api/cookies/status` | `{ active: bool }` |
| GET | `/api/health` | Health check |

### SSE Event Types

```json
{ "type": "progress", "percent": 42.0, "speed": "3.2MB/s", "eta": "0:12" }
{ "type": "authenticating", "method": "cookies file" }
{ "type": "merging" }
{ "type": "done", "filename": "video.mp4" }
{ "type": "cancelled" }
{ "type": "error", "message": "This video is private." }
```

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_HOST` | `0.0.0.0` | Backend bind address |
| `SERVER_PORT` | `8080` | Internal backend port |
| `FRONTEND_URL` | `http://localhost:5173` | CORS origin |
| `RUST_LOG` | `info` | Log level |
| `YTDLP_PATH` | *(auto)* | Override yt-dlp binary |
| `YTDLP_VENV` | `~/.local/share/vidclaw/venv` | Override venv path |
| `YTDLP_PROXY` | — | Residential proxy (required for Instagram/Facebook on VPS) |
| `INSTAGRAM_SESSION_ID` | — | Instagram sessionid cookie value |
| `FACEBOOK_SESSION_COOKIES` | — | Facebook cookies as `key=value;key=value` |
| `COOKIES_B64` | — | Base64-encoded cookies.txt (Docker/Render) |
| `MAX_REQUESTS_PER_MINUTE` | `60` | Global rate limit |
| `MAX_REQUESTS_PER_IP_PER_MINUTE` | `30` | Per-IP rate limit |
| `PORT` | `80` | Public port (Render injects this) |
| `VITE_API_URL` | `http://localhost:8080` | Frontend API base (empty = relative in Docker) |

---

## Platform Support

| Platform | Local | Render/VPS | Notes |
|----------|-------|------------|-------|
| YouTube | ✅ | ✅ | No auth needed |
| Twitter/X | ✅ | ✅ | HLS → MP4 via ffmpeg |
| TikTok | ⚠️ | ✅ | Local: run `scripts/setup_venv.sh` |
| Instagram | ❌ | ❌ | Needs cookies + residential proxy |
| Facebook | ❌ | ❌ | Needs cookies + residential proxy |

---

## Deployment (Render)

```bash
# 1. Push to GitHub
git push origin main

# 2. render.com → New Web Service → Connect repo
# 3. Render auto-detects render.yaml
# 4. Set in Render dashboard → Environment:
#    COOKIES_B64=<output of: base64 -w0 cookies.txt>
#    YTDLP_PROXY=http://user:pass@proxy.webshare.io:80
#    RUST_LOG=info
# 5. Deploy — first build ~8 min (Rust compile)
```

See `TODO.md` for Instagram/Facebook setup steps.
