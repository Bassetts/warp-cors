FROM rust:alpine AS build
RUN apk add --no-cache build-base
WORKDIR /usr/src/warp-cors
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

FROM alpine:3.17
RUN apk add --no-cache tini
COPY --from=build /usr/src/warp-cors/target/release/warp-cors /usr/local/bin/warp-cors
ENV RUST_LOG="warp_cors=info"
USER 1000:1000
ENTRYPOINT ["/sbin/tini", "--", "/usr/local/bin/warp-cors"]
