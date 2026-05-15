FROM oven/bun:latest AS builder

WORKDIR /app/frontend

# Copy frontend package manifest and install deps with Bun
COPY frontend/package*.json ./
RUN bun install

# Copy frontend source and build
COPY frontend/ .
RUN bun run build

# Stage 2: Rust backend
FROM rust:latest AS rust-builder

WORKDIR /app/backend

COPY backend/Cargo.toml Cargo.lock ./

COPY backend/src ./src

RUN cargo build --release

# Stage 3: Final image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/frontend/dist /static

COPY --from=rust-builder /app/backend/target/release/video-downloader /usr/local/bin/

EXPOSE 8080

CMD ["video-downloader"]
