# syntax=docker/dockerfile:1
FROM rust:1.99.0-slim-trixie@sha256:2752b332db73fdbb7dc576f06c82ed1f312005784ef913d7e04a28f5f55dc581 AS build

WORKDIR /usr/src/warp-cors
COPY Cargo.lock Cargo.toml ./
COPY ./src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/warp-cors/target,sharing=locked \
    cargo build --locked --release \
    && cp target/release/warp-cors /usr/local/bin/warp-cors

FROM debian:trixie-slim@sha256:a29215f6a35e51e22adffa17f89e9d2ef06214e64a2bad10d765c46aea49f11f
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /usr/local/bin/warp-cors /usr/local/bin/warp-cors
ENV RUST_LOG="warp_cors=info"
ENTRYPOINT [ "warp-cors" ]
