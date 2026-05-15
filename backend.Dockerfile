# Backend Dockerfile for development
FROM rust:1.70

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy backend files
COPY backend/ .

# Build
RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/video-downloader"]
