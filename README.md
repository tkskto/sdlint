# sdlint

sdlint is a deterministic command-line linter for schema.org structured data. It reads JSON-LD files and JSON-LD script elements embedded in HTML, reports structural and rule diagnostics, and provides stable exit codes for local development and CI.

The current rule set is an initial implementation. It checks baseline JSON-LD structure and the presence of documented Article properties; see [Rule sources](docs/rule-sources.md) for the source and compatibility policy.

## Installation

sdlint requires Rust 1.85 or later. Build and install the current checkout with Cargo:

```console
git clone https://github.com/tkskto/sdlint.git
cd sdlint
cargo install --path . --locked
```

For development, run the binary without installing it:

```console
cargo run -- example.html
```

## Usage

Pass one or more files, directories, quoted globs, or a single hyphen for standard input:

```console
sdlint page.html
sdlint structured-data.json
sdlint pages/
sdlint 'pages/**/*.html'
printf '%s\n' '{"@context":"https://schema.org","@type":"Organization"}' | sdlint -
```

Supported file extensions are .html, .htm, .json, .jsonld, and .json-ld. Directories are searched recursively without following symbolic-link directories. HTML inputs are inspected for script elements whose type is application/ld+json. Standard input is interpreted as JSON-LD.

Common options:

| Option | Purpose |
| --- | --- |
| --format text or --format json | Select the report format. |
| --severity LEVEL | Display diagnostics at or above info, warning, or error. |
| --fail-on LEVEL | Fail on error, warning, info, or none. |
| --no-color | Disable colored text output. |

Run `sdlint --help` for the complete command-line interface.

## Configuration

Configuration is optional. Starting at the working directory, sdlint searches parent directories for the nearest sdlint.toml and applies that one configuration to every input in the run.

```toml
[files]
ignore = ["dist/**", "vendor/**", "fixtures/invalid/**"]
respect_gitignore = true

[rules]
"core/jsonld-context-required" = "error"
"google/article/headline-recommended" = "off"
"google/article/image-recommended" = "warning"

[exit]
fail_on = "error"

[[overrides]]
files = ["fixtures/**/*.html"]
rules = { "core/jsonld-context-required" = "off" }
```

When a section or setting is omitted, sdlint uses these defaults:

| Setting | Default | Meaning |
| --- | --- | --- |
| files.ignore | Empty list | No configured ignore patterns are applied. |
| files.respect_gitignore | false | The .gitignore file is not loaded. |
| rules | Empty table | Each rule keeps its built-in severity. |
| overrides | None | No path-specific rule settings are applied. |
| exit.fail_on | error | A run fails when it produces an error diagnostic. |

Unknown sections, keys, Rule IDs, and setting values are execution errors. Configuration errors produce exit code 2 instead of being ignored.

### File exclusion

When expanding a directory or glob, sdlint skips any node_modules directory by default. A supported file inside node_modules is still linted when its path is passed explicitly.

The files.ignore array accepts Git-compatible ignore patterns relative to the working directory. Negation patterns can re-include a path:

```toml
[files]
ignore = ["generated/**", "!generated/keep.json"]
```

Set respect_gitignore to true to load .gitignore from the directory containing sdlint.toml:

```toml
[files]
respect_gitignore = true
```

Configured files.ignore and .gitignore exclusions apply equally to directly named files and files discovered through a directory or glob. Matching files are skipped silently. Configured files.ignore patterns take precedence over .gitignore patterns.

### Rule settings

The rules table accepts a Rule ID as its key and one of error, warning, info, or off as its value.

```toml
[rules]
"google/article/headline-recommended" = "error"
"google/article/image-recommended" = "off"
```

Setting a rule to off prevents that rule from running. The other values override the severity used when the rule produces a diagnostic. An unknown Rule ID is an error because it is likely a typo.

### Path overrides

Use overrides to apply rule settings only to selected files. File patterns use slash-separated paths relative to the working directory and do not distinguish uppercase and lowercase letters.

```toml
[[overrides]]
files = ["fixtures/**/*.html"]
rules = { "core/jsonld-context-required" = "off" }

[[overrides]]
files = ["legacy/**/*.html"]
rules = { "google/article/headline-recommended" = "info", "google/article/image-recommended" = "off" }
```

Standard input has no path and does not match overrides. If more than one overrides entry matches the same input, sdlint reports a configuration error instead of choosing one implicitly.

### Exit policy

The exit.fail_on setting controls the lowest diagnostic severity that produces exit code 1:

```toml
[exit]
fail_on = "warning"
```

The accepted values are error, warning, info, and none. A command-line --fail-on value takes precedence over sdlint.toml; otherwise the built-in default is error.

Cache and max-warnings settings are not yet implemented and are rejected rather than accepted without effect.

## Exit codes

| Code | Meaning |
| ---: | --- |
| 0 | The run completed without a diagnostic at or above the failure threshold. |
| 1 | At least one diagnostic met the failure threshold. |
| 2 | An input, parsing, configuration, command-line, output, or internal execution error occurred. |

Exit code 2 takes precedence over exit code 1.

## Development

See [Development getting started](CONTRIBUTING.md) for local setup and validation. The observable command contract is defined in the [CLI specification](docs/spec.md), and planned post-MVP functionality is described in [Practical linter features](docs/practical-linter-features.md).
