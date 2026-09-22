use serde_json::{Map, Value};

use crate::{
    diagnostic::{Diagnostic, Severity},
    parse::StructuredData,
    rules,
};

pub fn lint_structured_data(
    structured_data: &StructuredData,
    resolve_rule_severity: &rules::RuleSeverityResolver<'_>,
) -> Vec<Diagnostic> {
    let source_with_json_ld_number = structured_data.source_with_json_ld_number();
    match &structured_data.value {
        Value::Array(values) => values
            .iter()
            .enumerate()
            .flat_map(|(top_level_value_index, value)| {
                lint_top_level_value(
                    &source_with_json_ld_number,
                    top_level_value_index,
                    value,
                    resolve_rule_severity,
                )
            })
            .collect(),
        Value::Object(object) => lint_top_level_object(
            &source_with_json_ld_number,
            0,
            object,
            resolve_rule_severity,
        ),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            unreachable!("parse rejects scalar JSON-LD documents")
        }
    }
}

fn lint_top_level_value(
    source_with_json_ld_number: &str,
    top_level_value_index: usize,
    value: &Value,
    resolve_rule_severity: &rules::RuleSeverityResolver<'_>,
) -> Vec<Diagnostic> {
    let Some(object) = value.as_object() else {
        return resolve_rule_severity(rules::JSONLD_OBJECT_REQUIRED_RULE_ID, Severity::Error)
            .map(|severity| {
                vec![Diagnostic::new(
                    source_with_json_ld_number,
                    rules::JSONLD_OBJECT_REQUIRED_RULE_ID,
                    severity,
                    format!(
                        "Top-level JSON-LD value {} must be an object",
                        top_level_value_index + 1
                    ),
                )]
            })
            .unwrap_or_default();
    };

    lint_top_level_object(
        source_with_json_ld_number,
        top_level_value_index,
        object,
        resolve_rule_severity,
    )
}

fn lint_top_level_object(
    source_with_json_ld_number: &str,
    top_level_value_index: usize,
    object: &Map<String, Value>,
    resolve_rule_severity: &rules::RuleSeverityResolver<'_>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    if let Some(severity) =
        resolve_rule_severity(rules::JSONLD_CONTEXT_REQUIRED_RULE_ID, Severity::Error)
    {
        if !object.contains_key("@context") {
            diagnostics.push(Diagnostic::new(
                source_with_json_ld_number,
                rules::JSONLD_CONTEXT_REQUIRED_RULE_ID,
                severity,
                format!(
                    "JSON-LD object {} is missing @context",
                    top_level_value_index + 1
                ),
            ));
        }
    }
    if let Some(severity) =
        resolve_rule_severity(rules::JSONLD_TYPE_RECOMMENDED_RULE_ID, Severity::Warning)
    {
        if !object.contains_key("@type") && !object.contains_key("@graph") {
            diagnostics.push(Diagnostic::new(
                source_with_json_ld_number,
                rules::JSONLD_TYPE_RECOMMENDED_RULE_ID,
                severity,
                format!(
                    "JSON-LD object {} is missing @type",
                    top_level_value_index + 1
                ),
            ));
        }
    }
    diagnostics.extend(rules::check_object(
        source_with_json_ld_number,
        top_level_value_index,
        object,
        resolve_rule_severity,
    ));
    diagnostics.sort_by(|left, right| {
        match (&left.location, &right.location) {
            (Some(left), Some(right)) => (left.line, left.column).cmp(&(right.line, right.column)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
        .then_with(|| left.rule_id.as_bytes().cmp(right.rule_id.as_bytes()))
    });
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::SourceOrigin;
    use serde_json::json;

    fn use_built_in_severity(_: &str, built_in_severity: Severity) -> Option<Severity> {
        Some(built_in_severity)
    }

    #[test]
    fn reports_missing_context_as_error() {
        let structured_data = StructuredData {
            origin: SourceOrigin::Path("input.json".into()),
            json_ld_index: 0,
            value: json!({"@type": "Organization"}),
        };

        let diagnostics = lint_structured_data(&structured_data, &use_built_in_severity);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Error);
    }

    #[test]
    fn applies_rule_activation_and_severity_during_linting() {
        let structured_data = StructuredData {
            origin: SourceOrigin::Path("input.json".into()),
            json_ld_index: 0,
            value: json!({}),
        };
        let resolve_rule_severity = |rule_id: &str, built_in_severity| match rule_id {
            rules::JSONLD_CONTEXT_REQUIRED_RULE_ID => None,
            rules::JSONLD_TYPE_RECOMMENDED_RULE_ID => Some(Severity::Error),
            _ => Some(built_in_severity),
        };

        let diagnostics = lint_structured_data(&structured_data, &resolve_rule_severity);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].rule_id,
            rules::JSONLD_TYPE_RECOMMENDED_RULE_ID
        );
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

        let diagnostics = lint_structured_data(&structured_data, &use_built_in_severity);

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

        assert!(lint_structured_data(&structured_data, &use_built_in_severity).is_empty());
    }
}
