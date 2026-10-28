use std::any::Any;
use std::panic::{self, AssertUnwindSafe};

use optimizer::Optimizer;
use oxiri::Iri;
use oxttl::TurtleParser;

use crate::report::{
    result_document, Kind, Location, Record, TranslatedPlan, Translation,
    TranslationErrors,
};
use crate::request::{Language, Options, Translator};
use crate::{rml, shexml};

/// Translates a mapping document in the language that `options` names. A
/// panic becomes an internal error, so every caller sees a plan or errors.
pub fn translate(
    mapping: &str,
    options: &Options,
) -> Result<Translation, TranslationErrors> {
    guarded(|| translate_unguarded(mapping, options))
}

fn guarded(
    translate: impl FnOnce() -> Result<Translation, TranslationErrors>,
) -> Result<Translation, TranslationErrors> {
    panic::catch_unwind(AssertUnwindSafe(translate)).unwrap_or_else(|payload| {
        Err(Record::new(Kind::Internal, "panic", panic_message(payload)).into())
    })
}

/// Translates a mapping document with the options object as JSON text, and
/// returns the result document as JSON text: the entry point of the
/// bindings.
pub fn translate_json(mapping: &str, options: &str) -> String {
    let result = Options::from_json(options)
        .and_then(|options| translate(mapping, &options));
    result_document(&result).to_string()
}

fn translate_unguarded(
    mapping: &str,
    options: &Options,
) -> Result<Translation, TranslationErrors> {
    let optimizer = Optimizer::new(&options.optimizer).map_err(|err| {
        Record::new(Kind::Request, "unknown-rule", err.to_string())
    })?;
    if options.translator == Translator::Canonical
        && options.language != Language::RmlKgc
    {
        return Err(Record::new(
            Kind::Request,
            "invalid-options",
            "only rml-kgc has a canonical translator".to_string(),
        )
        .into());
    }
    if let Some(base_iri) = &options.base_iri {
        Iri::parse(base_iri.as_str()).map_err(|err| {
            Record::new(
                Kind::Request,
                "invalid-base-iri",
                format!("the base IRI {base_iri} is no absolute IRI: {err}"),
            )
        })?;
    }

    let base_iri = options.base_iri.as_deref();
    let mut warnings = Vec::new();
    if options.language == Language::Shexml && base_iri.is_some() {
        warnings.push(Record::new(
            Kind::Request,
            "ignored-option",
            "ShExML has no base IRI: base_iri is ignored".to_string(),
        ));
    }
    let invalid_mapping = |err: Box<dyn std::error::Error>| {
        vec![Record::from_error(
            Kind::Mapping,
            "invalid-mapping",
            err.as_ref(),
        )]
    };
    let plan = match options.language {
        Language::RmlKgc if options.translator == Translator::Canonical => {
            check_turtle(mapping).and_then(|document_base| {
                translator_kgc::translate(
                    mapping,
                    base_iri.or(document_base.as_deref()),
                )
                .map(|plan| {
                    TranslatedPlan::Canonical(
                        optimizer.optimize(plan).document(),
                    )
                })
                .map_err(|errors| errors.iter().map(Record::from).collect())
            })
        }
        Language::RmlKgc | Language::RmlIo => {
            let translate_rml = if options.language == Language::RmlKgc {
                rml::translate_kgc
            } else {
                rml::translate_rml_io
            };
            check_turtle(mapping).and_then(|document_base| {
                translate_rml(mapping, base_iri.or(document_base.as_deref()))
                    .map(TranslatedPlan::Legacy)
                    .map_err(invalid_mapping)
            })
        }
        Language::R2rml => Err(vec![Record::new(
            Kind::Unsupported,
            "unsupported-language",
            "no translator reads R2RML's logical tables yet; an RML.io \
             mapping takes the language rml-io"
                .to_string(),
        )]),
        Language::Shexml => shexml::parse(mapping)
            .map_err(|err| {
                vec![Record::from_error(
                    Kind::Syntax,
                    "invalid-shexml",
                    err.as_ref(),
                )]
            })
            .and_then(|document| {
                shexml::translate(document)
                    .map(TranslatedPlan::Legacy)
                    .map_err(invalid_mapping)
            }),
    };
    match plan {
        Ok(plan) => Ok(Translation { plan, warnings }),
        Err(errors) => Err(TranslationErrors { errors, warnings }),
    }
}

/// Reports every Turtle syntax error, with its location, or returns the
/// document's `@base`. The translators resolve the relative IRIs of the
/// document against its `@base` only, so a relative IRI in a document
/// without one is an error here too.
fn check_turtle(mapping: &str) -> Result<Option<String>, Vec<Record>> {
    let mut parser = TurtleParser::new().for_slice(mapping.as_bytes());
    let errors: Vec<Record> = parser
        .by_ref()
        .filter_map(Result::err)
        .map(|err| {
            let start = err.location().start;
            Record {
                location: Some(Location {
                    line: start.line + 1,
                    column: start.column + 1,
                }),
                ..Record::new(
                    Kind::Syntax,
                    "invalid-turtle",
                    err.message().to_string(),
                )
            }
        })
        .collect();
    if errors.is_empty() {
        Ok(parser.base_iri().map(str::to_string))
    } else {
        Err(errors)
    }
}

fn panic_message(payload: Box<dyn Any + Send>) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|message| message.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "the translator panicked".to_string())
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;

    /// An RML-KGC mapping that declares the R2RML namespace, which the
    /// language option keeps on the RML-KGC translator.
    const KGC: &str = r#"
@prefix rml: <http://w3id.org/rml/> .
@prefix rr: <http://www.w3.org/ns/r2rml#> .
@base <http://example.com/base/> .

<TriplesMap1> a rml:TriplesMap ;
  rml:logicalSource [
    rml:source [ a rml:RelativePathSource ; rml:path "student.csv" ] ;
    rml:referenceFormulation rml:CSV
  ] ;
  rml:subjectMap [ rml:template "{Name}" ] ;
  rml:predicateObjectMap [
    rml:predicate <http://xmlns.com/foaf/0.1/name> ;
    rml:objectMap [ rml:reference "Name" ]
  ] .
"#;

    fn plan_json(mapping: &str, options: Value) -> String {
        let options = Options::from_value(options).unwrap();
        translate(mapping, &options).unwrap().plan.to_json_string()
    }

    #[test]
    fn the_language_option_picks_the_translator() {
        let plan = plan_json(KGC, json!({"language": "rml-kgc"}));
        assert!(plan.contains("student.csv"), "{plan}");
        let errors =
            translate(KGC, &Options::new(Language::RmlIo)).unwrap_err();
        assert_eq!(errors.errors[0].kind, Kind::Mapping);
        assert_eq!(errors.errors[0].code, "invalid-mapping");
    }

    #[test]
    fn the_base_iri_option_replaces_the_base_directive() {
        let plan = plan_json(KGC, json!({"language": "rml-kgc"}));
        assert!(plan.contains(r#""base_iri":"http://example.com/base/""#));
        let plan = plan_json(
            KGC,
            json!({"language": "rml-kgc", "base_iri": "http://other.org/"}),
        );
        assert!(plan.contains(r#""base_iri":"http://other.org/""#), "{plan}");
        assert!(!plan.contains("http://example.com/base/"));
    }

    #[test]
    fn a_turtle_error_has_a_location() {
        let mapping = r#"@prefix rml: <http://w3id.org/rml/> .
<http://example.com/a> rml:x "\a" ."#;
        let errors =
            translate(mapping, &Options::new(Language::RmlKgc)).unwrap_err();
        assert_eq!(errors.errors.len(), 1);
        assert_eq!(errors.errors[0].kind, Kind::Syntax);
        assert_eq!(errors.errors[0].code, "invalid-turtle");
        assert_eq!(
            errors.errors[0].location,
            Some(Location {
                line: 2,
                column: 31,
            })
        );
    }

    #[test]
    fn a_relative_iri_without_base_is_a_turtle_error() {
        let mapping = KGC.replace("@base <http://example.com/base/> .", "");
        let errors =
            translate(&mapping, &Options::new(Language::RmlKgc)).unwrap_err();
        assert_eq!(errors.errors[0].code, "invalid-turtle");
        assert_eq!(errors.errors[0].location.unwrap().line, 6);
    }

    #[test]
    fn the_base_iri_option_is_an_absolute_iri() {
        for base_iri in ["", "base/"] {
            let options = Options {
                base_iri: Some(base_iri.to_string()),
                ..Options::new(Language::RmlKgc)
            };
            let errors = translate(KGC, &options).unwrap_err();
            assert_eq!(errors.errors[0].code, "invalid-base-iri");
        }
    }

    #[test]
    fn the_canonical_translator_gives_a_plan_document() {
        let plan: Value = serde_json::from_str(&plan_json(
            KGC,
            json!({"language": "rml-kgc", "translator": "canonical"}),
        ))
        .unwrap();
        assert_eq!(plan["version"], algemaploom_plan::VERSION);
        assert_eq!(plan["nodes"][1]["operator"]["type"], "SourceOp");

        let options = Options {
            translator: Translator::Canonical,
            ..Options::new(Language::RmlIo)
        };
        let errors = translate(KGC, &options).unwrap_err();
        assert_eq!(errors.errors[0].code, "invalid-options");
    }

    #[test]
    fn a_canonical_translation_error_has_its_origin() {
        let mapping = KGC.replace("{Name}", "{Name");
        let options = Options {
            translator: Translator::Canonical,
            ..Options::new(Language::RmlKgc)
        };
        let errors = translate(&mapping, &options).unwrap_err();
        assert_eq!(errors.errors[0].kind, Kind::Mapping);
        assert_eq!(errors.errors[0].code, "invalid-template");
        assert!(errors.errors[0].origin.is_some());
    }

    #[test]
    fn r2rml_is_unsupported() {
        let errors =
            translate(KGC, &Options::new(Language::R2rml)).unwrap_err();
        assert_eq!(errors.errors[0].kind, Kind::Unsupported);
    }

    #[test]
    fn shexml_ignores_the_base_iri_with_a_warning() {
        let options = Options {
            base_iri: Some("http://example.com/".to_string()),
            ..Options::new(Language::Shexml)
        };
        let errors = translate("not ShExML", &options).unwrap_err();
        assert_eq!(errors.errors[0].kind, Kind::Syntax);
        assert_eq!(errors.errors[0].code, "invalid-shexml");
        assert_eq!(errors.warnings[0].code, "ignored-option");
    }

    #[test]
    fn an_unknown_optimizer_rule_is_a_request_error() {
        let result = translate_json(
            KGC,
            r#"{"language": "rml-kgc", "optimizer": {"R9": true}}"#,
        );
        let result: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(result["errors"][0]["kind"], "request");
        assert_eq!(result["errors"][0]["code"], "unknown-rule");
    }

    #[test]
    fn the_result_document_holds_the_plan_or_the_errors() {
        let result: Value = serde_json::from_str(&translate_json(
            KGC,
            r#"{"language": "rml-kgc"}"#,
        ))
        .unwrap();
        assert_eq!(result["version"], algemaploom_plan::VERSION);
        assert!(result["plan"]["nodes"].is_array(), "{result}");
        assert_eq!(result["warnings"], json!([]));
        assert!(result.get("errors").is_none());

        let result: Value =
            serde_json::from_str(&translate_json(KGC, "{}")).unwrap();
        assert_eq!(result["errors"][0]["code"], "invalid-options");
        assert!(result.get("plan").is_none());
    }

    #[test]
    fn a_panic_is_an_internal_error() {
        let errors = guarded(|| panic!("boom")).unwrap_err();
        assert_eq!(
            errors.errors,
            vec![Record::new(Kind::Internal, "panic", "boom".to_string())]
        );
    }
}
