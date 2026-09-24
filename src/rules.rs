use crate::{
    diagnostic::{Diagnostic, RuleSeverityResolver, Severity},
    json_ld::{self, JsonLdNode},
};

mod article;
mod definitions;

pub(crate) fn known_rule_ids() -> Vec<&'static str> {
    json_ld::CORE_RULE_IDS
        .iter()
        .copied()
        .chain(definitions::ALL.iter().map(|rule| rule.id))
        .collect()
}

pub(crate) struct RuleContext<'a> {
    node: &'a JsonLdNode,
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

pub(crate) fn check_node(
    node: &JsonLdNode,
    resolve_rule_severity: &RuleSeverityResolver<'_>,
) -> Vec<Diagnostic> {
    let context = RuleContext { node };

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
                context.node.source_with_json_ld_number(),
                rule.id,
                severity,
                format!(
                    "{} (JSON-LD object {})",
                    rule.message,
                    context.node.top_level_value_index() + 1
                ),
            ))
        })
        .collect()
}

fn matches_condition(condition: &Condition, context: &RuleContext<'_>) -> bool {
    match condition {
        Condition::TargetTypes(expected_types) => {
            context.node.type_list().into_iter().any(|actual_type| {
                expected_types
                    .iter()
                    .any(|expected| type_matches(actual_type, expected))
            })
        }
    }
}

fn matches_assertion(assertion: &Assertion, context: &RuleContext<'_>) -> bool {
    match assertion {
        Assertion::PropertyPresent(property) => context.node.has_property(property),
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
