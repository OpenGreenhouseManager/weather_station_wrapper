# Multi-stage, minimal, non-root runtime

# -------- Builder --------
FROM rust:1.82-slim AS builder

# Install build dependencies (just in case native deps are needed)
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       pkg-config \
       libssl-dev \
       ca-certificates \
       git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Leverage caching by copying manifests first
COPY Cargo.toml Cargo.lock ./
COPY ambient_weather/Cargo.toml ./ambient_weather/Cargo.toml

# Create dummy src files to cache dependencies
RUN mkdir -p src ambient_weather/src \
    && echo "fn main() {}" > src/main.rs \
    && echo "pub fn _dummy() {}" > ambient_weather/src/lib.rs

# Fetch and build deps
RUN cargo build --release --locked || true

# Now copy real sources
COPY . .

# Build the actual binary
RUN cargo build --release --locked

# -------- Runtime --------
FROM debian:bookworm-slim AS runtime

# Minimal runtime requirements
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user and group
RUN groupadd -r app && useradd -r -u 10001 -g app appuser

# Create config directory (can be mounted)
RUN mkdir -p /config \
    && chown -R appuser:app /config

# Copy binary
COPY --from=builder /app/target/release/weather_station_wrapper /usr/local/bin/weather_station_wrapper

# Use non-root user
USER appuser

# Default config path inside container (can be overridden by providing a different argument)
ENV RUST_LOG=info

# Expose the default port; actual port can be changed via config file
EXPOSE 6001

# By default, the app reads a config path as first CLI argument; default to /config/config.json
ENTRYPOINT ["/usr/local/bin/weather_station_wrapper"]
CMD ["/config/config.json"]

# Optional: Uncomment if you later add a health endpoint
# HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
#   CMD wget -qO- http://localhost:6001/health || exit 1


