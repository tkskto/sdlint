use crate::{
    diagnostic::{Diagnostic, RuleSeverityResolver},
    json_ld::JsonLdValidation,
    rules,
};

pub(crate) fn lint_json_ld(
    validation: &JsonLdValidation,
    resolve_rule_severity: &RuleSeverityResolver<'_>,
) -> Vec<Diagnostic> {
    let source_with_json_ld_number = validation.document().source_with_json_ld_number();
    let mut indexed_diagnostic_list = validation
        .structural_diagnostics()
        .map(|structural_diagnostic| IndexedDiagnostic {
            top_level_value_index: structural_diagnostic.top_level_value_index(),
            diagnostic: structural_diagnostic.with_source(&source_with_json_ld_number),
        })
        .collect::<Vec<_>>();

    indexed_diagnostic_list.extend(validation.document().nodes().flat_map(|node| {
        rules::check_node(node, resolve_rule_severity)
            .into_iter()
            .map(|diagnostic| IndexedDiagnostic {
                top_level_value_index: node.top_level_value_index(),
                diagnostic,
            })
    }));

    indexed_diagnostic_list.sort_by(|left, right| {
        left.top_level_value_index
            .cmp(&right.top_level_value_index)
            .then_with(
                || match (&left.diagnostic.location, &right.diagnostic.location) {
                    (Some(left), Some(right)) => {
                        (left.line, left.column).cmp(&(right.line, right.column))
                    }
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                },
            )
            .then_with(|| {
                left.diagnostic
                    .rule_id
                    .as_bytes()
                    .cmp(right.diagnostic.rule_id.as_bytes())
            })
    });

    indexed_diagnostic_list
        .into_iter()
        .map(|indexed_diagnostic| indexed_diagnostic.diagnostic)
        .collect()
}

struct IndexedDiagnostic {
    top_level_value_index: usize,
    diagnostic: Diagnostic,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diagnostic::Severity,
        input::{SourceFormat, SourceOrigin, SourceText},
        json_ld::{
            JSONLD_CONTEXT_REQUIRED_RULE_ID, JSONLD_OBJECT_REQUIRED_RULE_ID,
            JSONLD_TYPE_RECOMMENDED_RULE_ID, validate_parsed_document,
        },
    };

    fn use_built_in_severity(_: &str, built_in_severity: Severity) -> Option<Severity> {
        Some(built_in_severity)
    }

    fn lint_json(source: &str) -> Vec<Diagnostic> {
        lint_json_with_severity(source, &use_built_in_severity)
    }

    fn lint_json_with_severity(
        source: &str,
        resolve_rule_severity: &RuleSeverityResolver<'_>,
    ) -> Vec<Diagnostic> {
        let parsed_document = crate::parse::parse_source_text(&SourceText {
            origin: SourceOrigin::Path("input.json".into()),
            format: SourceFormat::Json,
            text: source.into(),
        })
        .remove(0)
        .unwrap();
        let validation = validate_parsed_document(parsed_document, resolve_rule_severity);
        lint_json_ld(&validation, resolve_rule_severity)
    }

    #[test]
    fn article_without_context_reports_structural_and_feature_diagnostics() {
        let diagnostic_list = lint_json(r#"{"@type":"Article"}"#);

        assert!(
            diagnostic_list
                .iter()
                .any(|diagnostic| diagnostic.rule_id == JSONLD_CONTEXT_REQUIRED_RULE_ID)
        );
        assert!(
            diagnostic_list
                .iter()
                .any(|diagnostic| { diagnostic.rule_id == "google/article/headline-recommended" })
        );
    }

    #[test]
    fn object_without_type_or_graph_reports_type_recommendation() {
        let diagnostic_list = lint_json(r#"{"@context":"https://schema.org"}"#);

        assert_eq!(diagnostic_list.len(), 1);
        assert_eq!(diagnostic_list[0].rule_id, JSONLD_TYPE_RECOMMENDED_RULE_ID);
    }

    #[test]
    fn non_object_array_value_does_not_stop_feature_rules_for_other_nodes() {
        let diagnostic_list = lint_json(
            r#"[
                null,
                {"@context":"https://schema.org","@type":"Article"}
            ]"#,
        );

        assert_eq!(diagnostic_list[0].rule_id, JSONLD_OBJECT_REQUIRED_RULE_ID);
        assert!(diagnostic_list.iter().any(|diagnostic| {
            diagnostic.rule_id == "google/article/headline-recommended"
                && diagnostic.message.ends_with("JSON-LD object 2)")
        }));
    }

    #[test]
    fn disabled_core_rule_does_not_produce_a_structural_diagnostic() {
        let resolve_rule_severity = |rule_id: &str, built_in_severity| {
            (rule_id != JSONLD_CONTEXT_REQUIRED_RULE_ID).then_some(built_in_severity)
        };

        let diagnostic_list =
            lint_json_with_severity(r#"{"@type":"Organization"}"#, &resolve_rule_severity);

        assert!(diagnostic_list.is_empty());
    }

    #[test]
    fn core_rule_severity_override_is_applied() {
        let resolve_rule_severity = |rule_id: &str, built_in_severity| {
            if rule_id == JSONLD_TYPE_RECOMMENDED_RULE_ID {
                Some(Severity::Error)
            } else {
                Some(built_in_severity)
            }
        };

        let diagnostic_list = lint_json_with_severity(
            r#"{"@context":"https://schema.org"}"#,
            &resolve_rule_severity,
        );

        assert_eq!(diagnostic_list[0].severity, Severity::Error);
    }

    #[test]
    fn feature_rule_activation_and_severity_are_applied() {
        let resolve_rule_severity = |rule_id: &str, built_in_severity| match rule_id {
            "google/article/headline-recommended" => None,
            "google/article/image-recommended" => Some(Severity::Error),
            _ => Some(built_in_severity),
        };

        let diagnostic_list = lint_json_with_severity(
            r#"{"@context":"https://schema.org","@type":"Article"}"#,
            &resolve_rule_severity,
        );

        assert!(
            !diagnostic_list
                .iter()
                .any(|diagnostic| { diagnostic.rule_id == "google/article/headline-recommended" })
        );
        assert!(diagnostic_list.iter().any(|diagnostic| {
            diagnostic.rule_id == "google/article/image-recommended"
                && diagnostic.severity == Severity::Error
        }));
    }

    #[test]
    fn structural_and_feature_diagnostics_for_one_node_are_rule_id_sorted() {
        let diagnostic_list = lint_json(r#"{"@type":"Article"}"#);

        assert!(diagnostic_list.windows(2).all(|diagnostic_pair| {
            diagnostic_pair[0].rule_id.as_bytes() <= diagnostic_pair[1].rule_id.as_bytes()
        }));
    }

    #[test]
    fn article_family_and_schema_org_url_types_receive_article_rules() {
        for article_type in [
            "Article",
            "NewsArticle",
            "BlogPosting",
            "https://schema.org/Article",
            "http://schema.org/NewsArticle",
        ] {
            let source = format!(r#"{{"@context":"https://schema.org","@type":"{article_type}"}}"#);
            let diagnostic_list = lint_json(&source);

            assert!(
                diagnostic_list.iter().any(|diagnostic| {
                    diagnostic.rule_id == "google/article/headline-recommended"
                })
            );
        }
    }
}
