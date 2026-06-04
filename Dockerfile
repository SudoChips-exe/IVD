FROM rust:1.82 AS backend-builder

WORKDIR /app

COPY backend/ .
RUN cargo build --release


FROM oven/bun:latest AS frontend-builder

WORKDIR /app

COPY frontend/package.json frontend/bun.lock ./
RUN bun install --frozen-lockfile

COPY frontend/ .
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

COPY nginx.conf /etc/nginx/conf.d/default.conf.template
RUN rm -f /etc/nginx/sites-enabled/default

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080

ENTRYPOINT ["/entrypoint.sh"]
