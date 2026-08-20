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
    diagnostics.extend(lint_article_headline(source, index, object));
    diagnostics
}

fn lint_article_headline(
    source: &str,
    index: usize,
    object: &Map<String, Value>,
) -> Vec<Diagnostic> {
    let is_article = match object.get("@type") {
        Some(Value::String(value)) => is_article_type(value),
        Some(Value::Array(values)) => values.iter().filter_map(Value::as_str).any(is_article_type),
        _ => false,
    };

    if is_article && !object.contains_key("headline") {
        vec![Diagnostic::new(
            source,
            "google/article/headline-recommended",
            Severity::Warning,
            format!(
                "Article object {} is missing recommended property headline",
                index + 1
            ),
        )]
    } else {
        Vec::new()
    }
}

fn is_article_type(value: &str) -> bool {
    matches!(
        value,
        "Article"
            | "NewsArticle"
            | "BlogPosting"
            | "https://schema.org/Article"
            | "https://schema.org/NewsArticle"
            | "https://schema.org/BlogPosting"
            | "http://schema.org/Article"
            | "http://schema.org/NewsArticle"
            | "http://schema.org/BlogPosting"
    )
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
            value: json!({"@type": "Organization"}),
        };

        let diagnostics = lint(&document);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Error);
    }

    #[test]
    fn reports_missing_article_headline_as_warning() {
        let document = ParsedDocument {
            source: SourceId::Path("input.json".into()),
            block: None,
            value: json!({
                "@context": "https://schema.org",
                "@type": ["Article", "CreativeWork"]
            }),
        };

        let diagnostics = lint(&document);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].rule_id,
            "google/article/headline-recommended"
        );
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }

    #[test]
    fn does_not_apply_article_rules_to_other_types() {
        let document = ParsedDocument {
            source: SourceId::Path("input.json".into()),
            block: None,
            value: json!({
                "@context": "https://schema.org",
                "@type": "Organization"
            }),
        };

        assert!(lint(&document).is_empty());
    }
}
