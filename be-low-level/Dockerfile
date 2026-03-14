# Stage 1: Build
FROM rust:latest AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Run
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/be-low-level /usr/local/bin/be-low-level
CMD ["be-low-level"]
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

WORKDIR /app
COPY --from=builder /app/target/release/be-low-level .
EXPOSE 4000
ENTRYPOINT ["./be-low-level"]
