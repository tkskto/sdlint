use serde_json::{Map, Value};

use crate::{
    diagnostic::{Diagnostic, RuleSeverityResolver, Severity},
    input::SourceOrigin,
    parse::ParsedJsonLdDocument,
};

pub(crate) const JSONLD_CONTEXT_REQUIRED_RULE_ID: &str = "core/jsonld-context-required";
pub(crate) const JSONLD_OBJECT_REQUIRED_RULE_ID: &str = "core/jsonld-object-required";
pub(crate) const JSONLD_TYPE_RECOMMENDED_RULE_ID: &str = "core/jsonld-type-recommended";

pub(crate) const CORE_RULE_IDS: &[&str] = &[
    JSONLD_CONTEXT_REQUIRED_RULE_ID,
    JSONLD_OBJECT_REQUIRED_RULE_ID,
    JSONLD_TYPE_RECOMMENDED_RULE_ID,
];

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct JsonLdDocument {
    origin: SourceOrigin,
    json_ld_index: usize,
    node_list: Vec<JsonLdNode>,
}

impl JsonLdDocument {
    pub(crate) fn origin(&self) -> &SourceOrigin {
        &self.origin
    }

    pub(crate) fn json_ld_index(&self) -> usize {
        self.json_ld_index
    }

    pub(crate) fn nodes(&self) -> impl Iterator<Item = &JsonLdNode> {
        self.node_list.iter()
    }

    pub(crate) fn source_with_json_ld_number(&self) -> String {
        format!("{} (JSON-LD {})", self.origin(), self.json_ld_index() + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct JsonLdNode {
    origin: SourceOrigin,
    json_ld_index: usize,
    top_level_value_index: usize,
    properties: Map<String, Value>,
}

impl JsonLdNode {
    pub(crate) fn origin(&self) -> &SourceOrigin {
        &self.origin
    }

    pub(crate) fn json_ld_index(&self) -> usize {
        self.json_ld_index
    }

    pub(crate) fn top_level_value_index(&self) -> usize {
        self.top_level_value_index
    }

    pub(crate) fn type_list(&self) -> Vec<&str> {
        match self.properties.get("@type") {
            Some(Value::String(value)) => vec![value],
            Some(Value::Array(value_list)) => value_list.iter().filter_map(Value::as_str).collect(),
            _ => Vec::new(),
        }
    }

    pub(crate) fn has_property(&self, property: &str) -> bool {
        self.properties.contains_key(property)
    }

    pub(crate) fn source_with_json_ld_number(&self) -> String {
        format!("{} (JSON-LD {})", self.origin(), self.json_ld_index() + 1)
    }
}

#[derive(Debug)]
pub(crate) struct JsonLdValidation {
    document: JsonLdDocument,
    structural_diagnostic_list: Vec<JsonLdStructuralDiagnostic>,
}

impl JsonLdValidation {
    pub(crate) fn document(&self) -> &JsonLdDocument {
        &self.document
    }

    pub(crate) fn structural_diagnostics(
        &self,
    ) -> impl Iterator<Item = &JsonLdStructuralDiagnostic> {
        self.structural_diagnostic_list.iter()
    }
}

#[derive(Debug)]
pub(crate) struct JsonLdStructuralDiagnostic {
    top_level_value_index: usize,
    rule_id: &'static str,
    severity: Severity,
    message: String,
}

impl JsonLdStructuralDiagnostic {
    pub(crate) fn top_level_value_index(&self) -> usize {
        self.top_level_value_index
    }

    pub(crate) fn with_source(&self, source_with_json_ld_number: &str) -> Diagnostic {
        Diagnostic::new(
            source_with_json_ld_number,
            self.rule_id,
            self.severity,
            &self.message,
        )
    }
}

pub(crate) fn validate_parsed_document(
    parsed_document: ParsedJsonLdDocument,
    resolve_rule_severity: &RuleSeverityResolver<'_>,
) -> JsonLdValidation {
    let mut node_list = Vec::new();
    let mut structural_diagnostic_list = Vec::new();

    match parsed_document.top_level_value {
        Value::Array(value_list) => {
            for (top_level_value_index, value) in value_list.into_iter().enumerate() {
                let Value::Object(properties) = value else {
                    if let Some(severity) =
                        resolve_rule_severity(JSONLD_OBJECT_REQUIRED_RULE_ID, Severity::Error)
                    {
                        structural_diagnostic_list.push(JsonLdStructuralDiagnostic {
                            top_level_value_index,
                            rule_id: JSONLD_OBJECT_REQUIRED_RULE_ID,
                            severity,
                            message: format!(
                                "Top-level JSON-LD value {} must be an object",
                                top_level_value_index + 1
                            ),
                        });
                    }
                    continue;
                };
                validate_node_properties(
                    &parsed_document.origin,
                    parsed_document.json_ld_index,
                    top_level_value_index,
                    properties,
                    resolve_rule_severity,
                    &mut node_list,
                    &mut structural_diagnostic_list,
                );
            }
        }
        Value::Object(properties) => validate_node_properties(
            &parsed_document.origin,
            parsed_document.json_ld_index,
            0,
            properties,
            resolve_rule_severity,
            &mut node_list,
            &mut structural_diagnostic_list,
        ),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            unreachable!("parse rejects scalar JSON-LD documents")
        }
    }

    JsonLdValidation {
        document: JsonLdDocument {
            origin: parsed_document.origin,
            json_ld_index: parsed_document.json_ld_index,
            node_list,
        },
        structural_diagnostic_list,
    }
}

fn validate_node_properties(
    origin: &SourceOrigin,
    json_ld_index: usize,
    top_level_value_index: usize,
    properties: Map<String, Value>,
    resolve_rule_severity: &RuleSeverityResolver<'_>,
    node_list: &mut Vec<JsonLdNode>,
    structural_diagnostic_list: &mut Vec<JsonLdStructuralDiagnostic>,
) {
    if !properties.contains_key("@context")
        && let Some(severity) =
            resolve_rule_severity(JSONLD_CONTEXT_REQUIRED_RULE_ID, Severity::Error)
    {
        structural_diagnostic_list.push(JsonLdStructuralDiagnostic {
            top_level_value_index,
            rule_id: JSONLD_CONTEXT_REQUIRED_RULE_ID,
            severity,
            message: format!(
                "JSON-LD object {} is missing @context",
                top_level_value_index + 1
            ),
        });
    }
    if !properties.contains_key("@type")
        && !properties.contains_key("@graph")
        && let Some(severity) =
            resolve_rule_severity(JSONLD_TYPE_RECOMMENDED_RULE_ID, Severity::Warning)
    {
        structural_diagnostic_list.push(JsonLdStructuralDiagnostic {
            top_level_value_index,
            rule_id: JSONLD_TYPE_RECOMMENDED_RULE_ID,
            severity,
            message: format!(
                "JSON-LD object {} is missing @type",
                top_level_value_index + 1
            ),
        });
    }

    node_list.push(JsonLdNode {
        origin: origin.clone(),
        json_ld_index,
        top_level_value_index,
        properties,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{SourceFormat, SourceText};

    fn use_built_in_severity(_: &str, built_in_severity: Severity) -> Option<Severity> {
        Some(built_in_severity)
    }

    fn parse_json(source: &str) -> ParsedJsonLdDocument {
        crate::parse::parse_source_text(&SourceText {
            origin: SourceOrigin::Path("input.json".into()),
            format: SourceFormat::Json,
            text: source.into(),
        })
        .remove(0)
        .unwrap()
    }

    #[test]
    fn top_level_object_becomes_one_node() {
        let validation = validate_parsed_document(
            parse_json(r#"{"@context":"https://schema.org","@type":"Article"}"#),
            &use_built_in_severity,
        );
        let node_list = validation.document().nodes().collect::<Vec<_>>();

        assert_eq!(
            validation.document().origin(),
            &SourceOrigin::Path("input.json".into())
        );
        assert_eq!(validation.document().json_ld_index(), 0);
        assert_eq!(node_list.len(), 1);
        assert_eq!(node_list[0].top_level_value_index(), 0);
        assert_eq!(node_list[0].type_list(), vec!["Article"]);
    }

    #[test]
    fn object_array_preserves_node_order_and_original_numbers() {
        let validation = validate_parsed_document(
            parse_json(
                r#"[
                    {"@context":"https://schema.org","@type":"Article"},
                    {"@context":"https://schema.org","@type":"NewsArticle"}
                ]"#,
            ),
            &use_built_in_severity,
        );
        let node_list = validation.document().nodes().collect::<Vec<_>>();

        assert_eq!(node_list[0].type_list(), vec!["Article"]);
        assert_eq!(node_list[0].top_level_value_index(), 0);
        assert_eq!(node_list[1].type_list(), vec!["NewsArticle"]);
        assert_eq!(node_list[1].top_level_value_index(), 1);
    }

    #[test]
    fn document_and_node_preserve_source_and_block_index() {
        let mut parsed_document =
            parse_json(r#"{"@context":"https://schema.org","@type":"Article"}"#);
        parsed_document.json_ld_index = 2;

        let validation = validate_parsed_document(parsed_document, &use_built_in_severity);
        let node = validation.document().nodes().next().unwrap();

        assert_eq!(
            validation.document().origin(),
            &SourceOrigin::Path("input.json".into())
        );
        assert_eq!(validation.document().json_ld_index(), 2);
        assert_eq!(node.origin(), validation.document().origin());
        assert_eq!(node.json_ld_index(), 2);
        assert_eq!(node.source_with_json_ld_number(), "input.json (JSON-LD 3)");
    }

    #[test]
    fn non_object_array_value_is_a_structural_diagnostic_without_renumbering_nodes() {
        let validation = validate_parsed_document(
            parse_json(
                r#"[
                    {"@context":"https://schema.org","@type":"Article"},
                    null,
                    {"@context":"https://schema.org","@type":"BlogPosting"}
                ]"#,
            ),
            &use_built_in_severity,
        );
        let node_list = validation.document().nodes().collect::<Vec<_>>();
        let diagnostic_list = validation.structural_diagnostics().collect::<Vec<_>>();

        assert_eq!(node_list[1].top_level_value_index(), 2);
        assert_eq!(diagnostic_list.len(), 1);
        assert_eq!(diagnostic_list[0].top_level_value_index(), 1);
        assert_eq!(diagnostic_list[0].rule_id, JSONLD_OBJECT_REQUIRED_RULE_ID);
    }

    #[test]
    fn graph_property_suppresses_type_recommendation() {
        let validation = validate_parsed_document(
            parse_json(r#"{"@context":"https://schema.org","@graph":[]}"#),
            &use_built_in_severity,
        );

        assert!(validation.structural_diagnostics().next().is_none());
    }
}
