FROM oven/bun:latest

WORKDIR /app

# Copy package manifest and install with Bun
COPY frontend/package*.json ./
RUN bun install

# Copy the rest of the frontend source
COPY frontend/ .

EXPOSE 5173

# Use Bun to run the dev server
CMD ["bun", "run", "dev"]
