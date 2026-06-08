# VIDCLAW

Video and image downloader for Instagram, TikTok, YouTube, Twitter/X, and Facebook.
Paste a URL → preview → download MP4, MP3, or image. No account required for most platforms.

**Stack:** Rust + Actix-web backend · React + TypeScript frontend · yt-dlp subprocess · Docker + Render

---

## Features

- Video, audio (MP3), and image/photo post downloads
- Quality selector — Best / 1080p / 720p / 480p / 360p
- Playlist support — YouTube playlists, TikTok user pages
- Real-time SSE progress (speed, ETA, %)
- Multiple simultaneous downloads with independent queues
- Re-download and copy URL from history
- Cookie-based auth for private/authenticated content
- PWA — installable on Android, iOS, and desktop

---

## Local Development

### Prerequisites

- Rust 1.70+
- Bun (or Node 18+)
- yt-dlp in PATH
- ffmpeg in PATH

### Setup

```bash
# 1. Clone
git clone <repo-url> && cd IVD

# 2. Install frontend deps
bun install

# 3. Configure env
cp backend/.env.example backend/.env
# Edit backend/.env — add COOKIES_B64 and INSTAGRAM_SESSION_ID if you have them

# 4. Run both servers
bun run dev
# Backend: http://localhost:8080
# Frontend: http://localhost:5173
```

### yt-dlp venv (recommended)

TikTok requires `curl-cffi`. The setup script creates an isolated venv:

```bash
bash scripts/setup_venv.sh
```

Backend auto-detects the venv at `~/.local/share/vidclaw/venv`.

---

## Environment Variables

Copy `backend/.env.example` to `backend/.env`.

| Variable | Default | Description |
|---|---|---|
| `SERVER_HOST` | `0.0.0.0` | Bind address |
| `SERVER_PORT` | `8080` | Bind port |
| `RUST_LOG` | `info` | Log level (`debug` for verbose yt-dlp output) |
| `YTDLP_PATH` | *(auto)* | Override yt-dlp binary path |
| `YTDLP_VENV` | `~/.local/share/vidclaw/venv` | Override venv path |
| `YTDLP_PROXY` | *(none)* | HTTP proxy for yt-dlp (residential proxy for Instagram/Facebook on VPS) |
| `COOKIES_B64` | *(none)* | Base64-encoded Netscape cookies.txt — decoded at startup to `~/.config/vidclaw/cookies.txt` |
| `INSTAGRAM_SESSION_ID` | *(none)* | Instagram `sessionid` cookie value |
| `FACEBOOK_SESSION_COOKIES` | *(none)* | Facebook cookies as `key=value;key=value` pairs |
| `MAX_REQUESTS_PER_MINUTE` | `60` | Global rate limit |
| `MAX_REQUESTS_PER_IP_PER_MINUTE` | `30` | Per-IP rate limit |
| `CACHE_TTL_SECONDS` | `900` | Metadata cache TTL |

---

## ⚠️ CRITICAL: YouTube Bot Detection Fix — Do Not Break This

YouTube aggressively blocks datacenter IPs (including Render). The fix took days to get right. Do not change or remove any of the following without fully understanding what you are doing.

### How it works

**Component 1 — bgutil-ytdlp-pot-provider (Docker only)**

The Docker image installs `bgutil-ytdlp-pot-provider`, a yt-dlp plugin that solves YouTube's `po_token` challenge required for downloads from datacenter IPs. It runs against a pre-built Node.js bundle at `/opt/bgutil-pot/server/`.

Every yt-dlp call in `backend/src/api/ytdlp.rs` passes this argument — **it must stay in every invocation:**
```
--extractor-args "youtubepot-bgutilscript:server_home=/opt/bgutil-pot/server"
```

**Component 2 — COOKIES_B64 (YouTube authenticated cookies)**

Render's IP is known to YouTube as a datacenter. Even with the po_token plugin, YouTube also requires authenticated cookies from a real logged-in Google account.

Setup:
1. Log into YouTube in Chrome/Firefox (use a dedicated account, not your personal one)
2. Install the "Get cookies.txt LOCALLY" extension
3. Export cookies from `youtube.com` as Netscape format
4. Encode: `base64 -w0 cookies.txt` (Linux) or `base64 -i cookies.txt` (Mac)
5. Set `COOKIES_B64=<that output>` in Render environment + in `backend/.env`

**How COOKIES_B64 is decoded:**
- **Locally:** `main.rs` reads `COOKIES_B64` on startup (after loading `.env`) and writes to `~/.config/vidclaw/cookies.txt`
- **Render/Docker:** `entrypoint.sh` decodes `COOKIES_B64` to `/root/.config/vidclaw/cookies.txt` before starting the backend

Both paths write to the same location yt-dlp reads from.

### Rules — what NOT to do

| Never do this | Why it breaks everything |
|---|---|
| Add `--force-ipv6` to yt-dlp args | Render has no IPv6 outbound — always causes `[Errno 101] Network is unreachable` |
| Remove `--extractor-args youtubepot-bgutilscript:...` | YouTube blocks with bot detection / sign-in errors |
| Remove `--extractor-args youtube:player_client=...` | YouTube needs specific client targets for datacenter IPs |
| Use `--cookies-from-browser` on Render | Server has no browser profile; fails silently |
| Upgrade yt-dlp or bgutil without testing | Breaking changes in yt-dlp's YouTube extractor have broken things before |

---

## Instagram & Image Post Downloads

### Setting up Instagram authentication

Instagram blocks all anonymous access from server IPs.

**Step 1 — Session ID (required):**
1. Log into Instagram in Chrome
2. DevTools → Application → Cookies → `https://www.instagram.com`
3. Find the cookie named `sessionid` → copy the value
4. Set `INSTAGRAM_SESSION_ID=<value>` in `backend/.env` and in Render environment

**Step 2 — Full cookies (optional, improves reliability):**
Export cookies from a logged-in Instagram session using "Get cookies.txt LOCALLY", append to your `cookies.txt`, re-encode, and update `COOKIES_B64`.

### How image/photo post downloads work

When yt-dlp reports "There is no video in this post", the info endpoint returns `{ is_image: true }` instead of an error. The frontend hides the quality selector. The download then goes through this fallback chain:

1. All normal video format attempts (fail — expected for image posts)
2. `-f best` with `image_mode=true` (no `--merge-output-format mp4`), no cookies
3. Same with Instagram session cookies
4. Same with full cookies file
5. Browser cookie fallback chain
6. **Direct HTTP fallback:** `yt-dlp --print "%(url)s" --allow-unplayable-formats --no-check-formats -f best` to resolve the raw image URL → reqwest downloads it directly

---

## Playlist Support

Paste a YouTube playlist URL (containing `list=`) or a TikTok `@username` page URL. The frontend detects it via `isPlaylistUrl()`, calls `/api/playlist-info`, shows a banner with video count and playlist title, and lets you queue all entries at once with "Download all".

Backend limit: `--playlist-end 50`.

---

## Download Fallback Chain

For every download job, `backend/src/api/ytdlp.rs` (`extract_with_progress`) tries in order:

1. IPv6, no cookies (skipped on Render — no IPv6 available)
2. IPv4, no cookies
3. Session cookies (`INSTAGRAM_SESSION_ID` / `FACEBOOK_SESSION_COOKIES`)
4. Cookies file from `COOKIES_B64`
5. Browser cookies (chrome → chromium → firefox → brave → edge)
6. Image mode fallback: repeat 1–5 with `-f best` and no merge flag
7. Direct HTTP: resolve raw URL via `--print --allow-unplayable-formats`, download with reqwest

Hard errors (video deleted, private, geo-blocked, copyright) abort the chain immediately.

---

## API Reference

| Method | Path | Description |
|---|---|---|
| `POST` | `/api/info` | Fetch video metadata — returns `VideoInfo` including `is_image` |
| `POST` | `/api/download` | Start download job → `{ job_id }` |
| `GET` | `/api/progress/:job_id` | SSE stream until done/error/cancelled |
| `GET` | `/api/file/:job_id` | Stream completed file to browser |
| `POST` | `/api/cancel/:job_id` | Cancel in-progress download |
| `GET` | `/api/playlist-info?url=` | Fetch playlist title + entries (max 50) |
| `POST` | `/api/cookies` | Upload cookies.txt content |
| `DELETE` | `/api/cookies` | Remove uploaded cookies |
| `GET` | `/api/cookies/status` | `{ active: bool }` |
| `GET` | `/api/health` | Health check |

### SSE Events

```json
{ "type": "progress", "percent": 42.0, "speed": "3.2MB/s", "eta": "0:12" }
{ "type": "authenticating", "method": "cookies file" }
{ "type": "merging" }
{ "type": "done", "filename": "video.mp4" }
{ "type": "cancelled" }
{ "type": "error", "message": "This video is private." }
```

---

## Key Source Files

```
IVD/
├── Dockerfile              Multi-stage: Rust + Node + yt-dlp + ffmpeg + bgutil plugin
├── entrypoint.sh           Decodes COOKIES_B64 → cookies.txt, starts nginx + backend
├── render.yaml             Render deployment config (docker, healthcheck path)
│
├── backend/src/
│   ├── main.rs             Startup: dotenv load, COOKIES_B64 decode, Actix-web server init
│   ├── api/ytdlp.rs        ★ ALL download logic — yt-dlp subprocess, full fallback chain
│   ├── handlers/
│   │   ├── download.rs     POST /api/download
│   │   ├── info.rs         POST /api/info
│   │   ├── playlist.rs     GET /api/playlist-info
│   │   └── file_delivery.rs GET /api/file/:job_id
│   ├── jobs.rs             JobStore, JobEvent enum, JobResult struct
│   ├── models.rs           VideoInfo, PlaylistInfo, PlaylistEntry
│   └── error.rs            AppError → HTTP status codes
│
└── frontend/src/
    ├── App.tsx             Root component — URL input, playlist banner, queue, history
    ├── hooks/
    │   ├── useDownloadQueue.ts  SSE state machine, file save trigger
    │   └── useVideoInfo.ts      Debounced /api/info on URL change
    ├── services/api.ts     All backend HTTP calls
    ├── utils/urlDetection.ts    detectPlatform, isPlaylistUrl
    ├── utils/history.ts    localStorage read/write
    └── styles/components.css   All component CSS
```

---

## Deployment

```bash
# Push to GitHub — Render auto-deploys
git push origin main
```

First deploy: ~8 min (Rust compile). Subsequent deploys are faster (Docker layer cache).

### Required Render Environment Variables

| Variable | How to get it |
|---|---|
| `COOKIES_B64` | `base64 -w0 cookies.txt` — Netscape cookies from a logged-in YouTube/Google account |
| `INSTAGRAM_SESSION_ID` | DevTools → Application → Cookies → instagram.com → `sessionid` |
| `RUST_LOG` | `info` (use `debug` to see yt-dlp stderr in Render logs) |

### Docker (local testing)

```bash
docker build -t vidclaw .
docker run -p 8080:8080 \
  -e COOKIES_B64="$(base64 -w0 cookies.txt)" \
  -e INSTAGRAM_SESSION_ID="your_sessionid" \
  vidclaw
```

### Render Notes

- Free tier spins down after 15 min idle → ~30s cold start
- 512 MB RAM — fine for most downloads
- Ephemeral disk — `/tmp` cleared on restart (OK, temp files deleted after serving)
- **No IPv6 outbound** — `--force-ipv6` always fails on Render

---

## Legal

Only download content you own or have explicit permission to use. Respect platform terms of service and copyright law.
