use std::error::Error;

use plan::states::Init;
use plan::Plan;
use translator_api::LanguageTranslator;
use translator_shexml::error::ShExMLTranslationError;
use translator_shexml::parcombi::{self, ShExMLDocument};
use translator_shexml::ShExMLTranslator;

pub fn parse(mapping: &str) -> Result<ShExMLDocument, Box<dyn Error>> {
    Ok(parcombi::parse_string(mapping.to_string())
        .map_err(ShExMLTranslationError::from)?)
}

pub fn translate(
    document: ShExMLDocument,
) -> Result<Plan<Init>, Box<dyn Error>> {
    Ok(ShExMLTranslator::translate_to_plan(document)?)
}
