FROM rust:1.80.1 as builder
WORKDIR /dns-updater/source
COPY /Cargo.toml .
COPY /Cargo.lock .
COPY /src ./src
RUN cargo install --locked --path .

FROM debian:stable-slim AS runtime
WORKDIR /dns-updater
COPY --from=builder /dns-updater/source/target/release/dns-updater /dns-updater
COPY /configs/config.json config.json

ENTRYPOINT  ["./dns-updater", "config.json"]
