# Builder
FROM rust:1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runner
FROM debian:bookworm-slim AS runner
RUN apt update
RUN apt install openssl ca-certificates -y

COPY --from=builder /app/target/release/proofreader-fallback /usr/local/bin/proofreader-fallback
COPY --from=builder /app/static /static

CMD ["proofreader-fallback"]
