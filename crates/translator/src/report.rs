use std::error::Error;
use std::fmt::Display;

use algemaploom_plan::PlanDocument;
use plan::states::Init;
use plan::Plan;
use serde::Serialize;
use serde_json::{json, Value};

/// What kind of problem a [`Record`] reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// The options are invalid.
    Request,
    /// The mapping text is not valid Turtle or ShExML.
    Syntax,
    /// The mapping violates its specification.
    Mapping,
    /// The mapping is valid, but the translator doesn't handle a feature it
    /// uses yet.
    Unsupported,
    /// A bug in the translator.
    Internal,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Request => "request",
            Kind::Syntax => "syntax",
            Kind::Mapping => "mapping",
            Kind::Unsupported => "unsupported",
            Kind::Internal => "internal",
        }
    }

    /// The exit code of a command line that fails with this kind: 1 for a
    /// mapping that can't be translated, 2 for a request error, 3 for a bug.
    pub fn exit_code(self) -> u8 {
        match self {
            Kind::Request => 2,
            Kind::Internal => 3,
            Kind::Syntax | Kind::Mapping | Kind::Unsupported => 1,
        }
    }
}

/// A line and column in the mapping text, both counted from 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Location {
    pub line: u64,
    pub column: u64,
}

/// An error or a warning of a translation. The codes of each kind are listed
/// in the HANDBOOK (Errors and warnings).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Record {
    pub kind: Kind,
    pub code: &'static str,
    pub message: String,
    /// The mapping node the record concerns, as an IRI or skolem IRI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// The chain of underlying messages, outermost first.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub causes: Vec<String>,
}

impl Record {
    pub fn new(kind: Kind, code: &'static str, message: String) -> Self {
        Self {
            kind,
            code,
            message,
            origin: None,
            location: None,
            causes: Vec::new(),
        }
    }

    /// A record with the message of `err` and the chain of its sources.
    pub fn from_error(kind: Kind, code: &'static str, err: &dyn Error) -> Self {
        let mut causes = Vec::new();
        let mut source = err.source();
        while let Some(inner) = source {
            causes.push(inner.to_string());
            source = inner.source();
        }
        Self {
            causes,
            ..Self::new(kind, code, err.to_string())
        }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}: {}", self.kind.as_str(), self.code, self.message)?;
        if let Some(Location { line, column }) = self.location {
            write!(f, " (line {line}, column {column})")?;
        }
        if let Some(origin) = &self.origin {
            write!(f, " <{origin}>")?;
        }
        for cause in &self.causes {
            write!(f, "\n  caused by: {cause}")?;
        }
        Ok(())
    }
}

/// A successful translation: the plan and what the translator left out or
/// changed without failing.
#[derive(Debug)]
pub struct Translation {
    pub plan: TranslatedPlan,
    pub warnings: Vec<Record>,
}

/// The plan of a legacy translator, or the plan document of the canonical
/// pipeline.
#[derive(Debug)]
pub enum TranslatedPlan {
    Legacy(Plan<Init>),
    Canonical(PlanDocument),
}

impl TranslatedPlan {
    pub fn to_json(&self) -> Value {
        match self {
            Self::Legacy(plan) => {
                serde_json::from_str(&plan.to_json_string().unwrap()).unwrap()
            }
            Self::Canonical(document) => {
                serde_json::to_value(document).unwrap()
            }
        }
    }

    pub fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }

    pub fn to_pretty_json_string(&self) -> String {
        serde_json::to_string_pretty(&self.to_json()).unwrap()
    }
}

/// A failed translation: at least one error, and the warnings found before.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslationErrors {
    pub errors: Vec<Record>,
    pub warnings: Vec<Record>,
}

impl TranslationErrors {
    /// The most severe kind among the errors.
    pub fn kind(&self) -> Kind {
        self.errors.iter().map(|error| error.kind).max().unwrap()
    }
}

impl From<&translator_kgc::Error> for Record {
    fn from(error: &translator_kgc::Error) -> Self {
        let kind = match error.kind {
            translator_kgc::ErrorKind::Syntax => Kind::Syntax,
            translator_kgc::ErrorKind::Mapping => Kind::Mapping,
            translator_kgc::ErrorKind::Unsupported => Kind::Unsupported,
            translator_kgc::ErrorKind::Internal => Kind::Internal,
        };
        Self {
            origin: error.origin.clone(),
            ..Self::new(kind, error.code, error.message.clone())
        }
    }
}

impl From<Record> for TranslationErrors {
    fn from(error: Record) -> Self {
        vec![error].into()
    }
}

impl From<Vec<Record>> for TranslationErrors {
    fn from(errors: Vec<Record>) -> Self {
        Self {
            errors,
            warnings: Vec::new(),
        }
    }
}

/// The errors, one per line.
impl Display for TranslationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let errors: Vec<String> =
            self.errors.iter().map(Record::to_string).collect();
        f.write_str(&errors.join("\n"))
    }
}

/// The result document: the plan and the warnings, or the errors and the
/// warnings, with the version of the plan contract.
pub fn result_document(
    result: &Result<Translation, TranslationErrors>,
) -> Value {
    let (key, value, warnings) = match result {
        Ok(translation) => {
            ("plan", translation.plan.to_json(), &translation.warnings)
        }
        Err(errors) => ("errors", json!(errors.errors), &errors.warnings),
    };
    let mut document = json!({
        "version": algemaploom_plan::VERSION,
        "warnings": warnings,
    });
    document[key] = value;
    document
}
