# VIDCLAW — Architecture

## System Overview

```
Browser (React + TypeScript + Vite)
  │
  │  POST /api/info         → VideoInfo { title, thumbnail, is_image, ... }
  │  POST /api/download     → { job_id }
  │  GET  /api/progress/:id → SSE events (progress / merging / done / error)
  │  GET  /api/file/:id     → file stream (video/mp4, image/jpeg, audio/mpeg, ...)
  │  GET  /api/playlist-info?url= → PlaylistInfo { title, entries[] }
  ▼
nginx (port 80 / $PORT on Render)
  │ proxy_pass to backend:8081
  ▼
Rust + Actix-web (port 8081 in Docker, 8080 locally)
  │
  ├── Rate limiter middleware (global + per-IP)
  ├── CORS middleware
  │
  ├── handlers/info.rs       → ytdlp::get_info()
  ├── handlers/download.rs   → jobs::JobStore::spawn() → ytdlp::extract_with_progress()
  ├── handlers/progress.rs   → SSE stream from broadcast::Receiver<JobEvent>
  ├── handlers/file_delivery.rs → stream file from disk
  ├── handlers/playlist.rs   → ytdlp::get_playlist_info()
  └── handlers/cookies.rs    → read/write ~/.config/vidclaw/cookies.txt
       │
       ▼
  api/ytdlp.rs  (yt-dlp subprocess — the core engine)
       │
       ├── get_info()              --dump-json → VideoInfo
       ├── get_playlist_info()     --flat-playlist --dump-json → PlaylistInfo
       └── extract_with_progress() multi-step fallback chain → file on disk
```

---

## Backend: api/ytdlp.rs

This is the most critical file. All download logic lives here.

### `get_info(url)`

Tries to extract video metadata using `yt-dlp --dump-json`. Falls back through:
1. IPv6 + no cookies
2. IPv4 + no cookies
3. Session cookies (env vars)
4. Cookies file
5. `try_get_info_nofmt` — uses `--print` fields instead of `--dump-json` for platforms that fail format selection

Special case: if yt-dlp says "There is no video in this post" or "No video formats found", returns `VideoInfo { is_image: true }` instead of an error.

### `extract_with_progress(url, quality, audio_only, tx, result_store, cancelled)`

Runs in a spawned task. Streams `JobEvent` values over a `broadcast::Sender`. Tries each step until one succeeds:

```
Step 1: yt-dlp with video format, IPv6, no cookies
Step 2: yt-dlp with video format, IPv4, no cookies
Step 3: yt-dlp with video format, session cookies (INSTAGRAM_SESSION_ID / FACEBOOK_SESSION_COOKIES)
Step 4: yt-dlp with video format, cookies file (~/.config/vidclaw/cookies.txt)
Step 5: yt-dlp with video format, browser cookies (chrome/chromium/firefox/brave/edge)
Step 6: image mode (-f best, no --merge-output-format), steps 1–5 repeated
Step 7: --print "%(url)s" --allow-unplayable-formats → reqwest HTTP download
```

Hard error strings (video deleted, private, geo-blocked, copyright) abort the chain immediately without trying remaining steps.

### yt-dlp arguments — what they do and why

| Argument | Why it's there |
|---|---|
| `--extractor-args "youtubepot-bgutilscript:server_home=/opt/bgutil-pot/server"` | Solves YouTube's po_token challenge for datacenter IPs. Remove this and YouTube breaks on Render. |
| `--extractor-args "youtube:player_client=mweb,web"` | YouTube client targets for unauthenticated requests on datacenter IPs |
| `--extractor-args "youtube:player_client=web"` | YouTube client target for authenticated (cookies) requests |
| `--merge-output-format mp4` | Mux bestvideo + bestaudio into MP4 via ffmpeg. Skipped for image mode. |
| `--no-playlist` | Prevents yt-dlp from treating a playlist URL as a playlist in single-download mode |
| `--newline` | Makes yt-dlp flush progress lines immediately (needed for SSE streaming) |
| `--allow-unplayable-formats --no-check-formats` | Used in direct HTTP fallback to resolve image URLs yt-dlp refuses to "download" |

### Output file handling

Downloads write to `/tmp/vidclaw_{uuid}/media.%(ext)s`. The actual extension is determined by yt-dlp at runtime (jpg, mp4, webm, mp3, etc.). `find_output_file()` scans the dir for the first non-empty file. `content_type_from_path()` derives the MIME type from the extension. This is why `file_delivery.rs` must use `result.content_type` — do not hardcode `video/mp4`.

---

## Frontend Architecture

### Download Queue State Machine (`useDownloadQueue.ts`)

Each download item goes through: `downloading → done | error | cancelled`

1. `addDownload(url, quality, audioOnly, info?)` — creates queue item, POSTs to `/api/download`
2. Opens SSE connection to `/api/progress/:job_id`
3. Handles events: `progress` / `authenticating` / `merging` / `done` / `cancelled` / `error`
4. On `done`: GETs `/api/file/:job_id`, creates blob URL, triggers `<a download>` click
5. Saves to localStorage history (if `info` is present)

### URL → Info → Download Flow (`App.tsx`)

```
URL typed
  → isPlaylistUrl()? → show playlist banner, fetch /api/playlist-info
  → else → useVideoInfo() debounced 900ms → /api/info → VideoInfo
              → VideoPreview shows thumbnail/title/duration
              → QualitySelector hidden if is_image=true

User clicks Download
  → isPlaylist? → handleDownloadAll() → queue each playlist entry
  → else → handleDownload() → addDownload(url, quality, audioOnly, info)
```

### Key Frontend Files

| File | Purpose |
|---|---|
| `App.tsx` | Root — URL state, playlist state, queue/history rendering |
| `hooks/useDownloadQueue.ts` | SSE state machine, file save |
| `hooks/useVideoInfo.ts` | Debounced info fetch (skips playlist URLs) |
| `services/api.ts` | All axios calls to backend |
| `utils/urlDetection.ts` | `detectPlatform`, `isPlaylistUrl`, `isValidUrl` |
| `utils/history.ts` | localStorage read/write |
| `components/QualitySelector.tsx` | Returns null when `isImage=true` |
| `components/DownloadQueueItem.tsx` | Progress bar, copy URL, cancel, remove |
| `styles/components.css` | All component CSS (single file) |

---

## Docker / Deployment

```
Dockerfile (multi-stage):
  Stage 1: node — builds frontend (dist/)
  Stage 2: rust — compiles backend binary
  Stage 3: final — debian slim + yt-dlp + ffmpeg + bgutil plugin + nginx

entrypoint.sh:
  1. Decode COOKIES_B64 → /root/.config/vidclaw/cookies.txt
  2. Substitute $PORT into nginx config (Render injects PORT)
  3. Start nginx (background)
  4. exec video-downloader (foreground, BACKEND_PORT=8081)
```

### Why nginx in front of the Rust backend?

Render injects `$PORT` (e.g. 10000). Actix-web is bound to a fixed internal port (8081). nginx listens on `$PORT` and proxies to `localhost:8081`. This decouples the Render-assigned port from the backend config.

---

## Job System (`jobs.rs`)

```rust
JobStore: Arc<DashMap<String, JobHandle>>
  JobHandle {
    tx: broadcast::Sender<JobEvent>,
    result: Arc<Mutex<Option<JobResult>>>,
    cancelled: Arc<AtomicBool>,
  }

JobEvent: Progress { percent, speed, eta }
        | Authenticating { method }
        | Merging
        | Done { filename }
        | Cancelled
        | Error { message }

JobResult { file_path: String, filename: String, content_type: String }
```

`extract_with_progress` runs in `tokio::spawn`. The SSE handler holds a `broadcast::Receiver` and forwards events to the client. When the client disconnects, the SSE handler is dropped; cancellation is via `AtomicBool`.

---

## Rate Limiting (`middleware/rate_limiter.rs`)

Two limits enforced per request:
- Global: `MAX_REQUESTS_PER_MINUTE` (default 60)
- Per-IP: `MAX_REQUESTS_PER_IP_PER_MINUTE` (default 30)

Both are in-memory sliding window counters. No Redis — state is local to the process. On Render free tier (single instance) this is fine.

---

## Error Handling (`error.rs`)

```rust
AppError::BadRequest(msg)       → 400
AppError::PlatformError(msg)    → 502
AppError::InternalServerError   → 500
AppError::NotFound              → 404
AppError::TooManyRequests       → 429
```

Hard errors in yt-dlp stderr (private, geo-blocked, copyright, deleted) map to `PlatformError` with a human-readable message. Auth failures map to a different `PlatformError` message. The distinction matters for UX — users get actionable error messages.
