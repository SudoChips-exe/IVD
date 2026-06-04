FROM rust:1.82-slim AS backend-builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache deps — build dummy binary first so layer is reused when only src changes
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo 'fn main(){}' > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY backend/src ./src
RUN touch src/main.rs && cargo build --release


FROM oven/bun:latest AS frontend-builder

WORKDIR /app

COPY frontend/package.json frontend/bun.lock ./
RUN bun install --frozen-lockfile

COPY frontend/ .
# Empty VITE_API_URL → relative URLs → nginx proxies /api/ to backend
RUN VITE_API_URL="" bun run build


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    python3 \
    python3-pip \
    ffmpeg \
    nginx \
    gettext-base \
    && pip3 install yt-dlp curl-cffi --break-system-packages \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /app/target/release/video-downloader /usr/local/bin/video-downloader
COPY --from=frontend-builder /app/dist /usr/share/nginx/html

# Store as template — entrypoint substitutes $PORT at runtime
COPY nginx.conf /etc/nginx/conf.d/default.conf.template
# Remove default nginx site
RUN rm -f /etc/nginx/sites-enabled/default

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Render sets $PORT dynamically — EXPOSE is informational only
EXPOSE 8080

ENTRYPOINT ["/entrypoint.sh"]
