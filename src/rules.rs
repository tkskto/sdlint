use serde_json::{Map, Value};

use crate::diagnostic::{Diagnostic, Severity};

mod article;
mod definitions;

pub(crate) const JSONLD_CONTEXT_REQUIRED_RULE_ID: &str = "core/jsonld-context-required";
pub(crate) const JSONLD_OBJECT_REQUIRED_RULE_ID: &str = "core/jsonld-object-required";
pub(crate) const JSONLD_TYPE_RECOMMENDED_RULE_ID: &str = "core/jsonld-type-recommended";

const CORE_RULE_IDS: &[&str] = &[
    JSONLD_CONTEXT_REQUIRED_RULE_ID,
    JSONLD_OBJECT_REQUIRED_RULE_ID,
    JSONLD_TYPE_RECOMMENDED_RULE_ID,
];

pub(crate) fn known_rule_ids() -> Vec<&'static str> {
    CORE_RULE_IDS
        .iter()
        .copied()
        .chain(definitions::ALL.iter().map(|rule| rule.id))
        .collect()
}

pub(crate) type RuleSeverityResolver<'a> = dyn Fn(&str, Severity) -> Option<Severity> + 'a;

pub(crate) struct RuleContext<'a> {
    source: &'a str,
    object_index: usize,
    object: &'a Map<String, Value>,
}

pub(crate) enum Condition {
    TargetTypes(&'static [&'static str]),
}

pub(crate) enum Assertion {
    PropertyPresent(&'static str),
}

pub(crate) struct RuleDefinition {
    id: &'static str,
    severity: Severity,
    applies_to: Condition,
    assertion: Assertion,
    message: &'static str,
}

pub(crate) fn check_object(
    source: &str,
    object_index: usize,
    object: &Map<String, Value>,
    resolve_rule_severity: &RuleSeverityResolver<'_>,
) -> Vec<Diagnostic> {
    let context = RuleContext {
        source,
        object_index,
        object,
    };

    definitions::ALL
        .iter()
        .filter_map(|rule| {
            let severity = resolve_rule_severity(rule.id, rule.severity)?;
            if !matches_condition(&rule.applies_to, &context)
                || matches_assertion(&rule.assertion, &context)
            {
                return None;
            }

            Some(Diagnostic::new(
                context.source,
                rule.id,
                severity,
                format!(
                    "{} (JSON-LD object {})",
                    rule.message,
                    context.object_index + 1
                ),
            ))
        })
        .collect()
}

fn matches_condition(condition: &Condition, context: &RuleContext<'_>) -> bool {
    match condition {
        Condition::TargetTypes(expected_types) => {
            object_types(context.object).into_iter().any(|actual_type| {
                expected_types
                    .iter()
                    .any(|expected| type_matches(actual_type, expected))
            })
        }
    }
}

fn matches_assertion(assertion: &Assertion, context: &RuleContext<'_>) -> bool {
    match assertion {
        Assertion::PropertyPresent(property) => context.object.contains_key(*property),
    }
}

fn object_types(object: &Map<String, Value>) -> Vec<&str> {
    match object.get("@type") {
        Some(Value::String(value)) => vec![value],
        Some(Value::Array(values)) => values.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    }
}

fn type_matches(actual: &str, expected: &str) -> bool {
    normalize_schema_type(actual) == normalize_schema_type(expected)
}

fn normalize_schema_type(value: &str) -> &str {
    value
        .strip_prefix("https://schema.org/")
        .or_else(|| value.strip_prefix("http://schema.org/"))
        .unwrap_or(value)
}
