FROM rust:1.83-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/appscope-backend /app/appscope-backend

RUN mkdir -p /app/data

ENV DATABASE_URL=sqlite:/app/data/appscope.db?mode=rwc
ENV PORT=3001

EXPOSE 3001

CMD ["/app/appscope-backend"]
