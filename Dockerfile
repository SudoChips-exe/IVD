FROM rust:bookworm AS backend-builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY backend/ .
ENV CARGO_BUILD_JOBS=1
RUN cargo build --release --locked


FROM oven/bun:latest AS frontend-builder

WORKDIR /app

COPY frontend/package.json frontend/bun.lock ./
RUN bun install --frozen-lockfile

COPY frontend/ .
RUN VITE_API_URL="" bun run build


FROM debian:bookworm-slim

# Install system deps + Node.js 20 (required by bgutil pot generator)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    git \
    python3 \
    python3-pip \
    ffmpeg \
    nginx \
    gettext-base \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Build bgutil pot provider scripts (generates YouTube po_tokens to bypass datacenter bot detection)
RUN git clone --depth 1 https://github.com/Brainicism/bgutil-ytdlp-pot-provider.git /opt/bgutil-pot \
    && cd /opt/bgutil-pot/server \
    && npm ci \
    && npx tsc \
    && npm prune --production \
    && rm -rf /opt/bgutil-pot/.git /root/.npm

# Install Python packages: yt-dlp, curl-cffi (TikTok), get-pot framework + bgutil provider
RUN pip3 install yt-dlp curl-cffi yt-dlp-get-pot bgutil-ytdlp-pot-provider --break-system-packages

# Configure yt-dlp to use bgutil script mode (server_home = repo root, plugin appends server/build/)
# Two-line format avoids shell quoting issues in yt-dlp config parser
RUN mkdir -p /root/.config/yt-dlp \
    && printf '%s\n%s\n' '--extractor-args' \
       'youtubepot-bgutilscript:server_home=/opt/bgutil-pot' \
    > /root/.config/yt-dlp/config

COPY --from=backend-builder /app/target/release/video-downloader /usr/local/bin/video-downloader
COPY --from=frontend-builder /app/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf.template
RUN rm -f /etc/nginx/sites-enabled/default

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 8080

ENTRYPOINT ["/entrypoint.sh"]
