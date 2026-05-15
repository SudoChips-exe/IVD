# Architecture & Design Document

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     USER BROWSER (Frontend)                     │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │   React + TypeScript + Vite                             │  │
│  │  ┌─────────────────────────────────────────────────┐    │  │
│  │  │ URL Input Component → Validation → Download    │    │  │
│  │  │ Trigger HTTP POST /api/download                │    │  │
│  │  └─────────────────────────────────────────────────┘    │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────┬───────────────────────────────────────────┘
                     │ HTTP POST with URL
                     ▼
┌─────────────────────────────────────────────────────────────────┐
│            BACKEND SERVER (Rust + Actix-web)                   │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ HTTP Request Handler                                    │  │
│  │ ├─ Validate URL format & platform                      │  │
│  │ ├─ Rate limiting check                                 │  │
│  │ └─ Route to appropriate platform adapter              │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Platform Adapters (Modular)                            │  │
│  │ ├─ Instagram Adapter    → Instagram API calls         │  │
│  │ ├─ TikTok Adapter       → TikTok API calls            │  │
│  │ ├─ YouTube Adapter      → YouTube Data API calls      │  │
│  │ ├─ Twitter Adapter      → Twitter API v2 calls        │  │
│  │ ├─ Facebook Adapter     → Facebook Graph API calls    │  │
│  │ └─ Snapchat Adapter     → Snapchat API calls          │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Video Processing & Streaming                           │  │
│  │ ├─ Fetch video metadata (title, duration, codecs)     │  │
│  │ ├─ Retrieve direct video URL with audio stream        │  │
│  │ ├─ Stream video bytes to client (no disk storage)     │  │
│  │ └─ Set HTTP headers (Content-Disposition, MIME type) │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                     │ HTTP Response (Video Stream)
                     ▼
┌─────────────────────────────────────────────────────────────────┐
│              USER'S DEVICE (Browser Download)                   │
│         Video saved locally with audio preserved                │
└─────────────────────────────────────────────────────────────────┘
```

---

## Backend Architecture (Rust)

### Project Structure

```
backend/
├── Cargo.toml                 # Rust dependencies & metadata
├── Cargo.lock                 # Dependency lock file
├── .env.example               # Environment variables template
├── src/
│   ├── main.rs               # Application entry point & server setup
│   ├── lib.rs                # Library exports
│   ├── config.rs             # Configuration management (env vars, API keys)
│   ├── error.rs              # Error types & handling
│   ├── models.rs             # Data structures (URL, VideoMetadata, etc.)
│   ├── api/
│   │   ├── mod.rs            # API module exports
│   │   ├── instagram.rs       # Instagram video extraction
│   │   ├── tiktok.rs          # TikTok video extraction
│   │   ├── youtube.rs         # YouTube video extraction
│   │   ├── twitter.rs         # Twitter/X video extraction
│   │   ├── facebook.rs        # Facebook video extraction
│   │   ├── snapchat.rs        # Snapchat video extraction
│   │   └── common.rs          # Shared utilities (HTTP client, etc.)
│   ├── handlers/
│   │   ├── mod.rs            # Handler module exports
│   │   ├── download.rs        # POST /api/download handler
│   │   └── health.rs          # GET /health handler
│   ├── middleware/
│   │   ├── mod.rs            # Middleware exports
│   │   ├── rate_limit.rs      # Rate limiting middleware
│   │   └── logging.rs         # Request logging middleware
│   └── util.rs               # Helper functions (URL validation, etc.)
├── tests/
│   ├── integration_tests.rs  # End-to-end tests
│   └── platform_tests.rs     # Per-platform adapter tests
└── Dockerfile                # Container configuration
```

### Key Components

#### 1. **main.rs** — Application Bootstrap
```rust
// Initializes Actix-web server
// Configures routes and middleware
// Starts listening on PORT (default 8080)
```

#### 2. **config.rs** — Configuration Management
- Loads environment variables
- Validates required API keys
- Provides configuration to handlers

#### 3. **models.rs** — Data Structures

```rust
// Represents a download request
pub struct DownloadRequest {
    pub url: String,           // Social media video URL
    pub platform: Platform,    // Detected platform (Instagram, TikTok, etc.)
}

// Represents video metadata
pub struct VideoMetadata {
    pub title: String,
    pub duration_seconds: u32,
    pub author: String,
    pub video_url: String,           // Direct URL to video file
    pub audio_url: Option<String>,   // Separate audio stream (if needed)
    pub thumbnail_url: String,
    pub original_platform: Platform,
}

// Platform enum
pub enum Platform {
    Instagram,
    TikTok,
    YouTube,
    Twitter,
    Facebook,
    Snapchat,
    Unknown,
}
```

#### 4. **handlers/download.rs** — Main Download Endpoint

**Endpoint**: `POST /api/download`

**Request Body**:
```json
{
  "url": "https://www.instagram.com/p/ABC123/"
}
```

**Response Flow**:
1. Validate URL format
2. Detect platform
3. Fetch video metadata from platform's API
4. Stream video bytes with correct Content-Disposition header
5. Browser downloads file automatically

**Response Headers** (on success):
```
Content-Type: video/mp4
Content-Disposition: attachment; filename="instagram_video_ABC123.mp4"
Content-Length: 5242880
```

#### 5. **api/{platform}.rs** — Platform Adapters

Each platform adapter must implement:

```rust
pub trait PlatformAdapter {
    async fn validate_url(&self, url: &str) -> Result<bool>;
    async fn fetch_metadata(&self, url: &str) -> Result<VideoMetadata>;
    async fn get_download_url(&self, url: &str) -> Result<String>;
}

// Example implementation for Instagram
impl PlatformAdapter for InstagramAdapter {
    async fn validate_url(&self, url: &str) -> Result<bool> {
        // Check if URL matches Instagram patterns
        // https://www.instagram.com/p/{ID}/
        // https://www.instagram.com/reel/{ID}/
    }
    
    async fn fetch_metadata(&self, url: &str) -> Result<VideoMetadata> {
        // Call Instagram API or scrape metadata
        // Extract video URL, title, duration, etc.
    }
    
    async fn get_download_url(&self, url: &str) -> Result<String> {
        // Return direct MP4 URL (with audio)
    }
}
```

---

## Frontend Architecture (React + TypeScript)

### Project Structure

```
frontend/
├── package.json              # Node.js dependencies
├── tsconfig.json             # TypeScript configuration
├── vite.config.ts            # Vite bundler configuration
├── index.html                # Entry HTML file
├── src/
│   ├── main.tsx              # React app entry point
│   ├── App.tsx               # Root component
│   ├── components/
│   │   ├── URLInput.tsx       # URL input field & validation
│   │   ├── DownloadButton.tsx # Download trigger button
│   │   ├── ProgressBar.tsx    # Download progress indicator
│   │   ├── ErrorMessage.tsx   # Error display component
│   │   ├── Header.tsx         # Top navigation
│   │   └── Footer.tsx         # Footer with links
│   ├── pages/
│   │   ├── HomePage.tsx       # Main landing page
│   │   └── FAQPage.tsx        # FAQ & help page
│   ├── hooks/
│   │   ├── useDownload.ts     # Custom hook for download logic
│   │   └── useValidation.ts   # URL validation hook
│   ├── services/
│   │   └── api.ts             # HTTP client for backend communication
│   ├── types/
│   │   └── index.ts           # TypeScript type definitions
│   ├── styles/
│   │   ├── App.css            # App styling
│   │   └── components.css     # Component-specific styles
│   └── utils/
│       ├── urlDetection.ts    # Identify platform from URL
│       └── formatters.ts      # Format file names, sizes, etc.
└── public/
    ├── favicon.ico
    └── logo.svg
```

### User Flow

```
User Opens App
    ↓
[Home Page Rendered]
    ↓
User Pastes URL in URLInput
    ↓
URLInput validates format (is it a valid social media URL?)
    ↓
User Clicks "Download" Button
    ↓
DownloadButton calls useDownload hook
    ↓
API Service sends POST to backend
    ↓
ProgressBar shows "Fetching video..." status
    ↓
Backend responds with video stream
    ↓
ProgressBar shows "Downloading..." (with progress %)
    ↓
Browser's download manager saves file
    ↓
ProgressBar shows "Complete!" ✓
    ↓
User's file is ready to use
```

### Key Components

#### **URLInput.tsx**
- Text field with paste support
- Real-time URL validation
- Platform detection (shows icon for detected platform)
- Clear error messages for invalid URLs

#### **useDownload.ts** Hook
```typescript
const useDownload = () => {
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);

  const download = async (url: string) => {
    setLoading(true);
    setError(null);
    try {
      const response = await api.downloadVideo(url);
      // Trigger browser download
      const blob = await response.blob();
      const filename = extractFilename(response.headers);
      triggerDownload(blob, filename);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return { download, loading, progress, error };
};
```

#### **api.ts** Service
```typescript
export const api = {
  async downloadVideo(url: string) {
    const response = await fetch('http://localhost:8080/api/download', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ url }),
    });
    if (!response.ok) throw new Error(await response.text());
    return response;
  },
};
```

---

## API Strategy & Platform Coverage

### Approach: Official APIs First, Fallback to Custom Extraction

| Platform | Official API | Authentication | Rate Limit | Video Download? | Fallback Plan |
|----------|--------------|-----------------|-----------|-----------------|---------------|
| **Instagram** | Graph API | OAuth + AppID | 100/hour | Limited | `instagrapi` library |
| **TikTok** | TikTok API | OAuth | 50/hour | No | `TikTok-Api-Sharp` |
| **YouTube** | Data API v3 | API Key | 10k quota/day | No (ToS) | `yt-dlp` library |
| **Twitter/X** | API v2 | Bearer Token | 450/15min | Yes | `tweepy` wrapper |
| **Facebook** | Graph API | App Token | 200/hour | Limited | Custom URL parsing |
| **Snapchat** | Not Public | ❌ | N/A | N/A | Research required |

**Key Decision**: For platforms with restrictive APIs (Instagram, TikTok, YouTube), we'll use well-maintained open-source libraries wrapped in Rust via FFI or rewritten in Rust for better integration.

---

## Error Handling Strategy

```
User Error (4xx)
├─ Invalid URL format
├─ URL not recognized (not a video)
├─ Private/deleted video
├─ Platform not supported
└─ User rate-limited

Platform Error (5xx)
├─ API rate limit hit
├─ API authentication failure
├─ Video extraction failed
├─ Audio stream not available
└─ Platform API down

Server Error (5xx)
├─ Internal server error
├─ Out of memory
├─ Network connectivity issue
└─ Streaming interrupted

Response Format (all errors):
{
  "error": "ERROR_CODE",
  "message": "Human-readable error message",
  "retry_after": 60  // seconds until retry (if rate-limited)
}
```

---

## Deployment Architecture

### Local Development
```bash
# Terminal 1: Backend
cd backend && cargo run

# Terminal 2: Frontend
cd frontend && npm run dev

# Browser: http://localhost:5173
```

### Docker (Single Container)
```dockerfile
FROM rust:latest AS builder
WORKDIR /app
COPY backend/ .
RUN cargo build --release

FROM node:18 AS frontend_builder
WORKDIR /app
COPY frontend/ .
RUN npm install && npm run build

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/video-downloader /usr/local/bin/
COPY --from=frontend_builder /app/dist /static/
CMD ["video-downloader"]
```

### Cloud Deployment (AWS Example)
- **Backend**: EC2 instance or ECS Fargate container
- **Frontend**: S3 + CloudFront (static hosting)
- **Database**: (Optional) DynamoDB for caching metadata
- **Load Balancer**: Application Load Balancer for scaling

---

## Performance Considerations

### Memory Efficiency
- **Streaming approach**: Video never stored in RAM; data flows directly from source → client
- **Per-request memory**: ~10-50MB for metadata + headers
- **Concurrent connections**: Limited by OS file descriptors & network bandwidth

### Rate Limiting
```
Global: 60 requests/minute
Per-IP: 30 requests/minute
Per-platform: 
  - Instagram: 100/hour
  - TikTok: 50/hour
  - YouTube: 10k quota/day (shared)
```

### Caching Strategy
- **Metadata cache**: 15-minute TTL (same URL → avoid re-fetching from API)
- **Platform status cache**: 1-hour TTL (detect when platform is down)
- **Client-side**: Disable browser caching for downloads

---

## Security Considerations

1. **Input Validation**: Strict URL format validation to prevent injection attacks
2. **Rate Limiting**: Prevent DDoS and API quota exhaustion
3. **CORS Configuration**: Only allow requests from known frontend domain
4. **API Key Storage**: Use environment variables, never commit to git
5. **HTTPS Enforcement**: All production requests encrypted
6. **Download Sandboxing**: Files never execute; served as attachments

---

## Testing Strategy

### Unit Tests (Rust Backend)
- Test each platform adapter independently
- Mock API responses
- Verify URL validation logic

### Integration Tests (End-to-End)
- Test full flow: URL input → metadata fetch → download stream
- Use test URLs from each platform
- Verify audio is preserved

### Frontend Tests (React)
- Component rendering tests
- Error state handling
- Download progress visualization

---

## Maintenance & Monitoring

### Logging
- All API calls logged (timestamp, platform, status)
- Error logs with stack traces
- Rate limit hits tracked

### Metrics
- Requests per minute
- Average download size & duration
- Error rates per platform
- API quota usage

### Alerting
- Platform API goes down → alert
- Rate limits reached → alert
- Errors >5% → alert

---

## Future Enhancements

1. **Batch Downloads**: Download multiple videos in queue
2. **Playlist Support**: Download entire TikTok/YouTube playlists
3. **Format Selection**: Let users choose MP4, WebM, audio-only (MP3)
4. **Metadata Editing**: Allow users to add/edit title, tags before download
5. **Desktop App**: Electron wrapper for Windows/Mac/Linux
6. **Browser Extension**: One-click download from social media pages
7. **History & Favorites**: Track downloaded videos (opt-in)
8. **Video Conversion**: Convert to other formats on-the-fly

---

## Success Criteria (MVP)

✅ Download video from any of 6 platforms by pasting URL  
✅ Audio is always preserved in output  
✅ No intermediate server storage (stream directly)  
✅ Works on mobile browser  
✅ Handles errors gracefully with clear messages  
✅ Supports 1000+ concurrent downloads  
✅ Rate limits are respected per platform  
✅ Deployment is automated (Docker/cloud-ready)
