use serde_json::{Map, Value};

use crate::{
    diagnostic::{Diagnostic, Severity},
    parse::StructuredData,
    rules,
};

pub fn lint_structured_data(structured_data: &StructuredData) -> Vec<Diagnostic> {
    let source_with_json_ld_number = structured_data.source_with_json_ld_number();
    match &structured_data.value {
        Value::Array(values) => values
            .iter()
            .enumerate()
            .flat_map(|(top_level_value_index, value)| {
                lint_top_level_value(&source_with_json_ld_number, top_level_value_index, value)
            })
            .collect(),
        Value::Object(object) => lint_top_level_object(&source_with_json_ld_number, 0, object),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            unreachable!("parse rejects scalar JSON-LD documents")
        }
    }
}

fn lint_top_level_value(
    source_with_json_ld_number: &str,
    top_level_value_index: usize,
    value: &Value,
) -> Vec<Diagnostic> {
    let Some(object) = value.as_object() else {
        return vec![Diagnostic::new(
            source_with_json_ld_number,
            "core/jsonld-object-required",
            Severity::Error,
            format!(
                "Top-level JSON-LD value {} must be an object",
                top_level_value_index + 1
            ),
        )];
    };

    lint_top_level_object(source_with_json_ld_number, top_level_value_index, object)
}

fn lint_top_level_object(
    source_with_json_ld_number: &str,
    top_level_value_index: usize,
    object: &Map<String, Value>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if !object.contains_key("@context") {
        diagnostics.push(Diagnostic::new(
            source_with_json_ld_number,
            "core/jsonld-context-required",
            Severity::Error,
            format!(
                "JSON-LD object {} is missing @context",
                top_level_value_index + 1
            ),
        ));
    }
    if !object.contains_key("@type") && !object.contains_key("@graph") {
        diagnostics.push(Diagnostic::new(
            source_with_json_ld_number,
            "core/jsonld-type-recommended",
            Severity::Warning,
            format!(
                "JSON-LD object {} is missing @type",
                top_level_value_index + 1
            ),
        ));
    }
    diagnostics.extend(rules::check_object(
        source_with_json_ld_number,
        top_level_value_index,
        object,
    ));
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::SourceOrigin;
    use serde_json::json;

    #[test]
    fn reports_missing_context_as_error() {
        let structured_data = StructuredData {
            origin: SourceOrigin::Path("input.json".into()),
            json_ld_index: 0,
            value: json!({"@type": "Organization"}),
        };

        let diagnostics = lint_structured_data(&structured_data);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Error);
    }

    #[test]
    fn reports_missing_article_headline_as_warning() {
        let structured_data = StructuredData {
            origin: SourceOrigin::Path("input.json".into()),
            json_ld_index: 0,
            value: json!({
                "@context": "https://schema.org",
                "@type": ["Article", "CreativeWork"],
                "author": {"@type": "Person", "name": "Example"},
                "datePublished": "2026-08-20",
                "dateModified": "2026-08-20",
                "image": "https://example.com/image.jpg",
                "articleBody": "Article content.",
                "articleSection": "Technology",
                "backstory": "Background.",
                "pageEnd": 10,
                "pageStart": 1,
                "pagination": "1-10",
                "speakable": "https://example.com/article#headline",
                "wordCount": 100
            }),
        };

        let diagnostics = lint_structured_data(&structured_data);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].rule_id,
            "google/article/headline-recommended"
        );
        assert_eq!(diagnostics[0].severity, Severity::Warning);
    }

    #[test]
    fn does_not_apply_article_rules_to_other_types() {
        let structured_data = StructuredData {
            origin: SourceOrigin::Path("input.json".into()),
            json_ld_index: 0,
            value: json!({
                "@context": "https://schema.org",
                "@type": "Organization"
            }),
        };

        assert!(lint_structured_data(&structured_data).is_empty());
    }
}
