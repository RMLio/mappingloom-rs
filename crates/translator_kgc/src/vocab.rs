//! The IRIs of the RML-KGC vocabulary that the translator reads.

use oxigraph::model::NamedNodeRef;

macro_rules! iris {
    ($namespace:literal: $($name:ident = $local:literal),* $(,)?) => {
        $(pub const $name: NamedNodeRef<'static> =
            NamedNodeRef::new_unchecked(concat!($namespace, $local));)*
    };
}

iris! { "http://w3id.org/rml/":
    BASE_IRI = "baseIRI",
    BLANK_NODE = "BlankNode",
    CONSTANT = "constant",
    CSV = "CSV",
    CURRENT_WORKING_DIRECTORY = "CurrentWorkingDirectory",
    DATATYPE_MAP = "datatypeMap",
    DEFAULT_GRAPH = "defaultGraph",
    FUNCTION_EXECUTION = "functionExecution",
    GATHER = "gather",
    GRAPH_MAP = "graphMap",
    IRI = "IRI",
    ITERATOR = "iterator",
    JSON_PATH = "JSONPath",
    LANGUAGE_MAP = "languageMap",
    LITERAL = "Literal",
    LOGICAL_SOURCE = "logicalSource",
    LOGICAL_TARGET = "logicalTarget",
    MAPPING_DIRECTORY = "MappingDirectory",
    NULL = "null",
    OBJECT_MAP = "objectMap",
    PARENT_TRIPLES_MAP = "parentTriplesMap",
    PATH = "path",
    PREDICATE_MAP = "predicateMap",
    PREDICATE_OBJECT_MAP = "predicateObjectMap",
    QUOTED_TRIPLES_MAP = "quotedTriplesMap",
    REFERENCE = "reference",
    REFERENCE_FORMULATION = "referenceFormulation",
    RELATIVE_PATH_SOURCE = "RelativePathSource",
    ROOT = "root",
    SOURCE = "source",
    SUBJECT_MAP = "subjectMap",
    TEMPLATE = "template",
    TERM_TYPE = "termType",
    UNSAFE_IRI = "UnsafeIRI",
    UNSAFE_URI = "UnsafeURI",
    URI = "URI",
    VIEW_ON = "viewOn",
    XPATH = "XPath",
}

pub use normalizer::{
    ORIGIN as NF_ORIGIN, PARENT_TRIPLES_MAP as NF_PARENT_TRIPLES_MAP,
};
