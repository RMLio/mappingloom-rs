use schemars::JsonSchema;
use serde::Serialize;

/// An expression of an Extend: it maps an input tuple to a list of values or
/// terms. Unless stated otherwise, an expression over several lists yields
/// one result per combination of their elements, and nothing if one of them
/// is absent.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Expression {
    /// The list bound to an attribute of the input tuple.
    Reference { attribute: String },
    /// One `xsd:string` value per combination of the values of its
    /// attributes, one value per attribute piece: the pieces concatenated,
    /// each value's lexical form encoded by `encoding`. Absent if any
    /// attribute is absent.
    Template {
        pieces: Vec<Piece>,
        encoding: Encoding,
    },
    /// The one-element list of a complete term.
    Constant { term: Term },
    /// Per value, its lexical form as an IRI if it is an absolute one,
    /// otherwise `base` followed by it; a data error if neither is valid.
    Iri {
        inner: Box<Expression>,
        /// The IRI that a relative value follows; without it, only an
        /// absolute value gives an IRI.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        base: Option<String>,
        kind: IriKind,
    },
    /// Per value, the blank node with its lexical form as identifier.
    BlankNode { inner: Box<Expression> },
    /// One fresh blank node per tuple, identified by `key` and the values of
    /// the `index` attributes. Expressions with the same key give the same
    /// node for the same index values.
    FreshBlankNode {
        key: String,
        /// The attributes that identify the tuple's record, `#` first.
        index: Vec<String>,
    },
    /// Per value, a literal: its natural RDF literal, or one literal per
    /// datatype of `datatype`, or one per language tag of `language`.
    Literal {
        inner: Box<Expression>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        datatype: Option<Box<Expression>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        language: Option<Box<Expression>>,
    },
}

/// A piece of a template: literal text, or an attribute.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Piece {
    Text(String),
    Attribute(String),
}

/// How a template encodes the values of its attributes: IRI-safe (RFC 3987
/// `iunreserved` kept), URI-safe (RFC 3986 `unreserved` kept), or not at
/// all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
pub enum Encoding {
    IriSafe,
    UriSafe,
    None,
}

/// Whether the `Iri` expression checks for an absolute IRI or an absolute
/// URI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
pub enum IriKind {
    Iri,
    Uri,
}

/// A complete RDF term, as a constant gives it.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Term {
    Iri {
        iri: String,
    },
    /// A literal with a datatype IRI or a language tag; with neither, an
    /// `xsd:string`.
    Literal {
        value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        datatype: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        language: Option<String>,
    },
}
