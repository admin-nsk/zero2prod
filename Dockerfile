FROM rust:1.64.0 AS builder

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.64.0 AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configs configs
ENV APP_ENVINRONMENT production
ENTRYPOINT ["./zero2prod"]