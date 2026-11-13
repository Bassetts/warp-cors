# syntax=docker/dockerfile:1
FROM rust:1.99.1-slim-trixie@sha256:f6c4a35d21dbb3c89d7a8d056c193e94fc09037f2b58dd858b09c39a47dd02d7 AS build

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
