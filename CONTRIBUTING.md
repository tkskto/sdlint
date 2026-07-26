# Development getting started

This guide explains how to set up, build, and validate `sdlint` locally.

## Prerequisites

Install the stable Rust toolchain with [rustup](https://rustup.rs/). The project
uses Cargo for dependency management, builds, and tests.

Confirm that the required tools are available:

```console
rustc --version
cargo --version
```

## Set up the repository

Clone the repository and enter its working directory:

```console
git clone <repository-url>
cd sdlint
```

Build the CLI and library:

```console
cargo build
```

Cargo downloads the dependencies recorded in `Cargo.lock` and writes build
artifacts to `target/`.

## Run the CLI

Pass one or more JSON-LD or HTML inputs after `--`:

```console
cargo run -- example.json
cargo run -- 'examples/**/*.jsonld'
```

Use `-` to read JSON-LD from standard input. With no input operand, standard
input is selected automatically:

```console
printf '%s\n' '{"@context":"https://schema.org"}' | cargo run -- -
printf '%s\n' '{"@context":"https://schema.org"}' | cargo run
```

Display the complete command-line interface with:

```console
cargo run -- --help
```

## Validate a change

Before committing, run the same formatting, test, and lint checks expected for
the project:

```console
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

To apply Rust's standard formatting instead of only checking it, run
`cargo fmt`.

Unit tests for deterministic input expansion and readers live alongside the
implementation in `src/input/mod.rs`. End-to-end command tests live in
`tests/cli_input.rs`. Add or update tests in the corresponding location when
behavior changes.

## Project layout

| Path | Purpose |
| --- | --- |
| `src/cli.rs` | Command-line arguments and accepted option values. |
| `src/input/` | Input expansion, ordering, deduplication, and readers. |
| `src/app.rs` | Library-level application orchestration. |
| `src/main.rs` | Binary boundary and exit-code conversion. |
| `tests/` | CLI integration tests. |
| `docs/spec.md` | Normative observable CLI behavior. |
| `docs/rule-sources.md` | Rule sources and Rule ID compatibility policy. |

Library code must not terminate the process. Return a typed result or
`RunOutcome` and perform exit-code conversion only in `src/main.rs`. Preserve
the deterministic input and diagnostic ordering defined by `docs/spec.md`.
