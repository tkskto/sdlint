# sdlint CLI specification

This document defines the observable command-line contract of sdlint. The key words MUST, MUST NOT, SHOULD, and MAY are normative.

## 1. Input

The command accepts zero or more input operands:

```text
sdlint [options] [--] [FILE | DIRECTORY | GLOB | -]...
```

* A regular file is read as UTF-8. A UTF-8 BOM MAY be present and is ignored.
* A file whose extension is “.html” or “.htm” is parsed as HTML and every script element whose type is “application/ld+json” is inspected.
* A file whose extension is “.json”, “.jsonld”, or “.json-ld” is parsed as one JSON-LD document. The top level MAY be an object or an array.
* A directory is searched recursively for the extensions above. Symbolic-link directories are not followed.
* A single hyphen means standard input. It may occur at most once. Standard input is parsed as JSON-LD by default; the stdin-format option with the value “html” selects HTML. When there are no operands, sdlint reads standard input.
* An unsupported explicitly named file is an execution error. Unsupported files found while expanding a directory or glob are ignored.

JSON-LD may use either an absolute schema.org context or the commonly used [https://schema.org](https://schema.org) context. Remote contexts are not fetched: linting MUST be deterministic and MUST NOT require network access.

## 2. Glob operands

A glob is an operand containing an asterisk, question mark, or opening square bracket and is expanded by sdlint when the shell has not already expanded it. A single asterisk and a question mark do not cross a path separator; a double asterisk matches zero or more directories. Matching uses a forward slash as the logical separator, including on Windows. Hidden path components are matched only when the corresponding pattern component begins with a period.

Expansion is relative to the current working directory. Results are normalized and sorted by Unicode code-point order before duplicate paths are removed. A file selected by multiple operands is linted once, at the position of its first operand. A glob that matches no supported file is an execution error. Quote a glob to ensure these rules, rather than the invoking shell's rules, apply.

## 3. Diagnostics and execution errors

A diagnostic reports that successfully acquired input does not satisfy a lint rule—for example, a missing recommended property or an invalid property value. Diagnostics have a source, location when available, Rule ID, severity, and message. Finding diagnostics does not stop other inputs from being checked.

An execution error means the requested lint run could not be carried out reliably. Examples include an unreadable file, malformed HTML/JSON/JSON-LD, invalid CLI option, unsupported explicit input, unmatched glob, duplicate standard-input operands, or failure to write output. Execution errors have no Rule ID and MUST NOT be reported as lint diagnostics. Processing SHOULD continue after a per-input execution error when doing so is safe; command-usage and output-write errors are fatal.

This boundary is intentional: malformed JSON is an execution error because no JSON-LD graph exists to validate, while a well-formed graph with a malformed schema.org value is a diagnostic.

## 4. Severity

Rules have one of these stable severities:

| Severity | Meaning | Default exit effect |
| --- | --- | --- |
| error | A required constraint is violated, so the intended schema or search feature is invalid or materially unreliable. | fails |
| warning | A recommended constraint is violated, compatibility is uncertain, or a feature may be degraded. | does not fail |
| info | Non-blocking advice or an observation that requires human review. | does not fail |

Severity describes the rule result, not whether the tool ran. Execution errors therefore do not have a severity. The fail-on option accepts error, warning, info, or none and changes the lowest diagnostic severity that fails a run; the default is error.

## 5. Exit codes

| Code | Meaning |
| ---: | --- |
| 0 | The run completed and no diagnostic at or above the configured failure threshold was found. |
| 1 | The run completed and at least one diagnostic met the failure threshold. |
| 2 | One or more execution errors occurred. |

Code 2 takes precedence over code 1, even if diagnostics were also emitted. No other public exit code is defined; unexpected internal failures also return 2 and include a concise execution-error message.

## 6. Output order

Human-readable and machine-readable output MUST use the same deterministic order:

1. inputs in operand order; within a directory or glob, normalized path in Unicode code-point order;
2. documents or JSON-LD blocks in source order;
3. diagnostics by start location (line, then column; missing locations last);
4. Rule ID in ascending bytewise order as the final tie-breaker.

An execution error associated with an input occupies that input's position. Fatal command-usage errors precede all input output, and a final output-write or internal error is last. Summary counts are emitted after all individual records and do not affect their ordering. Implementations MAY process inputs in parallel, but MUST buffer results as needed to preserve this order.
