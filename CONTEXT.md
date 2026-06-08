# VIDCLAW — Project Context

**Last updated:** 2026-06-08
**Status:** Deployed and working on Render.

---

## What This Is

Full-stack video/image downloader. Paste a URL → preview → download MP4, MP3, or image.

- **Backend:** Rust + Actix-web 4.x, yt-dlp subprocess for all downloads
- **Frontend:** React + TypeScript + Vite, PWA-enabled
- **Deployed:** Render (free tier), single Docker container (nginx + Rust binary)

---

## Current Platform Status

| Platform | Videos | Images/Photos | Notes |
|---|---|---|---|
| YouTube | ✅ | N/A | Requires `COOKIES_B64` + bgutil plugin on Render |
| TikTok | ✅ | N/A | Docker has curl-cffi fix; run `bash scripts/setup_venv.sh` locally |
| Twitter/X | ✅ | ✅ | HLS → MP4 via ffmpeg |
| Instagram | ✅ | ✅ | Requires `INSTAGRAM_SESSION_ID` env var |
| Facebook | ⚠️ | ⚠️ | Requires `FACEBOOK_SESSION_COOKIES`; residential proxy improves reliability |

---

## How to Run Locally

```bash
# Both servers from repo root
bun run dev
# Backend: http://localhost:8080
# Frontend: http://localhost:5173
```

Make sure `backend/.env` exists (copy from `backend/.env.example`) with at minimum:
- `COOKIES_B64` — for YouTube on Render IP ranges (see README for how to get this)
- `INSTAGRAM_SESSION_ID` — for Instagram downloads

---

## Architecture: How a Download Works

```
User pastes URL
  → debounced POST /api/info → VideoInfo { title, thumbnail, is_image, ... }
  → User clicks Download
  → POST /api/download { url, quality, audio_only }
  → GET /api/progress/:job_id  (SSE stream)

Inside extract_with_progress() in api/ytdlp.rs:
  1. IPv6, no cookies           (skipped on Render — no IPv6)
  2. IPv4, no cookies
  3. Session cookies             (INSTAGRAM_SESSION_ID / FACEBOOK_SESSION_COOKIES)
  4. Cookies file                (~/.config/vidclaw/cookies.txt from COOKIES_B64)
  5. Browser cookie chain        (chrome → chromium → firefox → brave → edge)
  6. Image mode fallback         (repeat 1–5 with -f best, no --merge-output-format)
  7. Direct HTTP fallback        (yt-dlp --print %(url)s → reqwest download)

  → hard errors abort chain immediately (private, deleted, geo-blocked, copyright)
  → on success: write to /tmp/vidclaw_{uuid}/media.{ext}
  → GET /api/file/:job_id streams file to browser → temp dir deleted
```

---

## The YouTube Fix (Critical — Do Not Break)

YouTube blocks datacenter IPs. Two-part solution in place:

**1. bgutil-ytdlp-pot-provider** — Docker installs this yt-dlp plugin which solves YouTube's `po_token` challenge. The plugin runs against a Node.js bundle at `/opt/bgutil-pot/server/`.

Every yt-dlp call in `api/ytdlp.rs` must include:
```
--extractor-args "youtubepot-bgutilscript:server_home=/opt/bgutil-pot/server"
--extractor-args "youtube:player_client=mweb,web"   (no cookies)
--extractor-args "youtube:player_client=web"         (with cookies)
```

**2. COOKIES_B64** — base64-encoded Netscape cookies.txt from a logged-in Google/YouTube account.
- Decoded at startup by `main.rs` (local) or `entrypoint.sh` (Docker/Render)
- Written to `~/.config/vidclaw/cookies.txt` or `/root/.config/vidclaw/cookies.txt`
- yt-dlp uses it via `--cookies <path>`

**Never add `--force-ipv6`** — Render has no IPv6 outbound, causes `Network is unreachable`.

---

## Image/Photo Post Downloads

For posts with no video (Instagram photos, Twitter images):
- `/api/info` returns `{ is_image: true }` when yt-dlp says "There is no video in this post"
- Frontend hides quality selector, proceeds to download
- Download tries image mode (`-f best`, no `--merge-output-format mp4`)
- Final fallback: `yt-dlp --print "%(url)s" --allow-unplayable-formats` → reqwest HTTP download

---

## Playlist Support

- YouTube: URLs containing `list=` parameter
- TikTok: `@username` pages without `/video/`
- Frontend detects via `isPlaylistUrl()` in `utils/urlDetection.ts`
- Calls `GET /api/playlist-info?url=` → backend runs `--flat-playlist --dump-json --playlist-end 50`
- "Download all" queues each entry individually

---

## Key Files

```
IVD/
├── README.md               Full developer reference — READ THIS FIRST
├── CONTEXT.md              This file — current state overview
├── Dockerfile              Multi-stage: Rust + Node + yt-dlp + ffmpeg + bgutil plugin
├── entrypoint.sh           Startup: COOKIES_B64 decode → cookies.txt, nginx + backend
├── render.yaml             Render deployment config
│
├── backend/src/
│   ├── main.rs             Startup: dotenv, COOKIES_B64 decode, Actix-web init
│   ├── api/ytdlp.rs        ★ ALL download logic — read before touching
│   ├── handlers/
│   │   ├── download.rs     POST /api/download
│   │   ├── info.rs         POST /api/info
│   │   ├── playlist.rs     GET /api/playlist-info
│   │   └── file_delivery.rs GET /api/file/:job_id
│   ├── jobs.rs             JobStore, JobEvent, JobResult
│   ├── models.rs           VideoInfo, PlaylistInfo, PlaylistEntry
│   └── error.rs            AppError → HTTP status codes
│
└── frontend/src/
    ├── App.tsx             Root — URL input, playlist banner, queue, history UI
    ├── hooks/useDownloadQueue.ts  SSE state machine, file save trigger
    ├── hooks/useVideoInfo.ts      Debounced /api/info (skips playlist URLs)
    ├── services/api.ts     All backend calls (axios)
    ├── utils/urlDetection.ts  detectPlatform, isPlaylistUrl
    ├── utils/history.ts    localStorage read/write
    └── styles/components.css  All component CSS
```

---

## Environment Variables (Quick Reference)

| Var | Required on Render | Purpose |
|---|---|---|
| `COOKIES_B64` | ✅ Yes | YouTube cookies — must be from a logged-in Google account |
| `INSTAGRAM_SESSION_ID` | ✅ Yes | Instagram sessionid cookie |
| `RUST_LOG` | Recommended | Set to `info`; use `debug` to see yt-dlp stderr |
| `FACEBOOK_SESSION_COOKIES` | Optional | `key=value;key=value` format |
| `YTDLP_PROXY` | Optional | Residential proxy improves Instagram/Facebook reliability |

---

## Known Gotchas

- **Render has no IPv6** — `--force-ipv6` always fails. The code already skips it on Render.
- **YouTube cookies expire** — if YouTube downloads start failing on Render, re-export cookies, re-encode, update `COOKIES_B64`.
- **Instagram sessionid expires** — usually lasts weeks. If Instagram fails, refresh `INSTAGRAM_SESSION_ID`.
- **TikTok locally** — needs `curl-cffi` in venv: `bash scripts/setup_venv.sh`. Works in Docker without extra steps.
- **COOKIES_B64 decoding** — `main.rs` decodes it at startup (local). If you change the env var, restart the backend.
- **Playlist cap** — backend fetches max 50 entries. Larger playlists are truncated.
- **Content-type** — file extension is determined at runtime from actual yt-dlp output, not hardcoded to `video/mp4`. Don't revert `file_delivery.rs` to hardcode it.
