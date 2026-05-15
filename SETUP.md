# Universal Social Media Video Downloader

This project provides a comprehensive backend and frontend for downloading videos from multiple social media platforms with audio preservation.

## Project Status: Initial Setup Complete ✅

### ✅ Completed
- [x] Comprehensive README.md with features and setup instructions
- [x] Detailed ARCHITECTURE.md with system design and platform strategies
- [x] Rust backend project structure with:
  - Actix-web HTTP server setup
  - Configuration management system
  - Modular platform adapter architecture
  - Error handling framework
  - Middleware (rate limiting placeholder)
  - URL validation & platform detection utilities
- [x] React TypeScript frontend with:
  - Modern UI with component-based architecture
  - URL input with platform detection
  - Download progress tracking
  - Error handling
  - Responsive mobile-friendly design
- [x] Docker configuration for both backend and frontend
- [x] Environment configuration templates

### 📋 Next Steps

#### Phase 2: Platform Integration (Priority Order)
1. **Instagram Adapter** — Use instagrapi or Instagram Graph API
2. **TikTok Adapter** — Implement video extraction
3. **YouTube Adapter** — Use yt-dlp or YouTube API
4. **Twitter/X Adapter** — Twitter API v2 integration
5. **Facebook Adapter** — Graph API integration
6. **Snapchat Adapter** — Research and implement

#### Phase 3: Backend Features
- [ ] Implement actual video streaming to frontend
- [ ] Add rate limiting middleware (currently placeholder)
- [ ] Implement response headers (Content-Disposition, MIME types)
- [ ] Add metadata caching
- [ ] Error handling & validation

#### Phase 4: Frontend Integration
- [ ] Test against real backend endpoints
- [ ] Implement actual download streaming
- [ ] Add loading animations
- [ ] Mobile responsiveness testing

#### Phase 5: Deployment
- [ ] Docker build & test
- [ ] Cloud deployment setup
- [ ] Environment configuration
- [ ] Performance optimization

## Quick Start

### Local Development

**Backend (Terminal 1):**
```bash
cd backend
cargo run
# Runs on http://localhost:8080
```

**Frontend (Terminal 2):**
```bash
cd frontend
npm install
npm run dev
# Runs on http://localhost:5173
```

### Docker Compose
```bash
docker-compose up --build
```

## Project Structure

```
IVD/
├── README.md                 # User documentation
├── ARCHITECTURE.md           # System design document
├── .gitignore
├── docker-compose.yml
├── Dockerfile
├── backend.Dockerfile
├── frontend.Dockerfile
│
├── backend/                  # Rust backend
│   ├── Cargo.toml
│   ├── .env.example
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── config.rs
│       ├── error.rs
│       ├── models.rs
│       ├── util.rs
│       ├── api/              # Platform adapters (IN DEVELOPMENT)
│       ├── handlers/         # HTTP endpoints
│       └── middleware/       # Rate limiting, etc.
│
└── frontend/                 # React + TypeScript
    ├── package.json
    ├── vite.config.ts
    ├── tsconfig.json
    ├── index.html
    └── src/
        ├── main.tsx
        ├── App.tsx
        ├── components/       # Reusable UI components
        ├── hooks/            # Custom React hooks
        ├── services/         # API communication
        ├── types/            # TypeScript definitions
        ├── styles/           # CSS styling
        └── utils/            # Helper functions
```

## Key Architecture Decisions

1. **Rust Backend** — High performance, memory-safe, ideal for streaming
2. **React Frontend** — Modern, component-driven, responsive
3. **Modular Platform Adapters** — Easy to add/remove/modify platform support
4. **Direct Streaming** — No server-side storage, scalable approach
5. **Official APIs First** — Legal compliance, with fallback options

## Environment Configuration

Copy `.env.example` to `.env` in the backend directory and add your API keys:

```bash
cp backend/.env.example backend/.env
```

Then add your platform API keys as needed.

## Testing Checklist

Before moving to the next phase, verify:

- [ ] Backend compiles without errors (`cargo build`)
- [ ] Frontend dependencies install (`npm install`)
- [ ] URL validation works correctly (test various platform URLs)
- [ ] Platform detection returns correct results
- [ ] Error messages are clear and helpful
- [ ] Responsive design works on mobile

## Contributing

When implementing platform adapters:

1. Follow the `PlatformAdapter` trait interface
2. Include proper error handling
3. Preserve audio in all downloads
4. Add unit tests for URL parsing
5. Document rate limit requirements
6. Test with real URLs

## License

MIT License — see LICENSE file for details

---

**Last Updated**: May 16, 2026  
**Status**: Foundation Complete, Ready for Platform Integration
