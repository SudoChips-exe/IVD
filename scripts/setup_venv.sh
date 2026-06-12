#!/usr/bin/env bash
set -e

VENV_DIR="${YTDLP_VENV:-$HOME/.local/share/vidclaw/venv}"

echo "Setting up yt-dlp virtual environment at: $VENV_DIR"

python3 -m venv "$VENV_DIR"
"$VENV_DIR/bin/pip" install --upgrade pip --quiet
# curl-cffi 0.10.x–0.14.x required — 0.15+ breaks yt-dlp impersonation
"$VENV_DIR/bin/pip" install yt-dlp 'curl-cffi>=0.10,<0.15' --quiet

echo ""
echo "Done!"
echo ""
echo "  yt-dlp: $VENV_DIR/bin/yt-dlp"
echo "  version: $("$VENV_DIR/bin/yt-dlp" --version)"
echo ""
echo "Impersonation targets (TikTok support):"
"$VENV_DIR/bin/yt-dlp" --list-impersonate-targets 2>/dev/null | grep -v "unavailable" || echo "  (none available)"
echo ""
echo "The backend will auto-detect this venv. Restart the backend to apply."
