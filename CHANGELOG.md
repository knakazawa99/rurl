# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of Phase 1 MVP
- Support for GET, POST, PUT, DELETE, PATCH, HEAD methods
- Custom request headers (`-H`)
- Request body support (`-d`, `--data-raw`, `--json`)
- Response output to file (`-o`, `-O`)
- Verbose output (`-v`)
- Include response headers (`-i`)
- Silent mode (`-s`)
- Follow redirects (`-L`)
- Basic authentication (`-u`)
- Custom User-Agent (`-A`) and Referer (`-e`)
- TLS certificate verification skip (`-k`)
- Timeout controls (`-m`, `--connect-timeout`)
- HTTP/HTTPS proxy support (`-x`)
- curl-compatible exit codes
