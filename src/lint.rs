use serde_json::{Map, Value};

use crate::{
    diagnostic::{Diagnostic, Severity},
    parse::ParsedDocument,
};

pub fn lint(document: &ParsedDocument) -> Vec<Diagnostic> {
    let source = document.source_label();
    match &document.value {
        Value::Array(values) => values
            .iter()
            .enumerate()
            .flat_map(|(index, value)| lint_top_level_value(&source, index, value))
            .collect(),
        Value::Object(object) => lint_top_level_object(&source, 0, object),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            unreachable!("parse rejects scalar JSON-LD documents")
        }
    }
}

fn lint_top_level_value(source: &str, index: usize, value: &Value) -> Vec<Diagnostic> {
    let Some(object) = value.as_object() else {
        return vec![Diagnostic::new(
            source,
            "core/jsonld-object-required",
            Severity::Error,
            format!("Top-level JSON-LD value {} must be an object", index + 1),
        )];
    };

    lint_top_level_object(source, index, object)
}

fn lint_top_level_object(
    source: &str,
    index: usize,
    object: &Map<String, Value>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if !object.contains_key("@context") {
        diagnostics.push(Diagnostic::new(
            source,
            "core/jsonld-context-required",
            Severity::Error,
            format!("JSON-LD object {} is missing @context", index + 1),
        ));
    }
    if !object.contains_key("@type") && !object.contains_key("@graph") {
        diagnostics.push(Diagnostic::new(
            source,
            "core/jsonld-type-recommended",
            Severity::Warning,
            format!("JSON-LD object {} is missing @type", index + 1),
        ));
    }
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::SourceId;
    use serde_json::json;

    #[test]
    fn reports_missing_context_as_error() {
        let document = ParsedDocument {
            source: SourceId::Path("input.json".into()),
            block: None,
            value: json!({"@type": "Article"}),
        };

        let diagnostics = lint(&document);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Error);
    }
}
