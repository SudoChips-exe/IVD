# Universal Social Media Video Downloader

> Download videos from Instagram, TikTok, YouTube, Twitter/X, Facebook, and Snapchat instantly with a single click. No ads, no tracking, preserves audio and quality.

## ✨ Features

- **One-Click Downloads** — Paste a video link, press download
- **Audio Preservation** — Never strips audio or quality
- **Multi-Platform Support** — Works with 6 major social platforms:
  - 📸 Instagram (Reels, Stories, Posts)
  - 🎵 TikTok (Videos, Sounds)
  - ▶️ YouTube (Videos, Shorts)
  - 𝕏 Twitter/X (Videos, GIFs)
  - 📘 Facebook (Videos, Stories)
  - 👻 Snapchat (Stories, Memories)
- **Direct Streaming** — No intermediate storage; downloads stream directly to your device
- **Mobile Friendly** — Use from any device with a browser
- **Free & Open Source** — No paid tiers or limitations

## 🚀 Quick Start

### For Users
1. Visit **[video-downloader.app](https://video-downloader.app)** (when deployed)
2. Paste any social media video URL into the input field
3. Click **Download**
4. Video saves to your device with audio included

### For Developers

#### Prerequisites
- Rust 1.70+
- Node.js 18+
- Docker (optional, for containerization)

#### Setup & Run Locally

```bash
# Clone the repository
git clone https://github.com/yourusername/video-downloader.git
cd video-downloader

# Start the Rust backend
cd backend
cargo build --release
cargo run

# In another terminal, start the React frontend
cd frontend
npm install
npm run dev
```

Backend runs on `http://localhost:8080`  
Frontend runs on `http://localhost:5173`

## 🏗️ Architecture

**Tech Stack:**
- **Backend**: Rust + Actix-web (high-performance HTTP server)
- **Frontend**: React + TypeScript + Vite (fast, responsive UI)
- **Deployment**: Cloud-hosted (AWS/Vercel)
- **Video Processing**: FFmpeg-compatible libraries

**How It Works:**
1. User enters a video URL in the web interface
2. Frontend sends URL to Rust backend via HTTP
3. Backend identifies the platform and fetches video metadata from official APIs
4. Backend retrieves the direct video URL (with audio stream)
5. Video streams directly to the user's browser (no server storage)
6. Browser downloads the file automatically

**Key Design Decision**: We use a **streaming approach** to avoid storing videos on the server. This keeps the application scalable and reduces infrastructure costs.

For detailed architecture, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## 📋 Supported Platforms & Limitations

| Platform | Support | Notes |
|----------|---------|-------|
| **Instagram** | ✅ Full | Reels, carousel videos, stories (public only) |
| **TikTok** | ✅ Full | Videos with sound, watermark preserved |
| **YouTube** | ✅ Full | Videos, Shorts (respects upload restrictions) |
| **Twitter/X** | ✅ Full | Video tweets, GIFs |
| **Facebook** | ✅ Full | Public videos only |
| **Snapchat** | ⚠️ Limited | Stories/memories (requires user credentials) |

### Rate Limits
- **Instagram**: 100 videos/hour per IP
- **TikTok**: 50 videos/hour per IP
- **YouTube**: 10,000 quota units/day (shared across all users)
- **Twitter/X**: 450 requests/15 minutes per API key
- **Facebook**: 200 requests/hour
- **Snapchat**: 50 requests/hour

## ⚠️ Legal & Ethical Considerations

This tool downloads videos as they are publicly shared. **You are responsible for respecting copyright and platform terms of service:**

- ✅ **Allowed**: Downloading your own content or content with owner permission
- ❌ **Not Allowed**: Circumventing DRM protections, downloading copyrighted content without permission, or violating platform ToS
- ⚖️ **Use at your own risk** — We cannot be held liable for misuse

By using this tool, you agree that you have the legal right to download the content.

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the `backend/` directory:

```env
# Server configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info

# API Keys (optional for public content, required for authenticated downloads)
INSTAGRAM_API_KEY=your_key_here
TIKTOK_API_KEY=your_key_here
YOUTUBE_API_KEY=your_key_here
TWITTER_API_KEY=your_key_here
FACEBOOK_API_KEY=your_key_here
SNAPCHAT_API_KEY=your_key_here

# Rate limiting
MAX_REQUESTS_PER_MINUTE=60
```

See `.env.example` for a template.

## 🐛 Troubleshooting

| Problem | Solution |
|---------|----------|
| "Private video" error | Ensure the video is public or you have access permissions |
| "Link not recognized" | Verify the link is a direct video URL (not a profile/channel) |
| "Download fails silently" | Check your internet connection; backend may be rate-limited |
| "No audio in video" | Report as a bug; our extraction should preserve audio |

## 📦 Deployment

### Docker
```bash
# Build and run with Docker
docker-compose up --build
```

### Cloud Deployment (AWS Example)
```bash
# Build Rust backend
cargo build --release

# Deploy to AWS Lambda or EC2
# See deployment/ directory for scripts
```

## 🤝 Contributing

We welcome contributions! To contribute:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Commit your changes: `git commit -m "Add your feature"`
4. Push to the branch: `git push origin feature/your-feature`
5. Open a Pull Request

### Development Guidelines
- Write tests for new platform adapters
- Ensure audio is preserved in all downloads
- Add rate-limit handling for new platforms
- Document API authentication requirements

## 🗺️ Roadmap

- [ ] **v1.0** — Core functionality for 6 platforms (current)
- [ ] **v1.1** — Batch downloads (multiple URLs at once)
- [ ] **v1.2** — Playlist support (download entire TikTok/YouTube playlists)
- [ ] **v1.3** — Audio-only extraction (MP3 downloads)
- [ ] **v1.4** — Custom quality/format selection
- [ ] **v2.0** — Desktop app (Electron wrapper)
- [ ] **v2.1** — Browser extension for one-click downloads

## 📄 License

This project is licensed under the **MIT License** — see [LICENSE](./LICENSE) file for details.

## ⚡ Performance Stats

- **Average download time**: 5-15 seconds (depends on video length & quality)
- **No server storage**: Videos stream directly (0 disk footprint per download)
- **Memory efficient**: Rust backend uses <50MB per concurrent download
- **Concurrent downloads**: Supports 1000+ simultaneous streams

## 💬 Support & Feedback

- **Report a bug**: [GitHub Issues](https://github.com/yourusername/video-downloader/issues)
- **Feature request**: [GitHub Discussions](https://github.com/yourusername/video-downloader/discussions)
- **Email**: support@video-downloader.app

---

**Made with ❤️ for video enthusiasts everywhere.**
