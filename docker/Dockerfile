FROM rust:1.80.1 AS builder
WORKDIR /dns-updater/source
COPY /Cargo.toml .
COPY /Cargo.lock .
COPY /src ./src
RUN cargo install --locked --path .

FROM rust:1.80.1-slim AS runtime
WORKDIR /dns-updater
COPY --from=builder /dns-updater/source/target/release/dns-updater /dns-updater
COPY /configs/config.json config.json
ENV RUST_LOG=INFO
ENTRYPOINT  ["./dns-updater", "config.json"]
