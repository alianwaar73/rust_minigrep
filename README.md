# minigrep (Rust)

[![Rust 2024](https://img.shields.io/badge/rust-2024-orange)](https://doc.rust-lang.org/edition-guide/editions/2024.html)
[![Status: WIP](https://img.shields.io/badge/status-WIP-yellow)](#status)
[![CLI](https://img.shields.io/badge/type-CLI-informational)](#quick-start)
[![Lint: clippy](https://img.shields.io/badge/lint-clippy-green?logo=rust)](https://github.com/rust-lang/rust-clippy)
[![Style: rustfmt](https://img.shields.io/badge/style-rustfmt-blue?logo=rust)](https://github.com/rust-lang/rustfmt)

A tiny, learning‑oriented reimplementation of grep in Rust. Currently echoes the query and file path, then prints the file contents. Search/filtering will be added as the project evolves.

## Quick Start

- Prerequisite: Rust and Cargo installed (stable toolchain).
- Build: `cargo build`
- Run: `cargo run -- <query> <file>` (e.g., `cargo run -- to poem.txt`)
- Check types: `cargo check`
- Test: `cargo test`
- Format/Lint: `cargo fmt --all` and `cargo clippy --all-targets --all-features`

## Usage

Example run against a file `poem.txt`:

```
cargo run -- to poem.txt

Input query: to
Path to file: poem.txt
Containing contents:

<file contents are printed here>
```

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

## Contributing

See `CONTRIBUTING.md` for commit/PR conventions, testing guidance, and tooling. Small improvements and learning notes are welcome.

## Changelog

See `CHANGELOG.md` for a history of notable documentation and code updates with commit references.

## Status

This is a learning project and is actively evolving. Search logic is not yet implemented; current binary focuses on argument parsing and file I/O.

— README created/updated by codex-cli
