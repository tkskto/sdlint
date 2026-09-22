use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{diagnostic::Severity, rules};

use super::ConfigError;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum RuleLevel {
    Off,
    Info,
    Warning,
    Error,
}

impl RuleLevel {
    pub(super) fn severity(self) -> Option<Severity> {
        match self {
            Self::Off => None,
            Self::Info => Some(Severity::Info),
            Self::Warning => Some(Severity::Warning),
            Self::Error => Some(Severity::Error),
        }
    }
}

#[derive(Debug)]
pub(super) struct RuleSettings {
    level_by_rule_id: BTreeMap<String, RuleLevel>,
}

impl RuleSettings {
    pub(super) fn build(
        configured_rules: BTreeMap<String, RuleLevel>,
        location: &str,
    ) -> Result<Self, ConfigError> {
        let known_rule_id_list = rules::known_rule_ids();
        for rule_id in configured_rules.keys() {
            if !known_rule_id_list.contains(&rule_id.as_str()) {
                return Err(ConfigError::UnknownRule {
                    rule_id: rule_id.clone(),
                    location: location.to_owned(),
                });
            }
        }

        Ok(Self {
            level_by_rule_id: configured_rules,
        })
    }

    pub(super) fn find(&self, rule_id: &str) -> Option<RuleLevel> {
        self.level_by_rule_id.get(rule_id).copied()
    }
}
