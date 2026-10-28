//! The canonical RML-KGC translator: it normalizes a mapping and translates
//! the normal form into an `algemaploom-plan` plan, following Algorithms 1–3
//! of Min & Hartig with the corrections of the IR (see HANDBOOK: The
//! canonical pipeline).
//!
//! Each normalized triples map becomes one branch: a Source, an Extend that
//! binds its subject, predicate, object and graph, a Project and a
//! Serializer, which writes to the default Target.

mod mapping;
mod source;
mod term_map;
mod triples_map;
mod vocab;

use std::fmt::Display;

use algemaploom_plan::{Operator, Plan, TargetOp};
use mapping::Mapping;
use normalizer::NormalizerError;

/// What kind of problem an [`Error`] reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// The mapping is not valid Turtle.
    Syntax,
    /// The mapping violates RML-KGC.
    Mapping,
    /// The mapping is valid, but the translator doesn't handle a feature it
    /// uses yet.
    Unsupported,
    /// A bug in the translator or the normalizer.
    Internal,
}

/// A translation error, with a code that is stable within its kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    pub code: &'static str,
    pub message: String,
    /// The node of the original mapping document the error concerns.
    pub origin: Option<String>,
}

impl Error {
    fn new(kind: ErrorKind, code: &'static str, message: String) -> Self {
        Self {
            kind,
            code,
            message,
            origin: None,
        }
    }

    fn at(self, origin: String) -> Self {
        Self {
            origin: Some(origin),
            ..self
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for Error {}

/// Translates an RML-KGC mapping in Turtle. `base_iri` is the processor's
/// base IRI, which every triples map without `rml:baseIRI` takes. Every
/// error found is reported, one or more per triples map.
pub fn translate(
    mapping: &str,
    base_iri: Option<&str>,
) -> Result<Plan, Vec<Error>> {
    let normal_form =
        normalizer::normalize_kgc(mapping, base_iri).map_err(|err| {
            let (kind, code) = match err {
                NormalizerError::RDFParse(_) => {
                    (ErrorKind::Syntax, "invalid-turtle")
                }
                _ => (ErrorKind::Internal, "normalizer"),
            };
            let message = match std::error::Error::source(&err) {
                Some(source) => format!("{err}: {source}"),
                None => err.to_string(),
            };
            vec![Error::new(kind, code, message)]
        })?;
    let mapping = Mapping::new(normal_form);
    let mut branches = Vec::new();
    let mut errors = Vec::new();
    for triples_map in mapping.triples_maps() {
        match triples_map::translate(&mapping, triples_map) {
            Ok(branch) => branches.push(branch),
            Err(found) => errors.extend(found),
        }
    }
    // The normal form has no order, so errors and branches are sorted by
    // content to give the same result on every run.
    if !errors.is_empty() {
        errors.sort_by(|a, b| {
            (&a.origin, a.code, &a.message)
                .cmp(&(&b.origin, b.code, &b.message))
        });
        // The copies of one triples map report the same error once.
        errors.dedup();
        return Err(errors);
    }
    branches.sort_by_cached_key(|branch| format!("{branch:?}"));
    let mut plan = Plan::new();
    let target =
        plan.add(Operator::TargetOp(TargetOp { default: true }), vec![]);
    for branch in branches {
        let serializer = branch.add_to(&mut plan);
        plan.connect(serializer, target);
    }
    Ok(plan)
}
