# VIDCLAW — Project Context (Pick Up From Here)

**Last updated:** 2026-06-03  
**Status:** Feature-complete locally. Deployment pending.

---

## What This Is

Full-stack video downloader. Paste a URL from YouTube/TikTok/Twitter/Instagram/Facebook → downloads the video.

- **Backend:** Rust + Actix-web 4.x, uses `yt-dlp` subprocess for all downloads
- **Frontend:** React + TypeScript + Vite, PWA-enabled
- **Deployment target:** Render (free tier), single Docker container (nginx + Rust binary)

---

## Current Platform Status

| Platform | Works Locally | Works on Render | Notes |
|---|---|---|---|
| YouTube | ✅ | ✅ | No auth needed |
| Twitter/X | ✅ | ✅ | HLS → MP4 via ffmpeg |
| TikTok | ❌ (needs fix) | ✅ (Docker has fix) | Needs `sudo pip install curl-cffi --break-system-packages` locally |
| Instagram | ❌ | ❌ | Needs service account cookies + residential proxy |
| Facebook | ❌ | ❌ | Needs service account cookies + residential proxy |

---

## Test Suite

```bash
# Backend (from /backend)
cargo test
# 22 unit + 9 integration = 31 tests, all passing

# Frontend (from /frontend or project root)
bun run test
# 32 tests (urlDetection + useDownload + PWA manifest), all passing
```

---

## How to Run Locally

```bash
# Terminal 1 — backend
cd backend && cargo run
# Runs on http://localhost:8080

# Terminal 2 — frontend
cd frontend && bun run dev
# Runs on http://localhost:5173

# Or both together from root
bun run dev
```

---

## Key Files

```
IVD/
├── CONTEXT.md              ← you are here
├── TODO.md                 ← pending tasks (read this next)
├── Dockerfile              ← single container: nginx + Rust + yt-dlp + ffmpeg
├── render.yaml             ← Render deployment config
├── nginx.conf              ← template, $PORT substituted at runtime
├── entrypoint.sh           ← handles COOKIES_B64 decode + nginx port + backend start
│
├── backend/src/
│   ├── api/ytdlp.rs        ← ALL download logic lives here (yt-dlp subprocess)
│   ├── handlers/download.rs← HTTP POST /api/download handler
│   ├── error.rs            ← AppError enum → HTTP status codes
│   ├── util.rs             ← validate_url, detect_platform (test-only)
│   └── main.rs             ← Actix-web server, CORS, rate limiter
│
└── frontend/src/
    ├── hooks/useDownload.ts ← download state machine, retry logic
    ├── hooks/useScrollSpy.ts← URL updates as user scrolls
    ├── services/api.ts      ← axios client, VITE_API_URL env var
    └── test/               ← urlDetection, useDownload, pwa tests
```

---

## Architecture: How a Download Works

```
User pastes URL
  → POST /api/download { url }
  → validate_url() — rejects empty/too-long/no-protocol
  → ytdlp::extract(url)
      1. Try yt-dlp without cookies (YouTube, TikTok, Twitter work)
      2. Try cookies.txt at ~/.config/vidclaw/cookies.txt if exists
      3. Try browser cookies: chrome → chromium → firefox → brave → edge
      4. classify_hard_error() — fail fast on private/geo/copyright
      5. classify_auth_error() — specific message for auth failures
  → write to /tmp/vidclaw_{uuid}.mp4
  → read file into memory
  → delete temp file
  → stream as video/mp4 with Content-Disposition filename
```

---

## Environment Variables

| Var | Default | Purpose |
|---|---|---|
| `SERVER_HOST` | `0.0.0.0` | Backend bind address |
| `SERVER_PORT` | `8080` | Backend port (internal, nginx proxies to this) |
| `FRONTEND_URL` | `http://localhost:5173` | CORS allowed origin |
| `RUST_LOG` | `info` | Log level |
| `COOKIES_B64` | — | Base64-encoded cookies.txt for Instagram/Facebook |
| `YTDLP_PROXY` | — | Residential proxy URL (not yet wired in code — see TODO.md) |
| `PORT` | `80` | Injected by Render — nginx listens on this |
| `VITE_API_URL` | `http://localhost:8080` | Frontend API base URL (empty = relative in Docker) |

---

## Pending Before Deployment (See TODO.md)

1. **TikTok local fix:** `sudo pip install curl-cffi --break-system-packages`
2. **Instagram/Facebook:**
   - Create throwaway service accounts
   - Export cookies → `~/.config/vidclaw/cookies.txt`
   - Sign up for residential proxy (Webshare ~$3/mo)
   - Add `YTDLP_PROXY` support to `backend/src/api/ytdlp.rs`
3. **Render deploy:**
   - Push to GitHub
   - Connect repo on render.com
   - Set `COOKIES_B64` + `YTDLP_PROXY` env vars in dashboard
   - Update `FRONTEND_URL` in `render.yaml` to actual Render URL

---

## PWA

Fully configured. Icons at `frontend/public/icon-192.png` and `icon-512.png`.  
Replace with proper artwork when available — current icons are placeholders (cyan download arrow, dark bg).

Test PWA install: `bun run preview` → open `http://localhost:4173` in Chrome → look for ⊕ in address bar.

---

## Coding Notes

- yt-dlp format string: `bestvideo[ext=mp4]+bestaudio[ext=m4a]/bestvideo+bestaudio/best[ext=mp4]/best` — covers MP4 (YouTube) and HLS (Twitter)
- `classify_hard_error()` in ytdlp.rs — fails fast without trying all browsers for private/geo/copyright errors
- Frontend retry: 3 retries with exponential backoff (1s, 2s, 3s). Non-retryable: "Invalid URL" / "Please enter a valid URL"
- `useScrollSpy` uses `IntersectionObserver` with `rootMargin: '0px 0px -70% 0px'` — updates URL via `history.replaceState` without page reload
- All dead platform adapters removed (instagram.rs, tiktok.rs, youtube.rs, twitter.rs, facebook.rs, snapchat.rs) — yt-dlp handles everything
