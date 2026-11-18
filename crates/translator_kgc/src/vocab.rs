//! The IRIs of the RML-KGC vocabulary that the translator reads.

use oxigraph::model::NamedNodeRef;

/// The namespace of RML-KGC.
pub const RML: &str = "http://w3id.org/rml/";

macro_rules! iris {
    ($namespace:literal: $($name:ident = $local:literal),* $(,)?) => {
        $(pub const $name: NamedNodeRef<'static> =
            NamedNodeRef::new_unchecked(concat!($namespace, $local));)*
    };
}

iris! { "http://w3id.org/rml/":
    BASE_IRI = "baseIRI",
    BLANK_NODE = "BlankNode",
    CHILD_MAP = "childMap",
    CONSTANT = "constant",
    CSV = "CSV",
    CURRENT_WORKING_DIRECTORY = "CurrentWorkingDirectory",
    DATATYPE_MAP = "datatypeMap",
    DEFAULT_GRAPH = "defaultGraph",
    GRAPH_MAP = "graphMap",
    IRI = "IRI",
    ITERATOR = "iterator",
    JOIN_CONDITION = "joinCondition",
    JSON_PATH = "JSONPath",
    LANGUAGE_MAP = "languageMap",
    LITERAL = "Literal",
    LOGICAL_SOURCE = "logicalSource",
    MAPPING_DIRECTORY = "MappingDirectory",
    NULL = "null",
    OBJECT_MAP = "objectMap",
    PARENT_MAP = "parentMap",
    PARENT_TRIPLES_MAP = "parentTriplesMap",
    PATH = "path",
    PREDICATE_MAP = "predicateMap",
    PREDICATE_OBJECT_MAP = "predicateObjectMap",
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
