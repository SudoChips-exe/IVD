# VIDCLAW — Things To Be Done

---

## ✅ Completed

- [x] SSE progress — real download speed, ETA, % from yt-dlp stdout
- [x] Cookie upload UI — browser file upload → `POST /api/cookies`
- [x] Proxy support — `YTDLP_PROXY` env var passed to yt-dlp
- [x] Rate limiting per IP — global + per-IP middleware (already existed)
- [x] Video format selector — Best / 1080p / 720p / 480p / 360p
- [x] Cancel download — kills yt-dlp child process, cleans up temp file
- [x] PWA — installable, service worker, manifest
- [x] Smart error messages — private / geo / copyright / auth / no-video
- [x] Docker + Render deployment config

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

Without residential proxy, Instagram/Facebook detect the datacenter IP and block regardless of cookies. Services like saveclip.app use this exact approach.

1. Sign up for residential proxy:
   - [Webshare.io](https://webshare.io) — ~$3/mo (cheapest)
   - [Oxylabs](https://oxylabs.io) — more reliable, more expensive
2. Get proxy URL: `http://user:pass@proxy.webshare.io:80`
3. Set env var: `YTDLP_PROXY=http://user:pass@proxy.webshare.io:80`
   - Local: add to `backend/.env`
   - Render: set in dashboard → Environment
4. Proxy is already wired in code — `ytdlp.rs` reads `YTDLP_PROXY` and passes `--proxy` to yt-dlp

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
`curl-cffi` installed to user Python (`~/.local`) but yt-dlp uses system Python (`/usr/lib/python3.x`).
TikTok's JS challenge fails without impersonation support.

> **Note:** Docker image already installs `curl-cffi` at system level — TikTok works on Render.

### Fix (local dev only)

```bash
sudo pip install curl-cffi --break-system-packages

# Verify
yt-dlp --list-impersonate-targets
# Should show Chrome / Firefox / Edge / Safari as "available" (not greyed out)
```

### Status
- [ ] Run `sudo pip install curl-cffi --break-system-packages`
- [ ] Verify impersonate targets available
- [ ] Test TikTok download locally

---

## Deployment (Render)

### Pre-deployment Checklist
- [ ] All tests passing: `bun run test` + `cd backend && cargo test`
- [ ] Frontend builds clean: `bun run build`
- [ ] TikTok working locally (sudo pip install curl-cffi)
- [ ] Service account cookies ready (`COOKIES_B64` or UI upload)
- [ ] Residential proxy URL ready (`YTDLP_PROXY`)
- [ ] Update `FRONTEND_URL` in `render.yaml` to actual Render URL (e.g. `https://vidclaw.onrender.com`)

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
# 5. Deploy — first build takes ~5-10 min (Rust compile)
```

### Known Render Free Tier Limitations
- Spins down after 15 min inactivity → ~30s cold start on first request
- 512MB RAM — fine for most videos; very long videos may OOM
- Ephemeral disk — `/tmp` deleted on restart (fine, we delete temp files after serving)

### Post-deployment Testing
- [ ] Lighthouse PWA audit on live URL
- [ ] YouTube download ✓
- [ ] Twitter/X download ✓
- [ ] TikTok download (curl-cffi in Docker)
- [ ] Instagram download (needs cookies + proxy)
- [ ] Facebook download (needs cookies + proxy)
- [ ] Cancel download works
- [ ] Cookie upload via UI works
- [ ] Monitor Render logs for errors

---

## Future Ideas

- [ ] SSE for cookie upload progress (large cookie files)
- [ ] Multiple simultaneous downloads (job queue)
- [ ] Download history (localStorage)
- [ ] Audio-only download option (MP3 extraction)
- [ ] Thumbnail preview before download
