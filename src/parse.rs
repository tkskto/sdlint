use std::path::Path;

use scraper::{Html, Selector};
use serde_json::Value;
use thiserror::Error;

use crate::input::{SourceOrigin, SourceText};

#[derive(Debug, Clone, PartialEq)]
pub struct StructuredData {
    pub origin: SourceOrigin,
    pub json_ld_index: usize,
    pub value: Value,
}

impl StructuredData {
    pub fn source_with_json_ld_number(&self) -> String {
        format_source_with_json_ld_number(&self.origin, self.json_ld_index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("cannot parse {input}: {message}")]
    InvalidJson { input: String, message: String },
    #[error("cannot parse {input}: JSON-LD must be an object or array")]
    InvalidTopLevel { input: String },
}

pub fn parse_source_text(source_text: &SourceText) -> Vec<Result<StructuredData, ParseError>> {
    match &source_text.origin {
        SourceOrigin::Path(path) if is_html(path) => parse_html(source_text),
        _ => vec![parse_json(source_text, 0)],
    }
}

fn parse_html(source_text: &SourceText) -> Vec<Result<StructuredData, ParseError>> {
    let selector = Selector::parse("script").expect("the static script selector must be valid");
    let html = Html::parse_document(&source_text.text);
    let mut parsed_json_ld_list = Vec::new();

    for element in html.select(&selector) {
        let is_json_ld = element
            .value()
            .attr("type")
            .is_some_and(|value| value.eq_ignore_ascii_case("application/ld+json"));
        if is_json_ld {
            let json_ld_index = parsed_json_ld_list.len();
            let json_ld_source = SourceText {
                origin: source_text.origin.clone(),
                text: element.inner_html(),
            };
            parsed_json_ld_list.push(parse_json(&json_ld_source, json_ld_index));
        }
    }

    parsed_json_ld_list
}

fn parse_json(
    source_text: &SourceText,
    json_ld_index: usize,
) -> Result<StructuredData, ParseError> {
    let value = serde_json::from_str::<Value>(&source_text.text).map_err(|error| {
        ParseError::InvalidJson {
            input: format_source_with_json_ld_number(&source_text.origin, json_ld_index),
            message: error.to_string(),
        }
    })?;

    if !matches!(value, Value::Object(_) | Value::Array(_)) {
        return Err(ParseError::InvalidTopLevel {
            input: format_source_with_json_ld_number(&source_text.origin, json_ld_index),
        });
    }

    Ok(StructuredData {
        origin: source_text.origin.clone(),
        json_ld_index,
        value,
    })
}

fn is_html(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| matches!(extension.to_ascii_lowercase().as_str(), "html" | "htm"))
}

fn format_source_with_json_ld_number(source_origin: &SourceOrigin, json_ld_index: usize) -> String {
    format!("{} (JSON-LD {})", source_origin, json_ld_index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json_object() {
        let source_text = SourceText {
            origin: SourceOrigin::Path("input.json".into()),
            text: r#"{"@context":"https://schema.org"}"#.into(),
        };

        let structured_data = parse_source_text(&source_text).remove(0).unwrap();

        assert_eq!(structured_data.json_ld_index, 0);
        assert!(structured_data.value.is_object());
    }

    #[test]
    fn extracts_json_ld_scripts_from_html() {
        let source_text = SourceText {
            origin: SourceOrigin::Path("input.html".into()),
            text: r#"
                <script type="application/json">{"ignored":true}</script>
                <script type="application/ld+json">{"@type":"Article"}</script>
                <script type="application/ld+json">{"@type":"NewsArticle"}</script>
            "#
            .into(),
        };

        let parsed_json_ld_list = parse_source_text(&source_text);

        assert_eq!(parsed_json_ld_list.len(), 2);
        assert_eq!(parsed_json_ld_list[0].as_ref().unwrap().json_ld_index, 0);
        assert_eq!(parsed_json_ld_list[1].as_ref().unwrap().json_ld_index, 1);
    }

    #[test]
    fn continues_after_invalid_json_ld_in_html() {
        let source_text = SourceText {
            origin: SourceOrigin::Path("input.html".into()),
            text: r#"
                <script type="application/ld+json">{"@type":"Article"}</script>
                <script type="application/ld+json">invalid</script>
                <script type="application/ld+json">{"@type":"NewsArticle"}</script>
            "#
            .into(),
        };

        let parsed_json_ld_list = parse_source_text(&source_text);

        assert_eq!(parsed_json_ld_list.len(), 3);
        assert_eq!(parsed_json_ld_list[0].as_ref().unwrap().json_ld_index, 0);
        assert!(parsed_json_ld_list[1].is_err());
        assert_eq!(parsed_json_ld_list[2].as_ref().unwrap().json_ld_index, 2);
    }
}
