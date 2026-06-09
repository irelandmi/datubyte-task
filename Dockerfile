# syntax=docker/dockerfile:1

# 1. Build the web UI (Vite -> frontend/dist)
FROM docker.io/library/node:22-slim AS frontend
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

# 2. Build the server binary (bundled SQLite needs the C toolchain in the rust image)
FROM docker.io/library/rust:1-bookworm AS server
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
RUN cargo build --release --bin datubyte-task-server

# 3. Slim runtime: just the binary + built UI, running as non-root
FROM docker.io/library/debian:bookworm-slim AS runtime
RUN useradd -r -u 10001 datubyte && mkdir -p /data && chown datubyte /data
COPY --from=server   /app/target/release/datubyte-task-server /usr/local/bin/datubyte-task-server
COPY --from=frontend /app/frontend/dist                       /app/frontend/dist
USER datubyte
EXPOSE 3000
VOLUME ["/data"]
ENTRYPOINT ["datubyte-task-server"]
CMD ["--port", "3000", "--db", "/data/datubyte-task.db", "--static-dir", "/app/frontend/dist"]
