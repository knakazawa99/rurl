# rurl

A curl-compatible HTTP client written in pure Rust.

[![CI](https://github.com/kensukenakazawa/rurl/actions/workflows/ci.yml/badge.svg)](https://github.com/kensukenakazawa/rurl/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/rurl)](https://crates.io/crates/rurl)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Features

- Drop-in curl alternative for common HTTP operations
- Pure Rust with `rustls` TLS backend (no OpenSSL dependency)
- Supports HTTP/HTTPS
- Compatible with major curl flags

## Installation

### From crates.io

```bash
cargo install rurl
```

### From GitHub Releases

Download the pre-built binary for your platform from the [Releases page](https://github.com/kensukenakazawa/rurl/releases).

### From source

```bash
cargo install --path .
```

## Usage

```bash
# Basic GET request
rurl https://httpbin.org/get

# POST with JSON body
rurl -X POST --json '{"key":"value"}' https://httpbin.org/post

# POST with form data
rurl -X POST -d 'field=value' https://httpbin.org/post

# With custom headers
rurl -H "Authorization: Bearer token" https://httpbin.org/get

# Verbose output (shows request/response headers)
rurl -v https://httpbin.org/get

# Follow redirects
rurl -L https://httpbin.org/redirect/3

# Save response to file
rurl -o output.html https://example.com

# Basic authentication
rurl -u username:password https://httpbin.org/basic-auth/username/password

# Skip TLS verification
rurl -k https://self-signed.example.com

# Set timeout
rurl -m 10 https://httpbin.org/delay/5
```

## Supported Flags (Phase 1)

| Flag | Description |
|------|-------------|
| `-X, --request METHOD` | HTTP method (GET, POST, PUT, DELETE, etc.) |
| `-H, --header HEADER` | Custom request header (repeatable) |
| `-d, --data DATA` | Request body (`@file` reads from file) |
| `--data-raw DATA` | Request body (no `@file` expansion) |
| `--json DATA` | JSON body with automatic Content-Type/Accept headers |
| `-o, --output FILE` | Write response to file |
| `-O, --remote-name` | Write response to file named from URL |
| `-i, --include` | Include response headers in output |
| `-I, --head` | Send HEAD request |
| `-s, --silent` | Silent mode (no progress or errors) |
| `-v, --verbose` | Verbose output |
| `-L, --location` | Follow redirects |
| `-u, --user USER:PASS` | Basic authentication |
| `-A, --user-agent STRING` | User-Agent string |
| `-e, --referer URL` | Referer header |
| `-k, --insecure` | Skip TLS certificate verification |
| `-m, --max-time SECONDS` | Maximum time for operation |
| `--connect-timeout SECONDS` | Connection timeout |
| `-x, --proxy URL` | HTTP/HTTPS proxy |

## Exit Codes

Follows curl-compatible exit codes:

| Code | Meaning |
|------|---------|
| 0 | Success |
| 6 | DNS resolution failed |
| 7 | Connection refused |
| 23 | Write error |
| 28 | Timeout |
| 35 | SSL/TLS error |
| 47 | Too many redirects |

## License

MIT License. See [LICENSE](LICENSE) for details.
