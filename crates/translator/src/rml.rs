use log::info;
use plan::states::Init;
use plan::Plan;
use translator_api::LanguageTranslator;
use translator_new_rml::error::NewRMLTranslationError;
use translator_new_rml::translator::NewRMLDocumentTranslator;
use translator_rml::parser::extractors::io::parse_str as old_parse_str;
use translator_rml::OptimizedRMLDocumentTranslator;

use crate::error::TranslationError;
use crate::handler::TranslatorHandler;

#[derive(Debug)]
pub struct RMLHandler;

impl TranslatorHandler for RMLHandler {
    fn translate(&self, mapping: &str) -> Result<Plan<Init>, TranslationError> {
        // TODO: Needs a better way to detect old vs new RML mapping document <16-04-25, Min Oo> //

        if mapping.contains("<http://www.w3.org/ns/r2rml#>") {
            // Old RML mapping document

            info!("Using translator for the Old RML spec https://rml.io/specs/rml/");
            let document = old_parse_str(mapping)?;
            Ok(OptimizedRMLDocumentTranslator::translate_to_plan(document)?)
        } else {
            // New RML mapping document shouldn't contain R2RML's prefix (BIIIGG ASSUMPTION)
            info!("Using translator for the latest RML spec https://kg-construct.github.io/rml-resources/portal/");
            let document =
                translator_new_rml::extractors::io::parse_str(mapping)
                    .map_err::<NewRMLTranslationError, _>(|err| err.into())?;

            Ok(NewRMLDocumentTranslator::translate_to_plan(document)?)
        }
    }
}
