# minigrep (Rust)

[![Rust 2024](https://img.shields.io/badge/rust-2024-orange)](https://doc.rust-lang.org/edition-guide/editions/2024.html)
[![Status: WIP](https://img.shields.io/badge/status-WIP-yellow)](#status)
[![CLI](https://img.shields.io/badge/type-CLI-informational)](#quick-start)
[![Lint: clippy](https://img.shields.io/badge/lint-clippy-green?logo=rust)](https://github.com/rust-lang/rust-clippy)
[![Style: rustfmt](https://img.shields.io/badge/style-rustfmt-blue?logo=rust)](https://github.com/rust-lang/rustfmt)

A tiny, learning‑oriented reimplementation of grep in Rust. It parses CLI args, reads a file, and prints only the lines that contain the query. Search is case‑sensitive by default and can be toggled to case‑insensitive via the `IGNORE_CASE` environment variable.

## Quick Start

- Prerequisite: Rust and Cargo installed (stable toolchain).
- Build: `cargo build`
- Run: `cargo run -- <query> <file>` (e.g., `cargo run -- to poem.txt`)
- Check types: `cargo check`
- Test: `cargo test`
- Format/Lint: `cargo fmt --all` and `cargo clippy --all-targets --all-features`

## Installation

- From source (local install): `cargo install --path .`
- Build release binary: `cargo build --release` (binary at `target/release/minigrep`)
- Run from workspace: `cargo run -- <query> <file>`

## Usage

Example run against a file `poem.txt`:

Default (case‑sensitive):

```
cargo run -- to poem.txt

Input query: to
Path to file: poem.txt
to sleep, perchance to dream
not TO BE, but to become
```

Case-insensitive via env var:

```
IGNORE_CASE=1 cargo run -- to poem.txt

Input query: to
Path to file: poem.txt
to sleep, perchance to dream
TO SEE OR NOT TO SEE
not TO BE, but to become
```

Show the built-in help/usage text (exits with code 0):

```
cargo run -- --help

Usage: minigrep [OPTIONS] <query> <file_path>

OPTIONS:
    -i, --ignore-case       Search without case sensitivity or when IGNORE_CASE is set.
    -I, --no-ignore-case    Force case-sensitive search even if IGNORE_CASE is set.
    -n, --line-number       Prefix matches with their line number.
```

`minigrep --help` uses the same path and prints to stdout, skipping the error handler in [`src/main.rs`](src/main.rs#L7) so scripts can pipe diagnostics separately from help text.

### Error Handling

Matches are written to stdout while diagnostics are emitted on stderr via the `eprintln!` calls in [`src/main.rs`](src/main.rs). This keeps search results clean and lets you redirect messages separately:

```
cargo run -- to 2> errors.log
cat errors.log

Problem parsing input arguments: Not enough input arguments provided.
```

Missing files behave the same way; the `Application errored: ...` message lands on stderr, so piping stdout still yields only matched lines.

## Exit Codes

- 0: Successful execution (includes `--help`).
- 1: CLI parsing error or runtime error (e.g., cannot read file).

Errors print a short, user-friendly message to stderr and exit non‑zero.

## Project Layout

- [`src/main.rs`](src/main.rs): Minimal binary entrypoint; parses args and delegates to library.
- [`src/lib.rs`](src/lib.rs): Library crate exposing `Config` and `run()` used by `main` and tests.
- `tests/`: Integration tests using the public API.
- `tests/fixtures/`: Test input files.
- `target/`: Build artifacts (ignored by Git).

## Architecture

- [`main.rs`](src/main.rs): Keeps I/O and argument handling thin; prints a short header and calls `minigrep::run(config)`. On errors, it prints a friendly message and exits non‑zero. This follows the guidance in [`AGENTS.md`](AGENTS.md) to keep `main` minimal.
- [`lib.rs`](src/lib.rs): Owns core types and logic:
  - `pub struct Config { query, file_path, ignore_case }`
  - `impl Config { pub fn build(args: &[String]) -> Result<Config, &'static str> }` (reads `IGNORE_CASE` env var)
  - `pub fn run(config: Config) -> Result<(), Box<dyn Error>>`
  - `pub fn search(query: &str, contents: &str) -> Vec<&str>` and `pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<&str>`
  `run` reads the file and prints each matching line using the appropriate search function based on `ignore_case`. This separation enables unit tests on `lib` and keeps the CLI thin.

## Rust Project Structure (Brief)

- [`Cargo.toml`](Cargo.toml): Package metadata, edition, dependencies, and binary targets; source of truth for Cargo.
- [`Cargo.lock`](Cargo.lock): Exact, resolved dependency versions for reproducible builds (commit for apps; regenerate for libraries).
- [`src/main.rs`](src/main.rs): Binary crate entry point with `fn main()` that delegates to the library.
- [`src/lib.rs`](src/lib.rs): Library crate for reusable logic and a public API used by both `main` and tests.
- `tests/`: Integration tests compiled as separate crates that exercise the public API.
- `tests/fixtures/`: Sample input files used by tests.
- `target/`: Build outputs (`debug/`, `release/`, incremental); ignored by Git.
- [`README.md`](README.md): Project overview, usage, and layout.
- [`CHANGELOG.md`](CHANGELOG.md): Human-readable history with commit/diff references explaining what changed and why.
- [`CONTRIBUTING.md`](CONTRIBUTING.md): Contribution, style, and tooling guidelines.
- [`.gitignore`](.gitignore): Ignore patterns (e.g., `target/`).
- [`AGENTS.md`](AGENTS.md): Notes on using codex-cli in this repo.
- Optional (not present here): `examples/` (runnable samples), `benches/` (benchmarks), `build.rs` (build script), `.cargo/config.toml` (tooling/workspace config), CI files.

## Build Artifacts and `target/`

- `target/debug/`: Development profile artifacts (fast, incremental, debug info). Contains the main binary (e.g., `minigrep`) plus subdirs.
- `target/release/`: Optimized artifacts built with `--release` (slower to build, faster to run, smaller binaries).
- `target/<profile>/deps/`: Per-crate compiled outputs with hashed names. Includes libraries and intermediates used to link your binary and tests.
- `target/<profile>/incremental/`: Incremental compilation caches speeding up subsequent builds.
- `target/<profile>/build/`: Outputs from `build.rs` scripts (if any), often with an `out/` directory for generated code or headers.
- `target/<profile>/.fingerprint/`: Cargo/rustc metadata used to decide what needs rebuilding.
- `target/doc/`: Generated documentation from `cargo doc`.
- `target/package/`: Crate tarballs produced by `cargo package`/`cargo publish`.
- `target/debug/examples/` and `target/debug/deps/`: Binaries for examples and tests when built.

Common artifact file types you might see (platform/profile dependent):
- `.o`: Object files produced by the compiler or the `cc` crate (C/C++ build steps); linked into libraries or final binaries.
- `.d`: Dependency files (Makefile-style) typically emitted by `cc`/`clang` builds to track header dependencies.
- `.rlib`: Rust static library archives used when one Rust crate links another.
- `.rmeta`: Rust crate metadata (no code) used for faster checks and linking metadata.
- `.a`: Static libraries from C/C++ or crates built as `staticlib`.
- `.so`/`.dylib`/`.dll`: Shared libraries (Linux/macOS/Windows) for `cdylib` crates or native deps.
- Executables: The final program (`minigrep` on Unix, `minigrep.exe` on Windows) in `target/<profile>/`.
- Debug symbols (platform-specific): `.pdb` (Windows), `.dSYM/` bundle (macOS), or DWARF sections embedded on Unix.

Note: Exact layout and presence of files vary by OS, Rust/Cargo versions, enabled features, and whether native code is compiled via build scripts.

## From Build to Run (Lifecycle)

- Resolve: `cargo build` reads `Cargo.toml` and locks versions via `Cargo.lock`, resolving and preparing dependencies.
- Compile: Each crate compiles to intermediates (`.o`, `.rlib`, `.rmeta`) under `target/<profile>/deps/`; incremental caches go under `incremental/`.
- Link: Rustc links your binary crate with its deps to produce the executable in `target/<profile>/`.
- Run: `cargo run -- <args>` builds if needed, then executes the binary (e.g., `target/debug/minigrep`). You can also run the file directly.
- Test/Doc: `cargo test` builds test binaries under `target/<profile>/deps/`; `cargo doc` writes HTML docs to `target/doc/`.
- Clean: `cargo clean` removes `target/` to reclaim space or force full rebuilds.

## Notes

- Follows Rust 2024 edition conventions; 4‑space indentation.
- Prefer `Result<T, E>` and `?` to bubble errors; avoid panics on user I/O.
- Validate CLI args and handle missing/permission errors gracefully.
- Case‑insensitive search is enabled by setting `IGNORE_CASE` in the environment.

## Roadmap

- Add line numbers and file name in output.
- Add integration tests under `tests/` with fixtures.
- Support regex or whole‑word matches as optional modes.
- Add CLI flags to override env var (e.g., `--ignore-case/--case-sensitive`).

## Contributing

See `CONTRIBUTING.md` for commit/PR conventions, testing guidance, and tooling. Small improvements and learning notes are welcome.

## Changelog

See [`CHANGELOG.md`](CHANGELOG.md) for a history of notable documentation and code updates with commit references.

## Status

This is a learning project and is actively evolving. Basic grep behavior is implemented (search with optional case‑insensitivity via `IGNORE_CASE`).

— README created/updated by codex-cli
