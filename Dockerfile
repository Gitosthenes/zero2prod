# Setup
FROM lukemathwalker/cargo-chef AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

# Prep dependency skeleton
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build release Rust binary for runtime
FROM chef AS builder
# Get dependency skeleton
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
ENV SQL_Offline true
RUN cargo build --release

# Build runtime image
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy compiled binary from builder env, ignoing build artifacts
COPY --from=builder /app/target/release/zero2prod zero2prod
# Copy configuration directory
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]