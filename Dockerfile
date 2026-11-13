# syntax=docker/dockerfile:1
FROM rust:1.100.0-slim-trixie@sha256:e9d95b74296f7126a83ae3d041aeaba1ff4c478f13abc5cc034fcf7c131c3d4a AS build

WORKDIR /usr/src/warp-cors
COPY Cargo.lock Cargo.toml ./
COPY ./src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/warp-cors/target,sharing=locked \
    cargo build --locked --release \
    && cp target/release/warp-cors /usr/local/bin/warp-cors

FROM debian:trixie-slim@sha256:a72b4433de1cf7d6bae0c4cee74d0547f58601a643b3375f070c51600d63b6b2
RUN groupadd --gid 10001 warp-cors \
    && useradd --uid 10001 --gid warp-cors --no-create-home --no-log-init --shell /usr/sbin/nologin warp-cors
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /usr/local/bin/warp-cors /usr/local/bin/warp-cors
ENV RUST_LOG="warp_cors=info"
USER 10001:10001
ENTRYPOINT [ "warp-cors" ]
