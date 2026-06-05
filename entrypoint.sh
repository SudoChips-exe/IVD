#!/bin/sh
set -e

# Render injects $PORT; default to 8080 for local docker
PORT=${PORT:-8080}

# Diagnostic: verify pot plugins are installed and config is correct
echo "[vidclaw] installed yt-dlp packages:"
pip3 list 2>/dev/null | grep -iE 'yt-dlp|bgutil|get-pot' || echo "  (none found)"
echo "[vidclaw] yt-dlp config:"
cat /root/.config/yt-dlp/config 2>/dev/null || echo "  (no config file)"
echo "[vidclaw] bgutil server dir:"
ls /opt/bgutil-pot/server/build/ 2>/dev/null || echo "  (build dir missing)"

# Decode base64 cookies if provided
if [ -n "$COOKIES_B64" ]; then
    mkdir -p /root/.config/vidclaw
    echo "$COOKIES_B64" | base64 -d > /root/.config/vidclaw/cookies.txt
    echo "[vidclaw] cookies.txt loaded"
fi

# nginx takes the external port; backend runs on fixed internal port 8081
export BACKEND_PORT=8081
export SERVER_HOST=0.0.0.0
export SERVER_PORT=8081

# Inject $PORT and $BACKEND_PORT into nginx config
envsubst '$PORT $BACKEND_PORT' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf

# Start nginx in background, backend in foreground
nginx -g "daemon off;" &
exec video-downloader
