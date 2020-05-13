## warp-cors

[![ci](https://github.com/Bassetts/warp-cors/workflows/ci/badge.svg)](https://github.com/Bassetts/warp-cors/actions?query=workflow%3Aci)
[![GitHub](https://img.shields.io/github/license/bassetts/warp-cors?color=blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/warp-cors)](https://crates.io/crates/warp-cors)

warp-cors is a proxy server which enables CORS for the proxied request.

The path of the request is validated and then used as the url to proxy. Only
http and https protocols are allowed and must be specified.

Preflight requests will allow any methods and headers. All proxied requests will
have any origin allowed and allow all headers in the request to be expose in the
reponse.

### Usage

By default warp-cors will listen on port `3030` and use the package name
(`warp-cors`) as the pseudonym in the `Via` header sent with the proxied
request. These can be overwritten by using the `--port <PORT>` and
`--hostname <HOST>` flags respectively.

If you are running warp-cors on a pubicly accessible hostname it is
recommended to set the `--hostname` flag to match.

#### Running with `cargo`

```shell
# Run warp-cors with default options
$ cargo run

# Pass flags to override the defaults
$ cargo run -- --port 3000

# View help message
$ cargo run -- --help
```

#### Installing with `cargo`

```shell
# Install
$ cargo install --path .

# Run
$ warp-cors
```

### Example requests

```shell
http://localhost:3030/ # 404 Not Found
http://localhost:3030/http://example.org # Proxied HTTP response with CORS headers
http://localhost:3030/https://example.org # Proxied HTTPS response with CORS headers
http://localhost:3030/example.org # 404 Not Found (no scheme provided)
http://localhost:3030/ftp://example.org # 404 Not Found (invalid scheme provided)
```

### Logging

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
