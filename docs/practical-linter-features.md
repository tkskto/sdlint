# Practical Linter Features for sdlint

## Purpose

In addition to structured data validation, sdlint should support gradual adoption in existing projects, efficient repeated runs in CI, and auditable exceptions. These concerns must not be implemented directly in the CLI. Input selection, diagnostic filtering, and caching remain separate layers.

```text
CLI / configuration
    -> Input selection (glob, exclude, ignore files)
    -> Extract / parse / normalize
    -> Deterministic rule engine
    -> Suppression and baseline filter
    -> Reporter
    -> Exit policy
```

The Rule Engine produces unsuppressed diagnostics. Suppression, presentation, and exit-code evaluation happen later in the pipeline. Individual rules therefore do not need to know about CLI options, ignore files, or output formats.

## Priorities

| Priority | Feature | Target |
| --- | --- | --- |
| P0 | File exclusion, Rule ID enable/disable, and severity overrides | MVP |
| P0 | Explicit opt-outs such as `--no-ignore` and `--no-cache` | Alongside the corresponding feature |
| P1 | Content-hash cache for deterministic validation | First post-MVP improvement |
| P1 | `--max-warnings`, `--quiet`, and `--output-file` | Post-MVP |
| P1 | Baselines for gradual adoption with existing violations | Post-MVP |
| P2 | Local suppression comments in HTML | After validating real use cases |
| P2 | `--list-rules` and `--explain RULE_ID` | After the rule set grows |
| P2 | Parallel processing | Only after measurement shows a need |

## Configuration File

Configuration remains optional so that sdlint can be used with command-line arguments alone. Searching upward from every input file for `sdlint.toml` would be ambiguous when validating multiple inputs. Instead, sdlint searches upward once from the working directory and applies the same configuration to every input.

The precedence order is CLI options, then the sdlint.toml file, then built-in defaults. The specification must define whether each array-valued CLI option replaces or extends the configured value; it must not rely on implicit merging.

```toml
[files]
ignore = ["dist/**", "vendor/**", "fixtures/invalid/**"]
respect_gitignore = true

[rules]
"SDL001" = "error"
"ARTICLE002" = "off"
"FAQ*" = "warning"

[cache]
enabled = false
directory = ".sdlintcache"

[exit]
fail_on = "error"
max_warnings = 20

[[overrides]]
files = ["fixtures/**/*.html"]
rules = { "SDL001" = "off" }
```

An exact Rule ID takes precedence over a wildcard. Ambiguous settings at the same precedence level are configuration errors rather than "last one wins." Unknown exact Rule IDs are execution errors so that typos cannot silently disable validation. A wildcard that currently matches no rules may remain valid to support shared configuration across rule-set versions.

## Ignoring Files and Suppressing Diagnostics

Excluding a file from validation and suppressing a particular diagnostic are distinct operations.

### File Exclusion

Support these ignore sources:

- `files.ignore` in `sdlint.toml`
- `.sdlintignore`
- `--ignore-pattern <GLOB>`, repeatable on the command line
- `--no-ignore`
- Optional `.gitignore` support through `respect_gitignore`

The behavior is:

1. Explicitly named files are subject to ignore rules by default.
2. If an explicitly named file is ignored, report that fact to stderr at the informational level rather than silently succeeding.
3. `--no-ignore` disables `.gitignore`, `.sdlintignore`, configured ignore rules, and command-line ignore patterns.
4. Normalize matching paths to `/`-separated paths relative to the working directory.
5. Document symlink traversal and case-sensitivity rules, and avoid validating the same underlying file more than once.

Prefer the Rust `ignore` crate for Git-compatible negation patterns and directory semantics. Do not grow a custom ignore-pattern implementation.

### Rule Suppression

Use `[rules]` for project-wide suppression and `[[overrides]]` for path-specific suppression. Provide command-line options for temporary investigation:

```sh
sdlint page.html --disable ARTICLE002 --disable FAQ003
sdlint page.html --enable-only SDL001,BREADCRUMB001
```

In addition to the post-suppression diagnostic counts, the JSON summary includes `suppressedCount`. Suppressed diagnostics are omitted by default, with a possible `--show-suppressed` option for auditing.

Local suppression comments in HTML are useful, but JSON files cannot contain comments, which creates an asymmetry between input formats. They can also turn temporary exceptions into permanent markup. Do not implement local comments in the MVP. If added later, require a reason:

```html
<!-- sdlint-disable-next-block ARTICLE002 -- reason: legacy CMS omits image -->
<script type="application/ld+json">...</script>
```

The implementation should be able to diagnose unused suppressions, unknown Rule IDs, and suppressions without a reason.

## Baselines

A baseline is preferable to disabling rules across a project that already has many violations.

```sh
sdlint "pages/**/*.html" --write-baseline .sdlint-baseline.json
sdlint "pages/**/*.html" --baseline .sdlint-baseline.json
```

Do not identify baseline entries using only absolute paths or line numbers. Build an identity from:

```text
relative source path
+ JSON-LD block identity
+ Rule ID
+ JSON Path
+ normalized offending-value hash
```

Line numbers shift under routine HTML edits. Diagnostic messages also change as wording improves. A matching baseline entry suppresses that existing diagnostic, while new diagnostics still affect the exit code. `--report-stale-baseline` reports entries whose diagnostics no longer exist so users can remove them.

A baseline must not become a permanent exception dump. Its format records the creation time, sdlint version, rule-set version, and optional reason, expiry, and owner for each entry.

## Cache

### Scope and Default

Cache the deterministic parse, normalization, and rule-evaluation result for each input. Do not put execution errors, formatted reporter output, or future Semantic Checker results in the same cache.

Initially make caching opt-in with `--cache`. Reconsider enabling it by default only after invalidation behavior and the cache format have stabilized.

```sh
sdlint "pages/**/*.html" --cache
sdlint "pages/**/*.html" --cache --cache-dir .cache/sdlint
sdlint "pages/**/*.html" --no-cache
```

### Cache Key

Modification time and file size alone can return stale results after checkouts that preserve timestamps or edits that preserve size. Prefer correctness and use a content hash.

```text
cache format version
+ sdlint version
+ parser/normalizer version
+ rule-set IDs and versions
+ effective rule configuration hash
+ normalized relative source identity
+ input kind
+ input content hash
```

Output format, color, `--quiet`, and `--max-warnings` do not change diagnostics and are excluded from the key. Severity overrides, rule enable/disable settings, selected types, and every option that changes rule behavior are included.

Standard input has no stable file identity and is not cached by default. A future version may allow caching when `--stdin-filename` is supplied, using that name together with the content hash.

### Storage and Failure Behavior

- Store a versioned index and per-input entries under `.sdlintcache/`.
- Write to a temporary file and rename it so interruption cannot leave a partial entry.
- Use locking or conflict-safe per-entry writes for concurrent processes.
- Treat corrupt entries, unknown versions, and cache read failures as misses and continue linting.
- By default, cache failures do not produce exit code 2; report a warning in verbose output.
- Provide `sdlint --clear-cache` for safe removal.
- Do not commit the cache directory to version control.

Caching does not skip input discovery. Resolve the input set on every run so changes to ignore rules and globs take effect, then query the cache for each selected input.

### Separation from the Semantic Checker

AI results use a separate cache whose key includes the provider, model, prompt version, data-sharing policy, inference parameters, and input/evidence hashes. Do not negative-cache timeouts or provider failures as successful results. A deterministic lint cache hit must not depend on whether the Semantic Checker can run.

## CI-Oriented Features

These options are practical and can be added to the Reporter and Exit Policy without changing the Rule Engine:

- `--max-warnings N`: return exit code 1 if warnings exceed N
- `--quiet`: display only errors while still evaluating all severities
- `--output-file PATH`: atomically write the report instead of using stdout
- `--format json`: produce stable, versioned JSON
- `--no-color`: support non-TTY output and log collection
- `--stdin-filename PATH`: give stdin a virtual name for format detection, overrides, and diagnostic display

`--quiet` controls presentation, not rule evaluation. Generate warning diagnostics before filtering the display so that the summary and `--max-warnings` retain consistent semantics.

## Rule Discoverability

After the rule set grows, add:

```sh
sdlint --list-rules
sdlint --explain BREADCRUMB002
```

`--explain` displays the Rule ID, severity, target types, deterministic/semantic category, schema.org/Google source category, description, remediation hint, reference URLs, and version in which it was introduced. Generate this output from `RuleMetadata` rather than duplicating reporter-specific text.

## Implementation Boundaries

The main components are conceptually:

```rust
struct EffectiveConfig { /* immutable resolved CLI and file configuration */ }

trait InputFilter {
    fn decision(&self, path: &Path) -> IgnoreDecision;
}

trait DiagnosticFilter {
    fn decision(&self, diagnostic: &Diagnostic) -> SuppressionDecision;
}

trait LintCache {
    fn get(&self, key: &CacheKey) -> Result<Option<CachedLintResult>, CacheError>;
    fn put(&self, key: &CacheKey, value: &CachedLintResult) -> Result<(), CacheError>;
}
```

Do not mechanically create traits when the MVP has only one implementation. Start `InputFilter` and `DiagnosticFilter` as concrete structs and introduce traits when tests need substitution or multiple implementations exist. `LintCache` has a meaningful boundary because the Application should be able to receive no-op and filesystem implementations.

## Tests

### Ignore and Suppression

- Precedence among `.sdlintignore`, `.gitignore`, configuration, and CLI patterns
- Negated patterns, explicit files, and `--no-ignore`
- Exact Rule IDs and wildcard patterns
- Path matching for overrides
- Suppressed diagnostics not affecting the exit code
- Unknown and unused suppressions

### Baseline

- Only matching existing diagnostics are suppressed
- A changed offending value at the same path is a new diagnostic
- Adding lines does not invalidate an otherwise stable entry
- Stale entries are detected
- Rule-set version changes have defined behavior

### Cache

- Identical input and configuration produce a hit
- Content, rules, severity overrides, and rule-set version changes produce a miss
- Output-format changes still produce a hit
- Corrupt entries are ignored and regenerated
- Partial entries are never read
- Multiple processes can use the cache safely
- Stdin is not cached by default
- Cache hits and misses produce identical diagnostic ordering, JSON, and exit codes

## Rejected Alternatives

- Ignoring message strings: wording changes would break configuration; use Rule IDs.
- Line-number-only baselines: ordinary HTML edits make them unstable; use paths and value hashes.
- mtime-only caching: it risks stale results; use content hashes.
- Caching formatted output: it couples validation to format and color; cache the diagnostic model.
- Putting every suppression in HTML comments: it is asymmetric with JSON and difficult to audit.
- Implementing a remote cache initially: its complexity, confidentiality concerns, and compatibility costs outweigh its MVP value.

## References

Use these official resources when comparing established linter behavior for CLI options, ignores, and caches:

- [ESLint command line interface](https://eslint.org/docs/latest/use/command-line-interface)
- [ESLint ignore files](https://eslint.org/docs/latest/use/configure/ignore)
- [Ruff configuration](https://docs.astral.sh/ruff/configuration/)
- [Ruff settings](https://docs.astral.sh/ruff/settings/)

Do not copy another tool's option names without reviewing their semantics. Define behavior that fits sdlint's multiple JSON-LD blocks per HTML file, stable JSON Paths, and separation of deterministic and semantic checks.
