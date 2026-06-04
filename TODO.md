# VIDCLAW — Things To Be Done

---

## ✅ Completed

- [x] SSE progress — real download speed, ETA, % from yt-dlp stdout
- [x] Cookie upload UI — browser file upload → `POST /api/cookies`
- [x] Proxy support — `YTDLP_PROXY` env var passed to yt-dlp
- [x] Rate limiting per IP — global + per-IP middleware
- [x] Video format selector — Best / 1080p / 720p / 480p / 360p
- [x] Cancel download — kills yt-dlp child process, cleans up temp file
- [x] PWA — installable, service worker, manifest
- [x] Smart error messages — private / geo / copyright / auth / no-video
- [x] Docker + Render deployment config
- [x] Audio-only download (MP3 extraction)
- [x] Thumbnail preview before download — title, uploader, duration, filesize, platform
- [x] Multiple simultaneous downloads — independent job queue, each with own progress
- [x] Download history — localStorage, persists across sessions

---

## Instagram & Facebook Downloads

### Problem
Instagram and Facebook block requests from datacenter IPs (VPS/Render) even with valid cookies.
Requires two things: (1) a logged-in service account cookie, (2) residential proxy IP.

### Fix

**Step 1 — Service account cookies**
1. Create throwaway Instagram account (separate from personal)
2. Create throwaway Facebook account
3. Log into both in Chromium or Brave
4. Install **"Get cookies.txt LOCALLY"** extension
5. Visit `instagram.com` → export → save as `instagram.txt`
6. Visit `facebook.com` → export → save as `facebook.txt`
7. Combine: `cat instagram.txt facebook.txt > cookies.txt`
8. **Option A — Local:** `mkdir -p ~/.config/vidclaw && mv cookies.txt ~/.config/vidclaw/cookies.txt`
9. **Option B — Render/Docker:** encode → `base64 -w0 cookies.txt` → paste as `COOKIES_B64` env var
10. **Option C — UI upload:** use the Settings → Cookie Upload section in the app

**Step 2 — Residential proxy (required for server deployments)**

Without residential proxy, Instagram/Facebook detect the datacenter IP and block regardless of cookies.

1. Sign up for residential proxy:
   - [Webshare.io](https://webshare.io) — ~$3/mo (cheapest)
   - [Oxylabs](https://oxylabs.io) — more reliable, more expensive
2. Get proxy URL: `http://user:pass@proxy.webshare.io:80`
3. Set env var: `YTDLP_PROXY=http://user:pass@proxy.webshare.io:80`
   - Local: add to `backend/.env`
   - Render: set in dashboard → Environment

### Status
- [ ] Create service account cookies (throwaway Instagram + Facebook)
- [ ] Upload/configure cookies (local file or Render env var or UI upload)
- [ ] Sign up for residential proxy
- [ ] Set `YTDLP_PROXY` env var (local `.env` or Render dashboard)
- [ ] Test Instagram download
- [ ] Test Facebook download

---

## TikTok Fix (Local Only)

### Problem
`curl-cffi` installed to user Python (`~/.local`) but yt-dlp uses system Python.
TikTok's JS challenge fails without impersonation support.

> **Note:** Docker image already installs `curl-cffi` at system level — TikTok works on Render.
> Run `bash scripts/setup_venv.sh` to set up a local venv with curl-cffi.

### Fix (if not using venv)

```bash
sudo pip install curl-cffi --break-system-packages

# Verify
yt-dlp --list-impersonate-targets
# Should show Chrome / Firefox / Edge / Safari as "available" (not greyed out)
```

### Status
- [ ] Run `bash scripts/setup_venv.sh` (preferred) or sudo pip install
- [ ] Verify impersonate targets available
- [ ] Test TikTok download locally

---

## Deployment (Render)

### Pre-deployment Checklist
- [x] Frontend builds clean: `bun run build`
- [x] Backend compiles: `cargo check`
- [ ] All tests passing: `bun run test` + `cd backend && cargo test`
- [ ] TikTok working locally
- [ ] Service account cookies ready (`COOKIES_B64` or UI upload)
- [ ] Residential proxy URL ready (`YTDLP_PROXY`)

### Deploy Steps

```bash
# 1. Push to GitHub
git push origin main

# 2. render.com → New Web Service → Connect repo
# 3. Render detects render.yaml automatically
# 4. Set in Render dashboard → Environment:
#    COOKIES_B64=<output of: base64 -w0 cookies.txt>
#    YTDLP_PROXY=http://user:pass@proxy.webshare.io:80
#    RUST_LOG=info
# 5. Deploy — first build takes ~8 min (Rust compile)
```

### Post-deployment Testing
- [ ] YouTube download ✓
- [ ] Twitter/X download ✓
- [ ] TikTok download (curl-cffi in Docker)
- [ ] Video preview shows before download
- [ ] Audio-only (MP3) download works
- [ ] Queue multiple downloads simultaneously
- [ ] Download history persists across page refresh
- [ ] Instagram download (needs cookies + proxy)
- [ ] Facebook download (needs cookies + proxy)
- [ ] Cancel download works
- [ ] Cookie upload via UI works

---

## Future Ideas

- [ ] SSE for cookie upload progress (large cookie files)
- [ ] Playlist support (YouTube playlists, TikTok user pages)
- [ ] Re-download from history (one-click retry from history item)
- [ ] Share / copy download link
