FROM rust:1-bullseye AS builder
WORKDIR /app

RUN USER=root cargo new axum_mysql_app
WORKDIR /app/axum_mysql_app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY . .

RUN cargo build --release --locked

FROM debian:bullseye-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates tini \
&& rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=info
WORKDIR /app
COPY --from=builder /app/axum_mysql_app/target/release/axum_mysql_app /app/app
COPY --from=builder /app/axum_mysql_app/migrations /app/migrations

EXPOSE 8080
ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["/app/app"]