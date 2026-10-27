use std::string::FromUtf8Error;

use oxigraph::io::RdfParseError;
use oxigraph::model::IriParseError;
use oxigraph::sparql::{SparqlSyntaxError, UpdateEvaluationError};
use oxigraph::store::{LoaderError, SerializerError, StorageError};
use thiserror::Error;

pub type NormalizerResult<T> = Result<T, NormalizerError>;

#[derive(Error, Debug)]
pub enum NormalizerError {
    #[error("error while working with file")]
    FileIo(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("error while parsing RDF data")]
    RDFParse(
        #[from]
        #[source]
        RdfParseError,
    ),
    #[error("error while serializing in-memory store")]
    RDFSerializer(
        #[from]
        #[source]
        SerializerError,
    ),
    #[error("error while converting from UTF8 encodings")]
    ParseUtf8(
        #[from]
        #[source]
        FromUtf8Error,
    ),
    #[error("error while storing RDF data with Oxigraph")]
    Storage(
        #[from]
        #[source]
        StorageError,
    ),
    #[error("error while loading RML document into in-memory Oxigraph store")]
    Loading(
        #[from]
        #[source]
        LoaderError,
    ),
    #[error("error while evaluating the SPARQL update queries")]
    Evaluation(
        #[from]
        #[source]
        UpdateEvaluationError,
    ),
    #[error("error while parsing SPARQL update queries")]
    SparqlSyntaxParsing(
        #[from]
        #[source]
        SparqlSyntaxError,
    ),
    #[error("invalid base IRI")]
    BaseIri(
        #[from]
        #[source]
        IriParseError,
    ),
}
