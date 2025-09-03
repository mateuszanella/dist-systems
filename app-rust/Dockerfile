FROM rust:1-bullseye AS builder
WORKDIR /app

RUN USER=root cargo new axum_mysql_app
WORKDIR /app/axum_mysql_app

COPY . .

RUN cargo build --release --locked

FROM debian:bullseye-slim AS runtime

ENV RUST_LOG=info

WORKDIR /app

COPY --from=builder /app/axum_mysql_app/target/release/axum_mysql_app /app/app

EXPOSE 8080
CMD ["/app/app"]
