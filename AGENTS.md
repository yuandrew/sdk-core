# Contributor Guide

This manual provides tips and requirements for working with this repository.

## Repository layout

- `etc/` contains scripts like `regen-depgraph.sh` and monitoring configs.
- `README.md` is the primary contributor guide.
- `core/` – Core implementation used by other SDKs.
- `client/` – Client library for talking to Temporal service.
- `core-api/` – Public API surface.
- `sdk/` – Prototype Rust SDK used mainly for testing.
- `fsm/` – State machine macro and related crates.
- `test-utils/` – Helpers and binaries used by tests (for example, the `histfetch` tool).
- `sdk-core-protos/` – Generated protobuf code and extensions.
- `tests/` – Integration and heavy tests.
- `arch_docs/` and `ARCHITECTURE.md` – Architecture notes and design docs.
- Scripts such as `cargo-tokio-console.sh` and `integ-with-otel.sh` provide debugging or testing helpers.

The main contributor guide is this repository’s `README.md`.

## Building and testing

Run the following before submitting a PR:

1. **Build and unit tests**
   ```bash
   cargo build
   cargo test
   ```
2. **Integration tests** – automatically starts an ephemeral server unless `-s external` is passed.
   ```bash
   cargo integ-test
   ```
   Heavy load tests run with:
   ```bash
   cargo test --test heavy_tests
   ```
3. **Formatting and linting**
   ```bash
   cargo fmt --all
   cargo lint            # alias for clippy with warnings as errors
   cargo test-lint       # clippy across all crates and tests
   ```
4. **Documentation build** (ensures docs compile without warnings)
   ```bash
   cargo doc --workspace --all-features --no-deps
   ```

Additional utilities:

- `integ-with-otel.sh` – Runs integration tests with OpenTelemetry collection enabled.
- `cargo-tokio-console.sh` – Helper to run cargo commands with Tokio console support.
- `.cargo/multi-worker-manual-test` – Launches multiple workers in parallel during manual tests.
- `cargo run --bin histfetch <workflow_id> [run_id]` – Fetches workflow histories for replay.

## Formatting and linting

- Format code with `cargo fmt --all`.
- Lint using clippy: `cargo clippy --all -- -D warnings`.
- Both format and lint checks are executed in CI and must pass.
- Comments should be complete sentences and end with a period.
- Public errors must use `thiserror`. Test-only errors may use `anyhow`.
- Do not commit `Cargo.lock` (it is ignored by `.gitignore`).
- Keep code formatted with `cargo fmt --all`.

## Documentation

Generate API docs with `cargo doc --workspace --all-features --no-deps`.

## Repo utilities

- `cargo integ-test` runs integration tests.
- `integ-with-otel.sh` runs integration tests with OpenTelemetry tracing enabled.
- `cargo-tokio-console.sh` runs `cargo` with `tokio-console` feature.
- `etc/regen-depgraph.sh` regenerates `etc/deps.svg` for the dependency graph.

## Style notes

- Errors returned from public interfaces must be typed using `thiserror`.
- Tests may use `anyhow` for convenience.
- For debugging during tests, call `crate::telemetry::telemetry_init_fallback()`.

## Pull request expectations

1. Ensure builds, tests, formatting and linting succeed locally.
2. Keep commit messages concise; use the imperative mood.
3. The PR description should explain what changed and why.
4. Avoid bundling unrelated changes.

Reviewers check that:

- CI passes (format, lint, unit tests, integration tests).
- Code follows the style and error handling guidelines.
- Commits are focused and messages are clear.

The main contributor guide is `README.md` at the repository root.

## Summary

Before submitting a PR:

- Run all tests and static checks.
- Format, lint, type check.
- Write clear commit messages.
- Include tests, docstrings, and release notes.
- Keep PR focused and backward-compatible.
  Refer to `CONTRIBUTING.md` for deeper details if unsure.
