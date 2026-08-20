use std::io::{self, Write};

use serde_json::json;

use crate::{
    cli::OutputFormat,
    diagnostic::{Diagnostic, Severity},
};

pub fn write(
    diagnostics: &[Diagnostic],
    format: OutputFormat,
    color: bool,
    output: &mut dyn Write,
) -> io::Result<()> {
    match format {
        OutputFormat::Text => write_text(diagnostics, color, output),
        OutputFormat::Json => write_json(diagnostics, output),
    }
}

fn write_text(diagnostics: &[Diagnostic], color: bool, output: &mut dyn Write) -> io::Result<()> {
    for diagnostic in diagnostics {
        let severity = format_severity(diagnostic.severity, color);
        match &diagnostic.location {
            Some(location) => writeln!(
                output,
                "{}:{}:{} {} {}: {}",
                diagnostic.source,
                location.line,
                location.column,
                severity,
                diagnostic.rule_id,
                diagnostic.message
            )?,
            None => writeln!(
                output,
                "{} {} {}: {}",
                diagnostic.source, severity, diagnostic.rule_id, diagnostic.message
            )?,
        }
    }
    Ok(())
}

fn write_json(diagnostics: &[Diagnostic], output: &mut dyn Write) -> io::Result<()> {
    let records = diagnostics.iter().map(|diagnostic| {
        json!({
            "source": diagnostic.source,
            "location": diagnostic.location.as_ref().map(|location| json!({
                "line": location.line,
                "column": location.column,
            })),
            "rule_id": diagnostic.rule_id,
            "severity": diagnostic.severity.to_string(),
            "message": diagnostic.message,
        })
    });
    let value = serde_json::Value::Array(records.collect());
    serde_json::to_writer(&mut *output, &value).map_err(io::Error::other)?;
    writeln!(output)
}

fn format_severity(severity: Severity, color: bool) -> String {
    let label = severity.to_string();
    if !color {
        return label;
    }

    let color_code = match severity {
        Severity::Info => 36,
        Severity::Warning => 33,
        Severity::Error => 31,
    };
    format!("\x1b[{color_code}m{label}\x1b[0m")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_json_array() {
        let diagnostics = vec![Diagnostic::new(
            "input.json",
            "core/example",
            Severity::Warning,
            "example",
        )];
        let mut output = Vec::new();

        write(&diagnostics, OutputFormat::Json, false, &mut output).unwrap();

        let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(value[0]["rule_id"], "core/example");
    }
}
