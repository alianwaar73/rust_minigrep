# Repository Guidelines

## Project Structure & Module Organization
- Root binary crate `minigrep` with entrypoint at `src/main.rs`.
- Add shared logic to `src/lib.rs` if code grows beyond `main`.
- Place integration tests under `tests/` and fixtures under `tests/fixtures/`.
- Build artifacts are created in `target/` (ignored by Git).

## Build, Test, and Development Commands
- `cargo build`:
  Compile the project in debug mode.
- `cargo run -- <query> <file>`:
  Run locally, e.g. `cargo run -- to poem.txt`.
- `cargo check`:
  Type-check quickly without producing binaries.
- `cargo test`:
  Run unit and integration tests.
- `cargo fmt --all` and `cargo clippy --all-targets --all-features`:
  Format and lint; fix warnings before pushing.

## Coding Style & Naming Conventions
- Use Rust 2024 edition defaults; 4-space indentation.
- Naming: `snake_case` for functions/files, `CamelCase` for types/traits, `UPPER_SNAKE_CASE` for constants.
- Keep `main` minimal; move logic into `lib` functions.
- Prefer `Result<T, E>` over panics; bubble errors with `?`.

## Testing Guidelines
- Unit tests live next to code in `src/*.rs` behind `#[cfg(test)]`.
- Integration tests go in `tests/` using the public API.
- Name tests for behavior, e.g. `search_finds_substring()`.
- Aim for meaningful coverage of parsing, I/O, and search paths.
- Run `cargo test -- --nocapture` to see printed output when debugging.

## Commit & Pull Request Guidelines
- Commits: short, imperative subject (≤72 chars), e.g. `Add search API and tests`.
- Include focused changes per commit; explain reasoning in the body when nontrivial.
- PRs: clear description, reproduction steps, expected vs. actual output, and any CLI examples.
- Link related issues; add screenshots or sample input/output when applicable.

## Security & Configuration Tips
- Avoid unchecked `expect` on user-controlled I/O; return errors.
- Validate CLI args length before indexing; show helpful usage on failure.
- Treat file paths as untrusted; handle missing/permission errors gracefully.
