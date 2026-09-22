use super::{Assertion, Condition, RuleDefinition};
use crate::diagnostic::Severity;

const ARTICLE_TYPES: &[&str] = &["Article", "NewsArticle", "BlogPosting"];

pub(super) const RULES: &[RuleDefinition] = &[
    RuleDefinition {
        id: "schema/article/article-body-present",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("articleBody"),
        message: "Article is missing optional Schema.org property articleBody",
    },
    RuleDefinition {
        id: "schema/article/article-section-present",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("articleSection"),
        message: "Article is missing optional Schema.org property articleSection",
    },
    RuleDefinition {
        id: "schema/article/backstory-present",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("backstory"),
        message: "Article is missing optional Schema.org property backstory",
    },
    RuleDefinition {
        id: "schema/article/page-end-present",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("pageEnd"),
        message: "Article is missing optional Schema.org property pageEnd",
    },
    RuleDefinition {
        id: "schema/article/page-start-present",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("pageStart"),
        message: "Article is missing optional Schema.org property pageStart",
    },
    RuleDefinition {
        id: "schema/article/pagination-present",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("pagination"),
        message: "Article is missing optional Schema.org property pagination",
    },
    RuleDefinition {
        id: "schema/article/speakable-present",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("speakable"),
        message: "Article is missing optional Schema.org property speakable",
    },
    RuleDefinition {
        id: "schema/article/word-count-present",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("wordCount"),
        message: "Article is missing optional Schema.org property wordCount",
    },
    RuleDefinition {
        id: "google/article/author-recommended",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("author"),
        message: "Article is missing recommended property author",
    },
    RuleDefinition {
        id: "google/article/date-published-recommended",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("datePublished"),
        message: "Article is missing recommended property datePublished",
    },
    RuleDefinition {
        id: "google/article/date-modified-recommended",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("dateModified"),
        message: "Article is missing recommended property dateModified",
    },
    RuleDefinition {
        id: "google/article/headline-recommended",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("headline"),
        message: "Article is missing recommended property headline",
    },
    RuleDefinition {
        id: "google/article/image-recommended",
        severity: Severity::Warning,
        applies_to: Condition::TargetTypes(ARTICLE_TYPES),
        assertion: Assertion::PropertyPresent("image"),
        message: "Article is missing recommended property image",
    },
];
