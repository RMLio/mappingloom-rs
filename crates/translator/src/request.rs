use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

use crate::report::{Kind, Record, TranslationErrors};

/// The language of a mapping document. The caller names it: the translator
/// guesses nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Language {
    RmlKgc,
    RmlIo,
    R2rml,
    Shexml,
}

/// The options of a translation request, read from a JSON object.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Options {
    pub language: Language,
    /// The processor's base IRI, which every triples map without
    /// `rml:baseIRI` takes; without it, the mapping's `@base`.
    #[serde(default)]
    pub base_iri: Option<String>,
    /// Switches optimizer rules on or off by their id; a rule without a
    /// toggle takes its default.
    #[serde(default)]
    pub optimizer: BTreeMap<String, bool>,
}

impl Options {
    pub fn new(language: Language) -> Self {
        Self {
            language,
            base_iri: None,
            optimizer: BTreeMap::new(),
        }
    }

    /// Reads the options object. An unknown key or value is a request error,
    /// so a misspelt option never passes unnoticed.
    pub fn from_value(value: Value) -> Result<Self, TranslationErrors> {
        serde_json::from_value(value).map_err(invalid_options)
    }

    /// Reads the options object from JSON text.
    pub fn from_json(text: &str) -> Result<Self, TranslationErrors> {
        serde_json::from_str(text).map_err(invalid_options)
    }
}

fn invalid_options(err: serde_json::Error) -> TranslationErrors {
    Record::new(Kind::Request, "invalid-options", err.to_string()).into()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn reads_every_option() {
        let options = Options::from_value(json!({
            "language": "rml-kgc",
            "base_iri": "http://example.com/",
            "optimizer": {"R1": false}
        }))
        .unwrap();
        assert_eq!(
            options,
            Options {
                language: Language::RmlKgc,
                base_iri: Some("http://example.com/".to_string()),
                optimizer: BTreeMap::from([("R1".to_string(), false)]),
            }
        );
    }

    #[test]
    fn an_unknown_key_or_value_or_a_missing_language_is_a_request_error() {
        for options in [
            json!({"language": "rml-kgc", "baseIRI": "http://example.com/"}),
            json!({"language": "rml"}),
            json!({"base_iri": "http://example.com/"}),
            json!("rml-kgc"),
        ] {
            let errors = Options::from_value(options.clone()).unwrap_err();
            assert_eq!(errors.errors[0].kind, Kind::Request, "{options}");
            assert_eq!(errors.errors[0].code, "invalid-options");
        }
        let errors = Options::from_json("{").unwrap_err();
        assert_eq!(errors.errors[0].code, "invalid-options");
    }
}
