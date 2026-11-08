# AGENTS.md — working in rok-dev

`rok-dev` is one member repo of the rok toolchain (see `rok-workspace`,
the folder around it). It holds the `rok-dev-cli` crate: a library that
the toolchain's `rok` (rok-v2) links, and the standalone `rok-dev` binary.
It owns the words `dev`, `run`, `doctor` and `port` (rok-data's workspace
ADR 0034; the spec is rok-data's `docs/specs/rok-dev.md`, the tickets
`.scratch/rok-dev/issues/`).

## Where a change belongs

| Change | Where |
| --- | --- |
| A command's grammar, flags or help | `crates/rok-dev-cli/src/lib.rs` (`command()`); rok-v2 reads its subcommands, never re-declares them |
| What a command does | `crates/rok-dev-cli/src/` |
| A word rok routes here | rok-v2, after the word exists in `command()` |
| A term (Runnable, …) | `GLOSSARY.md` |
| The `[dev]` tables a template ships | rok-templates |

## Rules

1. `command()` is the contract with rok-v2: its subcommands are the words
   `rok` routes here. Adding or renaming one is a rok-v2 change too.
2. `run_from(args)` takes the program name first (`rok` when embedded,
   so help reads `rok dev …`) and returns the exit code: 0 for success
   and help, 1 for a runtime failure, 2 for a usage error.
3. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --
   -D warnings` and `cargo test --workspace` clean; CI runs the tests on
   Linux, macOS and Windows, since process groups and Job Objects differ.
4. A command not built yet prints "not built yet", naming its ticket,
   and exits 1.

## Releasing

Bump `version` in `Cargo.toml`, add a `## [X.Y.Z]` section to
`CHANGELOG.md`, merge, then tag **the merge commit** `vX.Y.Z`.
`release.yml` builds the `rok-dev` binaries, publishes the GitHub release
and `rok-dev-cli` to crates.io (the `CARGO_REGISTRY_TOKEN` secret). Then
rok-v2 bumps its `rok-dev-cli` dependency.
