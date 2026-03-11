# aihelp AGENTS

## What This Repo Does

`aihelp` is a Rust CLI for LM Studio's OpenAI-compatible API with optional MCP tool discovery and tool calling. It stores user config in `~/.config/aihelp/config.toml`.

## Main Entrypoints

- `src/main.rs`: CLI parsing and dispatch.
- `src/client.rs`: LM Studio HTTP client.
- `src/mcp.rs`: MCP discovery and invocation.
- `src/config.rs`: config loading and persistence.
- `tests/`: integration tests.
- `man/aihelp.1`: manpage source.

## Commands

- `cargo build`
- `cargo run -- --setup`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

## Repo-Specific Constraints

- Keep the config format as TOML.
- Keep TLS on `rustls`; do not introduce OpenSSL-only paths.
- New CLI flags should go through `clap` derive in `main.rs`.
- Do not weaken MCP defaults by making `all` the implicit policy.
- Preserve the optimized release profile in `Cargo.toml`.

## Agent Notes

- Read the existing CLI behavior before changing user-facing flags.
- Prefer narrow Rust changes and validate with the full Cargo gate for touched code.
