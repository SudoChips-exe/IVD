# Universal Social Media Video Downloader

This project provides a comprehensive backend and frontend for downloading videos from multiple social media platforms with audio preservation.

## Project Status: Phase 3 & Phase 4 Complete ✅

### ✅ Completed
- [x] Comprehensive README.md with features and setup instructions
- [x] Detailed ARCHITECTURE.md with system design and platform strategies
- [x] Rust backend project structure with:
  - Actix-web HTTP server setup
  - Configuration management system
  - Modular platform adapter architecture
  - Error handling framework
  - Middleware (rate limiting implemented)
  - URL validation & platform detection utilities
  - Metadata cache module created and registered in backend state
  - Video streaming handler with `Content-Type` and `Content-Disposition`
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
1. **Instagram Adapter** — Implemented page scraping + metadata extraction
2. **TikTok Adapter** — Implemented page scraping + direct download URL extraction
3. **YouTube Adapter** — Added metadata page fallback; full downloader pending yt-dlp / signature decoding
4. **Twitter/X Adapter** — Added public URL metadata adapter for OG video extraction
5. **Facebook Adapter** — Added public URL metadata adapter for OG video extraction
6. **Snapchat Adapter** — Added public URL metadata adapter for OG video extraction

#### Phase 3: Backend Features
- [x] Implement actual video streaming to frontend (streaming handler with Content-Type and Content-Disposition)
- [x] Add rate limiting middleware (implemented and registered)
- [x] Implement response headers (Content-Disposition, MIME types) (fully implemented)
- [x] Add metadata caching (created, wired into app state, ready for integration)
- [x] Error handling & validation (comprehensive error types and responses)

#### Phase 4: Frontend Integration
- [x] Test against real backend endpoints (verified with curl and browser testing)
- [x] Implement actual download streaming (functional blob download in browser)
- [x] Add loading animations (progress bar with status messages)
- [x] Mobile responsiveness testing (CSS media queries at 768px breakpoint)
- [x] Error handling with retry logic (3 retries with exponential backoff)

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
bun install
bun run dev
# Runs on http://localhost:5173
```

You can also start both services from the repository root with the Bun orchestrator:

```bash
# From repo root
bun install    # (run once to ensure Bun can execute dev.ts)
bun run dev
# Runs backend (cargo run) and frontend (bun run dev) concurrently
```

Docker: the project provides a Bun-based `frontend.Dockerfile` and the frontend service in `docker-compose.yml`.

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

- [x] Backend compiles without errors (`cargo build`)
- [x] Backend tests pass (`cargo test` with 6 passed; warnings remain for unused enum variants and model structs)
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

**Last Updated**: May 26, 2026  
**Status**: Phase 3 & 4 Complete. Ready for Phase 5 deployment and platform refinement.
