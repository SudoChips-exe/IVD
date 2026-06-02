#!/bin/sh
set -e

# Render injects $PORT; default to 80 for local docker
PORT=${PORT:-80}

# Decode base64 cookies if provided
if [ -n "$COOKIES_B64" ]; then
    mkdir -p /root/.config/vidclaw
    echo "$COOKIES_B64" | base64 -d > /root/.config/vidclaw/cookies.txt
    echo "[vidclaw] cookies.txt loaded"
fi

# Inject $PORT into nginx config
envsubst '$PORT' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf

# Backend always runs on internal port 8080
export SERVER_HOST=0.0.0.0
export SERVER_PORT=8080

# Start nginx in background, backend in foreground
nginx -g "daemon off;" &
exec video-downloader
