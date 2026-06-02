# VIDCLAW — Things To Be Done

## Instagram & Facebook Downloads

### Problem
Instagram and Facebook block requests from datacenter IPs (VPS/Render) even with valid cookies.
Public content requires auth cookies; yt-dlp without them returns "empty media response".

### Fix (two parts)

**Part 1 — Service account cookies**
1. Create a throwaway Instagram account (separate from personal account)
2. Create a throwaway Facebook account
3. Log into both in Chromium or Brave
4. Install "Get cookies.txt LOCALLY" browser extension
5. Visit `instagram.com` → export cookies → save as `instagram.txt`
6. Visit `facebook.com` → export cookies → save as `facebook.txt`
7. Combine: `cat instagram.txt facebook.txt > cookies.txt`
8. Local: move to `~/.config/vidclaw/cookies.txt`
9. Render: `base64 -w0 cookies.txt` → paste output as `COOKIES_B64` env var in dashboard

**Part 2 — Residential proxy (required for Render/VPS)**
- Without a residential proxy, Instagram blocks the server IP regardless of cookies
- Sign up for a residential proxy (cheapest options: Webshare ~$3/mo, Proxyscrape ~$5/mo)
- Add proxy support to `ytdlp.rs` — set `YTDLP_PROXY` env var:
  ```
  YTDLP_PROXY=http://user:pass@proxy.webshare.io:80
  ```
- Code change needed: pass `--proxy $YTDLP_PROXY` to yt-dlp when env var is set
- Only apply proxy to Instagram and Facebook URLs (YouTube/Twitter/TikTok don't need it)

### Status
- [ ] Create service account cookies
- [ ] Add proxy support to `backend/src/api/ytdlp.rs`
- [ ] Set `COOKIES_B64` + `YTDLP_PROXY` on Render

---

## TikTok Fix

### Problem
yt-dlp needs `curl-cffi` installed in the system Python for TikTok's JS challenge.
Currently installed only in user Python (`~/.local`), not system Python (`/usr/lib/python3.x`).

### Fix
```bash
sudo pip install curl-cffi --break-system-packages
```

Verify: `yt-dlp --list-impersonate-targets` — should show Chrome/Firefox/Edge as **available**.

### Status
- [ ] Run `sudo pip install curl-cffi --break-system-packages` locally
- [ ] Verify: `yt-dlp --list-impersonate-targets` shows Chrome/Firefox/Edge as available
- [ ] Test TikTok download locally after fix
- [ ] TikTok is included in the Docker image (already in `Dockerfile` via `pip3 install curl-cffi`)

### Pre-deployment checklist for TikTok
- [ ] Add `YTDLP_PROXY` env var if TikTok also blocks the server IP (less common than Instagram)
- [ ] Test TikTok on Render after deploy — if blocked, apply same residential proxy fix

---

## Deployment (Render)

### Pre-deployment Checklist
- [ ] All tests passing (`bun run test` + `cargo test`)
- [ ] Frontend builds clean (`bun run build`)
- [ ] TikTok working locally (`sudo pip install curl-cffi --break-system-packages`)
- [ ] Service account cookies ready (`COOKIES_B64`) — Instagram + Facebook
- [ ] Residential proxy credentials ready (`YTDLP_PROXY`) — optional, Instagram/Facebook/TikTok
- [ ] Update `FRONTEND_URL` in `render.yaml` to actual Render URL

### Steps
1. Push repo to GitHub
2. Go to [render.com](https://render.com) → New Web Service → Connect GitHub repo
3. Render auto-detects `render.yaml` → review settings
4. Add environment variables in Render dashboard:
   - `COOKIES_B64` — base64 encoded cookies.txt (Instagram + Facebook)
   - `YTDLP_PROXY` — residential proxy URL (optional)
   - `RUST_LOG=info`
5. Deploy → wait for build (~5-10 min first time, Rust compile is slow)
6. Test each platform after deploy

### Known Render Free Tier Limitations
- Spins down after 15 min inactivity → ~30s cold start on first request
- 512MB RAM — sufficient for yt-dlp + ffmpeg on short videos
- Ephemeral disk — `/tmp` files deleted on restart (fine, we delete them after serving)
- No persistent storage — cookies set via env var only

### Post-deployment
- [ ] Run Lighthouse PWA audit on live URL
- [ ] Test YouTube download
- [ ] Test Twitter/X download
- [ ] Test TikTok download (needs `curl-cffi` in Docker — already included)
- [ ] Test Instagram download (needs `COOKIES_B64` + `YTDLP_PROXY`)
- [ ] Test Facebook download (needs `COOKIES_B64` + `YTDLP_PROXY`)
- [ ] Monitor Render logs for yt-dlp errors

---

## Nice-to-Have (Future)

- [ ] SSE progress endpoint — stream yt-dlp download progress to frontend instead of fake progress bar
- [ ] Cookie upload UI — let users upload their own cookies.txt via the app
- [ ] Proxy UI toggle — admin setting to enable/disable proxy per platform
- [ ] Rate limiting per IP — prevent abuse on public deployment
- [ ] Video format selector — let user choose quality (360p / 720p / 1080p)
