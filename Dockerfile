# syntax=docker/dockerfile:1
FROM rust:1.99.0-slim-trixie@sha256:2752b332db73fdbb7dc576f06c82ed1f312005784ef913d7e04a28f5f55dc581 AS build

WORKDIR /usr/src/warp-cors
COPY Cargo.lock Cargo.toml ./
COPY ./src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/warp-cors/target,sharing=locked \
    cargo build --locked --release \
    && cp target/release/warp-cors /usr/local/bin/warp-cors

FROM debian:trixie-slim@sha256:a72b4433de1cf7d6bae0c4cee74d0547f58601a643b3375f070c51600d63b6b2
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /usr/local/bin/warp-cors /usr/local/bin/warp-cors
ENV RUST_LOG="warp_cors=info"
ENTRYPOINT [ "warp-cors" ]
