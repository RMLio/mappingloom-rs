use schemars::JsonSchema;
use serde::Serialize;

use crate::expression::{Expression, Term};

/// An operator of the plan graph.
/// A Source has no input and a Target no output;
/// a Join takes a Left and a Right input,
/// a Union and a Target any number of inputs,
/// and every other operator exactly one.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(tag = "type", content = "config")]
pub enum Operator {
    Source(SourceOp),
    Extend(ExtendOp),
    Project(ProjectOp),
    Rename(RenameOp),
    Join(JoinOp),
    /// The bag union of inputs with equal schemas.
    Union,
    Serializer(SerializerOp),
    Target(TargetOp),
}

/// Yields one tuple per record:
/// `#`, the record's zero-based position in the iteration,
/// and each field bound to the list of its query's values.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct SourceOp {
    pub access: Access,
    /// The IRI of the query language of the iterator and the field references, such as JSONPath;
    /// absent for `OneRecord` access.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,
    /// The values that stand for no value, besides the format's own null.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub null_values: Vec<String>,
    pub fields: Vec<Field>,
}

/// Where a Source reads its records.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Access {
    /// A file, at `path` relative to `root`.
    File { path: String, root: Root },
    /// One record without values.
    OneRecord,
}

/// The directory a relative file path starts from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
pub enum Root {
    MappingDirectory,
    CurrentWorkingDirectory,
}

/// A list field: the attribute bound to the values of a query per record.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct Field {
    pub attribute: String,
    pub reference: String,
    /// Whether the query gives at most one value per record.
    pub single_valued: bool,
}

/// Binds new attributes, each to the list its expression gives on the input tuple.
/// An expression reads input attributes only, never a sibling's.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct ExtendOp {
    pub pairs: Vec<ExtendPair>,
}

#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct ExtendPair {
    pub attribute: String,
    pub expression: Expression,
    /// The mapping nodes the expression comes from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<String>,
}

/// Keeps the listed attributes.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct ProjectOp {
    pub attributes: Vec<String>,
}

/// Renames attributes; the others keep their names.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct RenameOp {
    pub pairs: Vec<RenamePair>,
}

#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct RenamePair {
    pub from: String,
    pub to: String,
}

/// Joins the tuples of its Left and Right inputs, whose schemas are disjoint.
/// A pair holds for a Left tuple t and a Right tuple t′
/// if some value of t's attribute and some value of t′'s have the same lexical form,
/// so an absent attribute never matches;
/// a tuple pair matches if every pair holds,
/// so a Join without pairs gives the cartesian product.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct JoinOp {
    pub kind: JoinKind,
    pub pairs: Vec<JoinPair>,
}

/// What a Join gives for its matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
pub enum JoinKind {
    /// The union t ∪ t′ of every matching tuple pair.
    Inner,
}

/// A Left and a Right attribute whose values a Join compares.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct JoinPair {
    pub left: String,
    pub right: String,
}

/// Per tuple and pattern, one quad per combination of the four positions' lists;
/// an absent position gives none.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct SerializerOp {
    pub patterns: Vec<QuadPattern>,
}

#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct QuadPattern {
    pub subject: Position,
    pub predicate: Position,
    pub object: Position,
    /// Absent for the default graph.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph: Option<Position>,
    /// The mapping nodes the pattern comes from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<String>,
}

/// A position of a quad pattern:
/// an attribute, `{"attribute": …}`,
/// or a constant term, `{"term": …}`.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Position {
    Attribute {
        attribute: String,
    },
    Constant {
        term: Term,
        /// The mapping nodes the constant comes from.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        origin: Vec<String>,
    },
}

/// Writes the set of the quads of its inputs.
/// The default Target takes its destination and serialization from the engine.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct TargetOp {
    pub default: bool,
}
