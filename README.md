# VIDCLAW

Download videos from Instagram, TikTok, YouTube, Twitter/X, and Facebook. No account required, no ads.

## Features

- **Video preview** — thumbnail, title, uploader, duration, and file size shown before you download
- **Multi-platform** — Instagram, TikTok, YouTube, Twitter/X, Facebook
- **Quality selector** — Best / 1080p / 720p / 480p / 360p
- **Audio-only (MP3)** — extract audio from any video
- **Simultaneous downloads** — queue multiple URLs; each runs in parallel
- **Real-time progress** — live speed, ETA, and % via SSE
- **Download history** — persisted in localStorage across sessions
- **Cancel at any time** — kills the yt-dlp process and cleans up
- **PWA** — installable on Android, iOS, and desktop
- **Cookie support** — upload cookies.txt for private/authenticated content

## Stack

| Layer | Tech |
|---|---|
| Backend | Rust + Actix-web |
| Frontend | React + TypeScript + Vite |
| Video engine | yt-dlp (+ ffmpeg for muxing) |
| Deployment | Docker + Render |

## Local Development

### Prerequisites

- Rust 1.70+
- Bun
- yt-dlp in PATH (or see venv setup below)
- ffmpeg in PATH

### Run

```bash
# From repo root — starts backend (port 8080) + frontend (port 5173)
bun install
bun run dev
```

Stop with Ctrl+C.

### yt-dlp venv (recommended for TikTok)

TikTok requires `curl-cffi` for JS challenge impersonation. The setup script creates an isolated venv so you don't need `sudo pip`:

```bash
bash scripts/setup_venv.sh
```

The backend auto-detects the venv at `~/.local/share/vidclaw/venv`.

## Environment Variables

Copy `backend/.env.example` to `backend/.env` and fill in as needed.

| Variable | Default | Description |
|---|---|---|
| `SERVER_HOST` | `0.0.0.0` | Bind address |
| `SERVER_PORT` | `8080` | Bind port |
| `FRONTEND_URL` | `http://localhost:5173` | Used in CORS |
| `RUST_LOG` | `info` | Log level |
| `YTDLP_PATH` | *(auto)* | Override yt-dlp binary path |
| `YTDLP_VENV` | `~/.local/share/vidclaw/venv` | Override venv path |
| `YTDLP_PROXY` | *(none)* | Residential proxy for Instagram/Facebook on VPS |
| `INSTAGRAM_SESSION_ID` | *(none)* | Instagram sessionid cookie value |
| `FACEBOOK_SESSION_COOKIES` | *(none)* | Facebook cookies as `key=value;key=value` |
| `COOKIES_B64` | *(none)* | Base64-encoded cookies.txt (Render/Docker) |
| `MAX_REQUESTS_PER_MINUTE` | `60` | Global rate limit |
| `MAX_REQUESTS_PER_IP_PER_MINUTE` | `30` | Per-IP rate limit |

## Instagram & Facebook

These platforms block datacenter IPs. Two things required:

1. **Cookies** — log into a throwaway account in Chromium/Brave, export via "Get cookies.txt LOCALLY" extension, upload in app Settings or set `COOKIES_B64` env var
2. **Residential proxy** — set `YTDLP_PROXY=http://user:pass@proxy.webshare.io:80` ([Webshare](https://webshare.io) ~$3/mo)

## Deployment (Render)

```bash
# 1. Push to GitHub
git push origin main

# 2. render.com → New Web Service → Connect repo
# Render auto-detects render.yaml

# 3. Set in Render dashboard → Environment:
#    COOKIES_B64=<base64 -w0 cookies.txt>
#    YTDLP_PROXY=http://user:pass@proxy.webshare.io:80
#    RUST_LOG=info
```

First deploy takes ~8 min (Rust compile). Subsequent deploys are faster due to Docker layer caching.

### Render Free Tier Notes

- Spins down after 15 min idle → ~30s cold start
- 512 MB RAM — sufficient for most videos
- Ephemeral disk — `/tmp` cleared on restart (fine, temp files deleted after serving)

## API

| Method | Path | Description |
|---|---|---|
| `POST` | `/api/info` | Fetch video metadata (title, thumbnail, duration, etc.) |
| `POST` | `/api/download` | Start download job → returns `{ job_id }` |
| `GET` | `/api/progress/:job_id` | SSE stream of progress events |
| `GET` | `/api/file/:job_id` | Download the completed file |
| `POST` | `/api/cancel/:job_id` | Cancel in-progress download |
| `POST` | `/api/cookies` | Upload cookies.txt content |
| `DELETE` | `/api/cookies` | Remove uploaded cookies |
| `GET` | `/api/cookies/status` | Check if cookies are active |
| `GET` | `/api/health` | Health check |

### SSE Event Types

```json
{ "type": "progress", "percent": 42.0, "speed": "3.2MB/s", "eta": "0:12" }
{ "type": "authenticating", "method": "cookies file" }
{ "type": "merging" }
{ "type": "done", "filename": "video.mp4" }
{ "type": "cancelled" }
{ "type": "error", "message": "This video is private." }
```

## Legal

Only download content you own or have explicit permission to use. Respect platform terms of service and copyright law.
