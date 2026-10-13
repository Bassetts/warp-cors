# warp-cors

[![ci](https://github.com/Bassetts/warp-cors/workflows/ci/badge.svg)](https://github.com/Bassetts/warp-cors/actions?query=workflow%3Aci)
[![GitHub](https://img.shields.io/github/license/bassetts/warp-cors?color=blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/warp-cors)](https://crates.io/crates/warp-cors)

warp-cors is a proxy server which enables CORS for the proxied request.

The path of the request is validated and then used as the url to proxy. Only
http and https protocols are allowed and must be specified.

Preflight `OPTIONS` requests are answered locally by echoing
`Access-Control-Request-Method` and `Access-Control-Request-Headers` into
`Access-Control-Allow-Methods` and `Access-Control-Allow-Headers`. There is no
method or header allowlist. Currently, both request headers must be present for
the request to be handled as a preflight; otherwise it is proxied upstream.

Successful proxy and preflight responses echo a valid serialized request
`Origin` (such as `https://example.org` or the opaque origin `null`) in
`Access-Control-Allow-Origin` and set `Access-Control-Allow-Credentials: true`.
When no `Origin` is supplied, they use `Access-Control-Allow-Origin: *` and omit
`Access-Control-Allow-Credentials`. Invalid origins receive neither header.
Responses include `Vary: Origin`, preserving any upstream `Vary` fields; an
existing `Vary: *` already covers all request headers.

Proxied responses list the forwarded upstream **response header names** in
`Access-Control-Expose-Headers`, making those headers readable by browser
JavaScript. Upstream `Set-Cookie` headers are removed, and request `Cookie`
headers are not forwarded upstream.

## Installation

### Binaries

[Archives of precompiled binaries](https://github.com/Bassetts/warp-cors/releases)
are available for Linux, macOS, and Windows.

The Windows binaries require [Microsoft Visual C++ 2015 Redistributable](https://www.microsoft.com/en-us/download/details.aspx?id=53587)
to be installed.

### Cargo

The minimum supported Rust version (MSRV) is **1.88.0**.

```shell
$ cargo install warp-cors
```

### Docker

Docker images are published in the
[bassetts/warp-cors](https://hub.docker.com/repository/docker/bassetts/warp-cors)
repository.

```shell
$ docker run -dp 3030:3030 bassetts/warp-cors
```

To build the image locally:

```shell
$ docker build -t warp-cors .
```

The Dockerfile pins the Rust compiler and both base images by digest, and builds
with `cargo build --locked --release` to use the committed `Cargo.lock`.
BuildKit caches downloaded crates and compiled dependencies between builds;
these caches are excluded from the runtime image.

The runtime image contains Debian slim, the stripped executable, and a CA
certificate bundle copied from the builder for HTTPS requests. No packages are
downloaded during runtime image assembly. The existing weekly Docker Dependabot
configuration proposes base image updates; merging them refreshes the image
packages and certificate bundle.
These pins make build inputs repeatable; image timestamps and build metadata
can still differ between builds.

## Usage

By default warp-cors will listen on port `3030` and use the package name
(`warp-cors`) as the pseudonym in the `Via` header sent with the proxied
request. These can be overwritten by using the `--port <PORT>` and
`--hostname <HOST>` flags respectively.

If you are running warp-cors on a pubicly accessible hostname it is
recommended to set the `--hostname` flag to match.

#### Example

```shell
$ warp-cors --port 3000 --hostname cors.example.org
```

## Example requests

```shell
http://localhost:3030/ # 404 Not Found
http://localhost:3030/http://example.org # Proxied HTTP response with CORS headers
http://localhost:3030/https://example.org # Proxied HTTPS response with CORS headers
http://localhost:3030/example.org # 404 Not Found (no scheme provided)
http://localhost:3030/ftp://example.org # 404 Not Found (invalid scheme provided)
```

## Logging

warp-cors uses [`pretty_env_logger`], so you can control log levels by setting
the `RUST_LOG` environment variable. This is useful for both developing and
for running warp-cors.

```shell
# Output all info level logs, including those from libraries used by warp-cors
$ RUST_LOG=info warp-cors

# Output info level logs for only warp-cors
$ RUST_LOG=warp_cors=info warp-cors

# Filter logging by module, useful for development or debugging
$ RUST_LOG=warp_cors::filters::request=trace warp-cors
```

[`pretty_env_logger`]: https://docs.rs/pretty_env_logger/*/pretty_env_logger/
