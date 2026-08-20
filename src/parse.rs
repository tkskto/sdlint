use std::path::Path;

use scraper::{Html, Selector};
use serde_json::Value;
use thiserror::Error;

use crate::input::{SourceDocument, SourceId};

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedDocument {
    pub source: SourceId,
    pub block: Option<usize>,
    pub value: Value,
}

impl ParsedDocument {
    pub fn source_label(&self) -> String {
        source_label(&self.source, self.block)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("cannot parse {input}: {message}")]
    InvalidJson { input: String, message: String },
    #[error("cannot parse {input}: JSON-LD must be an object or array")]
    InvalidTopLevel { input: String },
}

pub fn parse(document: &SourceDocument) -> Result<Vec<ParsedDocument>, ParseError> {
    match &document.source {
        SourceId::Path(path) if is_html(path) => parse_html(document),
        _ => parse_json(document, None).map(|document| vec![document]),
    }
}

fn parse_html(document: &SourceDocument) -> Result<Vec<ParsedDocument>, ParseError> {
    let selector = Selector::parse("script").expect("the static script selector must be valid");
    let html = Html::parse_document(&document.text);
    let mut documents = Vec::new();

    for element in html.select(&selector) {
        let is_json_ld = element
            .value()
            .attr("type")
            .is_some_and(|value| value.eq_ignore_ascii_case("application/ld+json"));
        if is_json_ld {
            let block = documents.len();
            let script = SourceDocument {
                source: document.source.clone(),
                text: element.inner_html(),
            };
            documents.push(parse_json(&script, Some(block))?);
        }
    }

    Ok(documents)
}

fn parse_json(
    document: &SourceDocument,
    block: Option<usize>,
) -> Result<ParsedDocument, ParseError> {
    let value =
        serde_json::from_str::<Value>(&document.text).map_err(|error| ParseError::InvalidJson {
            input: source_label(&document.source, block),
            message: error.to_string(),
        })?;

    if !matches!(value, Value::Object(_) | Value::Array(_)) {
        return Err(ParseError::InvalidTopLevel {
            input: source_label(&document.source, block),
        });
    }

    Ok(ParsedDocument {
        source: document.source.clone(),
        block,
        value,
    })
}

fn is_html(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| matches!(extension.to_ascii_lowercase().as_str(), "html" | "htm"))
}

fn source_label(source: &SourceId, block: Option<usize>) -> String {
    match block {
        Some(index) => format!("{} (JSON-LD block {})", source, index + 1),
        None => source.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json_object() {
        let document = SourceDocument {
            source: SourceId::Path("input.json".into()),
            text: r#"{"@context":"https://schema.org"}"#.into(),
        };

        let result = parse(&document).unwrap();

        assert_eq!(result.len(), 1);
        assert!(result[0].value.is_object());
    }

    #[test]
    fn extracts_json_ld_scripts_from_html() {
        let document = SourceDocument {
            source: SourceId::Path("input.html".into()),
            text: r#"
                <script type="application/json">{"ignored":true}</script>
                <script type="application/ld+json">{"@type":"Article"}</script>
                <script type="application/ld+json">{"@type":"NewsArticle"}</script>
            "#
            .into(),
        };

        let result = parse(&document).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].block, Some(0));
        assert_eq!(result[1].block, Some(1));
    }
}
