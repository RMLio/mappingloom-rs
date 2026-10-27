use std::error::Error;

use log::info;
use plan::states::Init;
use plan::Plan;
use translator_api::LanguageTranslator;
use translator_new_rml::translator::NewRMLDocumentTranslator;
use translator_rml::OptimizedRMLDocumentTranslator;

/// Translates an RML-KGC document with `translator_new_rml`.
pub fn translate_kgc(
    mapping: &str,
    base_iri: Option<&str>,
) -> Result<Plan<Init>, Box<dyn Error>> {
    info!("Using translator for the latest RML spec https://kg-construct.github.io/rml-resources/portal/");
    let document =
        translator_new_rml::extractors::io::parse_str(mapping, base_iri)
            .map_err(translator_new_rml::error::NewRMLTranslationError::from)?;
    Ok(NewRMLDocumentTranslator::translate_to_plan(document)?)
}

/// Translates an RML.io document with `translator_rml`.
pub fn translate_rml_io(
    mapping: &str,
    base_iri: Option<&str>,
) -> Result<Plan<Init>, Box<dyn Error>> {
    info!("Using translator for the Old RML spec https://rml.io/specs/rml/");
    let mut document =
        translator_rml::parser::extractors::io::parse_str(mapping)?;
    if let Some(base_iri) = base_iri {
        document.default_base_iri = Some(base_iri.to_string());
    }
    Ok(OptimizedRMLDocumentTranslator::translate_to_plan(document)?)
}
