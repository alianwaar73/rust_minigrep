# Changelog

All notable changes to this project are documented here. This is a learning project; entries include brief reasoning and links to commits when available.

## 2025-09-10
- Docs: Update [`README.md`](README.md) to reflect current behavior:
  - Clarify default case‑sensitive search and `IGNORE_CASE` for case‑insensitive mode.
  - Add concrete usage examples for both modes and align sample output with [`src/main.rs`](src/main.rs) printing only matching lines.
  - Update Architecture to include `ignore_case` in `Config` and document `search_case_insensitive` in [`src/lib.rs`](src/lib.rs).
  - Refresh Status and Roadmap to mark case‑insensitive search available via env var and note future CLI flags.
  Rationale: keep docs truthful to implemented features and CLI UX.
- Code: No source changes in this revision; behavior remains as in prior commits.

### src/main.rs history (delta)
- No code changes since [`92652ab`](https://github.com/alianwaar73/rust_minigrep/commit/92652ab). Current `main` collects args, builds `Config` with `unwrap_or_else` error handling, prints the query and file path, and delegates to `minigrep::run(config)`; on error, prints a friendly message and exits non‑zero.

### src/lib.rs history (delta)
- [`3d67322`](https://github.com/alianwaar73/rust_minigrep/commit/3d67322): Add case‑insensitive search via `search_case_insensitive` and plumb `IGNORE_CASE` env var through `Config { ignore_case }` to `run`. Rationale: enable grep‑like `-i` behavior by environment flag for now.
- [`92652ab`](https://github.com/alianwaar73/rust_minigrep/commit/92652ab): Wire `run` to iterate matches from `search(&config.query, &contents)` and print only matching lines. Rationale: switch from dumping full file to grep‑style filtered output.
- [`916788f`](https://github.com/alianwaar73/rust_minigrep/commit/916788f): Introduce `search(query, contents) -> Vec<&str>` and add a unit test for a basic substring match. Rationale: establish a testable core API for search.
- [`363d3a4`](https://github.com/alianwaar73/rust_minigrep/commit/363d3a4): Extract core logic from `main` into new `src/lib.rs` exposing `Config` and `run`. Rationale: keep `main` minimal and enable library‑level testing.
- [`1f3eb7a`](https://github.com/alianwaar73/rust_minigrep/commit/1f3eb7a): Further separation of concerns between `main.rs` and `lib.rs`. Rationale: continue refactor to stabilize public API.

## 2025-09-09
- Code: Finish basic search pipeline. `run` now filters and prints only matching lines via `search()` instead of dumping the entire file (commit [`92652ab`](https://github.com/alianwaar73/rust_minigrep/commit/92652ab)). Rationale: make the binary behave like a minimal grep.
  - Diff focus: `src/lib.rs` changed the `run` loop to iterate `search(&config.query, &content)` and `println!("{line}")` for each match.
- Code: Introduce `search(query, contents) -> Vec<&str>` with a unit test demonstrating a simple substring match (commit [`916788f`](https://github.com/alianwaar73/rust_minigrep/commit/916788f)). Rationale: enable TDD on search behavior and keep logic in the library.
- Docs: Update README usage and overview to reflect case‑sensitive searching and removal of the extra "Containing contents:" header. Rationale: keep docs accurate to current output.

### src/main.rs history (delta)
- [`92652ab`](https://github.com/alianwaar73/rust_minigrep/commit/92652ab): Remove `println!("Containing contents:\n")`; `main` now prints the query and file path, then delegates to `run`, which prints only matching lines. This aligns user‑visible output with the new search behavior.
- [`363d3a4`](https://github.com/alianwaar73/rust_minigrep/commit/363d3a4): Earlier refactor kept `main` minimal, delegating to the library. No CLI surface changes since then besides the removal above.

## 2025-09-04
- Docs: Add "Rust Project Structure (Brief)" to README covering `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/lib.rs`, `tests/`, `tests/fixtures/`, `target/`, and repo docs (`README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `AGENTS.md`). Rationale: provide a concise Rust-specific orientation to the repository layout and standard project conventions. Diff: see README.md changes in this change set (commit to be referenced upon merge) and run `git diff -- README.md` locally to review.
- Docs: Expand README with detailed `target/` subdirectories and common artifact filetypes (`.o`, `.d`, `.rlib`, `.rmeta`, `.a`, `.so/.dylib/.dll`, executables, debug symbols) plus a brief build→link→run lifecycle. Rationale: clarify what Cargo/rustc generate from compilation to execution and where to find outputs. Diff: compare README before/after this entry.
- Docs: Add explicit `src/main.rs` history summary below to satisfy project guideline of documenting version history of the entrypoint.

- Docs: Enrich README with new sections: Installation (cargo install and release build paths), Exit Codes (0 on success; 1 on CLI/runtime error), and Roadmap (search, case-insensitive mode, move logic to lib, tests). Rationale: clarify usage, behavior, and planned direction. Diff: see README changes in this commit; `git diff -- README.md`.

- Code/Structure: Introduce [`src/lib.rs`](src/lib.rs) exposing `Config` and `run()`, and refactor [`src/main.rs`](src/main.rs) to delegate to the library, keeping `main` minimal per `AGENTS.md` (commit [`363d3a4`](https://github.com/alianwaar73/rust_minigrep/commit/363d3a4)). Rationale: enable testing of core logic and improve separation of concerns. See diff for `src/main.rs` and new `src/lib.rs` in this change.

### src/main.rs history (recent)
- [`363d3a4`](https://github.com/alianwaar73/rust_minigrep/commit/363d3a4): Move `run` and `Config` into [`src/lib.rs`](src/lib.rs); `main` now imports from the library and delegates. Rationale: keep `main` minimal per AGENTS.md and enable unit testing via the library crate.
- [`0676f4a`](https://github.com/alianwaar73/rust_minigrep/commit/0676f4a): Introduce `run(config) -> Result` and switch file I/O to use `Result` with `?`, removing `expect`; add `std::error::Error` and propagate errors to `main` with graceful exit. Rationale: avoid panics on user-controlled I/O and align with guideline to prefer `Result<T, E>`.
- [`96fe626`](https://github.com/alianwaar73/rust_minigrep/commit/96fe626): Harden CLI parsing: replace `Config::new` returning a value with `Config::build` returning `Result<_, &'static str>`; use `unwrap_or_else` in `main` to print a helpful message and exit non-zero. Also gate `dbg!(&args)` behind a comment. Rationale: clearer UX and safer argument handling.
- [`831cb37`](https://github.com/alianwaar73/rust_minigrep/commit/831cb37): Replace free function `parse_config(&[String]) -> (String, String)` with a `Config` struct and `new` constructor; adjust call sites to use `config.query` and `config.file_path`. Rationale: prepare for growth and encapsulate CLI parameters.
- [`d6e795c`](https://github.com/alianwaar73/rust_minigrep/commit/d6e795c): Initial `parse_config` introduced to parse CLI args; `main` printed query/path and read file directly with `fs::read_to_string`.

## 2025-09-03
- Docs: Expand README with accurate current behavior, usage example, contributing and changelog pointers; add badges. Rationale: keep docs truthful to current binary (prints file contents, search WIP). See commit `570869f` for the initial README addition.
- Docs: Add CONTRIBUTING.md summarizing coding, testing, and PR conventions.
- Docs: Add CHANGELOG.md and adopt simple date-based entries.

## Earlier notable commits
- [`570869f`](https://github.com/alianwaar73/rust_minigrep/commit/570869f) — Add README with usage and badges [codex-cli]
- [`96fe626`](https://github.com/alianwaar73/rust_minigrep/commit/96fe626) — Improve error handling around input argument parsing
- [`a515cec`](https://github.com/alianwaar73/rust_minigrep/commit/a515cec) — Add AGENTS.md for codex-cli usage
- [`831cb37`](https://github.com/alianwaar73/rust_minigrep/commit/831cb37) — Introduce `Config` constructor replacing `parse_config`
- [`d6e795c`](https://github.com/alianwaar73/rust_minigrep/commit/d6e795c) — Implement `parse_config` for CLI argument parsing

— CHANGELOG created/updated by codex-cli
