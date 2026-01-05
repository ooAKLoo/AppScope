FROM rust:1.83-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source files (let cargo regenerate lock file)
COPY Cargo.toml ./
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/appscope-backend /app/appscope-backend

RUN mkdir -p /app/data

ENV DATABASE_URL=sqlite:/app/data/appscope.db?mode=rwc
ENV PORT=3001

EXPOSE 3001

CMD ["/app/appscope-backend"]
