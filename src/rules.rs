use serde_json::{Map, Value};

use crate::diagnostic::{Diagnostic, Severity};

mod article;
mod definitions;

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
) -> Vec<Diagnostic> {
    let context = RuleContext {
        source,
        object_index,
        object,
    };

    definitions::ALL
        .iter()
        .filter(|rule| {
            matches_condition(&rule.applies_to, &context)
                && !matches_assertion(&rule.assertion, &context)
        })
        .map(|rule| {
            Diagnostic::new(
                context.source,
                rule.id,
                rule.severity,
                format!(
                    "{} (JSON-LD object {})",
                    rule.message,
                    context.object_index + 1
                ),
            )
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
