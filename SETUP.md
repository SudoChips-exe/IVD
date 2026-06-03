# VIDCLAW — Video Downloader

Full-stack video downloader. Paste a URL → downloads MP4. Built with Rust + Actix-web backend and React + TypeScript frontend.

## Project Status: Phase 5 — Deployment Ready ✅

**Last Updated**: June 3, 2026

---

## ✅ Completed

### Backend
- Rust + Actix-web 4.x server
- yt-dlp subprocess for all platform downloads (no API keys needed)
- Job-based download system with SSE progress streaming
- Real-time progress: percent, speed, ETA streamed from yt-dlp stdout
- Download cancellation — kills yt-dlp child process on request
- Quality selector — 1080p / 720p / 480p / 360p / Best
- Browser cookie fallback chain (Chrome → Chromium → Firefox → Brave → Edge)
- Cookie file support (`~/.config/vidclaw/cookies.txt`) for Instagram/Facebook
- Cookie upload endpoint (`POST /api/cookies`) — upload via browser UI
- Proxy support via `YTDLP_PROXY` env var
- Per-IP + global rate limiting middleware
- Smart error classification (private, geo-blocked, copyright, auth, no-video)
- Docker deployment (nginx + Rust binary + yt-dlp + ffmpeg in single container)
- Render deployment config (`render.yaml`)

### Frontend
- React + TypeScript + Vite
- Platform detection badge (Instagram, TikTok, YouTube, Twitter, Facebook)
- Real SSE progress bar with speed + ETA
- Cancel button during download
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
# Terminal 1 — backend
cd backend && cargo run
# Runs on http://localhost:8080

# Terminal 2 — frontend
cd frontend && bun install && bun run dev
# Runs on http://localhost:5173
```

Or both from root:
```bash
bun run dev
```

### Run Tests

```bash
# All tests from root
bun run test          # frontend (33 tests)
cd backend && cargo test  # backend (57 tests)
```

### Build for Production

```bash
bun run build         # builds frontend → frontend/dist/
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
│   ├── src/
│   │   ├── main.rs           # server setup, app state
│   │   ├── lib.rs            # module exports
│   │   ├── jobs.rs           # job store, JobEvent enum, cancel flag
│   │   ├── config.rs         # env var config
│   │   ├── error.rs          # AppError → HTTP status
│   │   ├── models.rs         # DownloadRequest, Platform enum
│   │   ├── util.rs           # validate_url, detect_platform (test-only)
│   │   ├── cache.rs          # metadata cache (reserved)
│   │   ├── api/ytdlp.rs      # yt-dlp subprocess, progress parsing, cancel
│   │   ├── handlers/
│   │   │   ├── download.rs   # POST /api/download → { job_id }
│   │   │   ├── progress.rs   # GET /api/progress/{job_id} — SSE
│   │   │   ├── file_delivery.rs # GET /api/file/{job_id} — binary
│   │   │   ├── cancel.rs     # POST /api/cancel/{job_id}
│   │   │   ├── cookies.rs    # POST/DELETE /api/cookies, GET /api/cookies/status
│   │   │   └── health.rs     # GET /api/health
│   │   └── middleware/rate_limit.rs
│   └── tests/integration.rs
│
└── frontend/
    ├── package.json
    ├── vite.config.ts        # Vite + PWA plugin config
    ├── tsconfig.json
    ├── index.html
    └── src/
        ├── App.tsx
        ├── hooks/
        │   ├── useDownload.ts   # SSE-based download + cancel
        │   └── useScrollSpy.ts  # URL updates on scroll
        ├── components/
        │   ├── URLInput.tsx
        │   ├── QualitySelector.tsx
        │   ├── DownloadButton.tsx
        │   ├── ProgressBar.tsx  # shows speed + ETA
        │   ├── CookieUpload.tsx # cookie file upload UI
        │   ├── Header.tsx
        │   ├── Footer.tsx
        │   ├── ErrorMessage.tsx
        │   └── Icons.tsx
        ├── services/api.ts      # axios client, all endpoints
        ├── utils/urlDetection.ts
        ├── types/index.ts
        └── test/                # urlDetection, useDownload, pwa tests
```

---

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/download` | Start download → `{ job_id }` |
| GET | `/api/progress/{job_id}` | SSE stream (progress/done/error/cancelled) |
| GET | `/api/file/{job_id}` | Fetch binary after done event |
| POST | `/api/cancel/{job_id}` | Cancel running download |
| POST | `/api/cookies` | Upload cookies.txt content (plain text body) |
| DELETE | `/api/cookies` | Remove stored cookies |
| GET | `/api/cookies/status` | `{ active: bool }` |
| GET | `/api/health` | Health check |

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_HOST` | `0.0.0.0` | Backend bind address |
| `SERVER_PORT` | `8080` | Internal backend port |
| `FRONTEND_URL` | `http://localhost:5173` | CORS origin |
| `RUST_LOG` | `info` | Log level |
| `COOKIES_B64` | — | Base64-encoded cookies.txt (Docker/Render) |
| `YTDLP_PROXY` | — | Residential proxy URL for blocked platforms |
| `PORT` | `80` | Public port (Render injects this) |
| `VITE_API_URL` | `http://localhost:8080` | Frontend API base (empty = relative in Docker) |

---

## Platform Support

| Platform | Local | Render/VPS | Notes |
|----------|-------|------------|-------|
| YouTube | ✅ | ✅ | No auth needed |
| Twitter/X | ✅ | ✅ | HLS → MP4 via ffmpeg |
| TikTok | ⚠️ | ✅ | Local: `sudo pip install curl-cffi --break-system-packages` |
| Instagram | ❌ | ❌ | Needs service account cookies + residential proxy |
| Facebook | ❌ | ❌ | Needs service account cookies + residential proxy |

---

## Deployment (Render)

```bash
# 1. Push to GitHub
# 2. render.com → New Web Service → connect repo
# 3. Render auto-detects render.yaml
# 4. Set env vars in Render dashboard:
#    COOKIES_B64=<base64 of cookies.txt>
#    YTDLP_PROXY=http://user:pass@proxy.webshare.io:80
# 5. Deploy (first build ~5-10 min, Rust compile slow)
```

See `TODO.md` for Instagram/Facebook fix steps before deploying.
