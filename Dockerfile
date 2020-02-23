FROM rust:slim AS build

RUN USER=root cargo new /usr/src/warp-cors
WORKDIR /usr/src/warp-cors
COPY Cargo.lock Cargo.toml ./
RUN cargo install --path .

RUN rm ./target/release/deps/warp_cors*
RUN rm -rf ./src
COPY ./src ./src
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/warp-cors /usr/local/bin/warp-cors
ENV RUST_LOG="warp_cors=info"
ENTRYPOINT [ "warp-cors" ]