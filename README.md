# minigrep (Rust)

[![Rust 2024](https://img.shields.io/badge/rust-2024-orange)](https://doc.rust-lang.org/edition-guide/editions/2024.html)
[![Status: WIP](https://img.shields.io/badge/status-WIP-yellow)](#status)
[![CLI](https://img.shields.io/badge/type-CLI-informational)](#quick-start)
[![Lint: clippy](https://img.shields.io/badge/lint-clippy-green?logo=rust)](https://github.com/rust-lang/rust-clippy)
[![Style: rustfmt](https://img.shields.io/badge/style-rustfmt-blue?logo=rust)](https://github.com/rust-lang/rustfmt)

A tiny, learning‑oriented reimplementation of grep in Rust. The CLI reads a file and prints lines that contain a query string. Work in progress.

## Quick Start

- Prerequisite: Rust and Cargo installed (stable toolchain).
- Build: `cargo build`
- Run: `cargo run -- <query> <file>` (e.g., `cargo run -- to poem.txt`)
- Check types: `cargo check`
- Test: `cargo test`
- Format/Lint: `cargo fmt --all` and `cargo clippy --all-targets --all-features`

## Project Layout

- `src/main.rs`: Binary entrypoint with minimal CLI wiring.
- `src/lib.rs`: Shared logic as code grows beyond `main`.
- `tests/`: Integration tests using the public API.
- `tests/fixtures/`: Test input files.
- `target/`: Build artifacts (ignored by Git).

## Notes

- Follows Rust 2024 edition conventions; 4‑space indentation.
- Prefer `Result<T, E>` and `?` to bubble errors; avoid panics on user I/O.
- Validate CLI args and handle missing/permission errors gracefully.

## Status

This is a learning project and is actively evolving. Suggestions and small PRs are welcome.
