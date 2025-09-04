# Changelog

All notable changes to this project are documented here. This is a learning project; entries include brief reasoning and links to commits when available.

## 2025-09-04
- Docs: Add "Rust Project Structure (Brief)" to README covering `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/lib.rs`, `tests/`, `tests/fixtures/`, `target/`, and repo docs (`README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `AGENTS.md`). Rationale: provide a concise Rust-specific orientation to the repository layout and standard project conventions. Diff: see README.md changes in this change set (commit to be referenced upon merge) and run `git diff -- README.md` locally to review.
- Docs: Expand README with detailed `target/` subdirectories and common artifact filetypes (`.o`, `.d`, `.rlib`, `.rmeta`, `.a`, `.so/.dylib/.dll`, executables, debug symbols) plus a brief build→link→run lifecycle. Rationale: clarify what Cargo/rustc generate from compilation to execution and where to find outputs. Diff: compare README before/after this entry.
- Docs: Add explicit `src/main.rs` history summary below to satisfy project guideline of documenting version history of the entrypoint.

- Docs: Enrich README with new sections: Installation (cargo install and release build paths), Exit Codes (0 on success; 1 on CLI/runtime error), and Roadmap (search, case-insensitive mode, move logic to lib, tests). Rationale: clarify usage, behavior, and planned direction. Diff: see README changes in this commit; `git diff -- README.md`.

- Code/Structure: Introduce `src/lib.rs` exposing `Config` and `run()`, and refactor `src/main.rs` to delegate to the library, keeping `main` minimal per `AGENTS.md` (commit `363d3a4`). Rationale: enable testing of core logic and improve separation of concerns. See diff for `src/main.rs` and new `src/lib.rs` in this change.

### src/main.rs history (recent)
- 363d3a4: Move `run` and `Config` into `src/lib.rs`; `main` now imports from the library and delegates. Rationale: keep `main` minimal per AGENTS.md and enable unit testing via the library crate.
- 0676f4a: Introduce `run(config) -> Result` and switch file I/O to use `Result` with `?`, removing `expect`; add `std::error::Error` and propagate errors to `main` with graceful exit. Rationale: avoid panics on user-controlled I/O and align with guideline to prefer `Result<T, E>`.
- 96fe626: Harden CLI parsing: replace `Config::new` returning a value with `Config::build` returning `Result<_, &'static str>`; use `unwrap_or_else` in `main` to print a helpful message and exit non-zero. Also gate `dbg!(&args)` behind a comment. Rationale: clearer UX and safer argument handling.
- 831cb37: Replace free function `parse_config(&[String]) -> (String, String)` with a `Config` struct and `new` constructor; adjust call sites to use `config.query` and `config.file_path`. Rationale: prepare for growth and encapsulate CLI parameters.
- d6e795c: Initial `parse_config` introduced to parse CLI args; `main` printed query/path and read file directly with `fs::read_to_string`.

## 2025-09-03
- Docs: Expand README with accurate current behavior, usage example, contributing and changelog pointers; add badges. Rationale: keep docs truthful to current binary (prints file contents, search WIP). See commit `570869f` for the initial README addition.
- Docs: Add CONTRIBUTING.md summarizing coding, testing, and PR conventions.
- Docs: Add CHANGELOG.md and adopt simple date-based entries.

## Earlier notable commits
- 570869f — Add README with usage and badges [codex-cli]
- 96fe626 — Improve error handling around input argument parsing
- a515cec — Add AGENTS.md for codex-cli usage
- 831cb37 — Introduce `Config` constructor replacing `parse_config`
- d6e795c — Implement `parse_config` for CLI argument parsing

— CHANGELOG created/updated by codex-cli
