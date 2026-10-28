//! Algorithm 3: the expression of a term map, with the IR's corrections
//! (D10): structured templates with the encoding of their term type, natural
//! literals, datatype and language maps, every term type of RML-KGC, and
//! fresh blank nodes.

use std::collections::BTreeMap;

use algemaploom_plan::{Encoding, Expression, IriKind, Piece, Term};
use oxigraph::model::vocab::xsd;
use oxigraph::model::{NamedNodeRef, NamedOrBlankNodeRef, TermRef};

use crate::mapping::{string, Mapping};
use crate::{vocab, Error, ErrorKind};

/// The attribute that a Source binds to a record's position.
pub const INDEX: &str = "#";

/// The position of a term map, which decides the terms it may give.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Subject,
    Predicate,
    Object,
    Graph,
    Datatype,
    Language,
}

/// What the term maps of one triples map share: the base IRI and the
/// attributes of their queries, which become the fields of the Source
/// (Algorithm 2).
pub struct Context {
    pub base_iri: Option<String>,
    /// Each query with its attribute.
    pub queries: BTreeMap<String, String>,
}

impl Context {
    fn attribute(&mut self, query: &str) -> String {
        self.queries
            .entry(query.to_string())
            .or_insert_with(|| format!("q:{query}"))
            .clone()
    }
}

/// The term map properties of later slices, with the feature each belongs
/// to.
const UNSUPPORTED: &[(NamedNodeRef<'static>, &str)] = &[
    (vocab::PARENT_TRIPLES_MAP, "joins"),
    (vocab::FUNCTION_EXECUTION, "function executions"),
    (vocab::GATHER, "gather maps"),
    (vocab::QUOTED_TRIPLES_MAP, "star maps"),
    (vocab::LOGICAL_TARGET, "logical targets"),
];

/// The term types that give an IRI.
const IRI_TYPES: [NamedNodeRef<'static>; 4] =
    [vocab::IRI, vocab::URI, vocab::UNSAFE_IRI, vocab::UNSAFE_URI];

/// The term types that give an IRI or a blank node.
const NODE_TYPES: [NamedNodeRef<'static>; 5] = [
    vocab::IRI,
    vocab::URI,
    vocab::UNSAFE_IRI,
    vocab::UNSAFE_URI,
    vocab::BLANK_NODE,
];

/// The term types of RML-KGC.
const TERM_TYPES: [NamedNodeRef<'static>; 6] = [
    vocab::IRI,
    vocab::URI,
    vocab::UNSAFE_IRI,
    vocab::UNSAFE_URI,
    vocab::BLANK_NODE,
    vocab::LITERAL,
];

/// The expression of the term map `map` in position `role`.
pub fn expression(
    mapping: &Mapping,
    map: NamedOrBlankNodeRef<'_>,
    role: Role,
    context: &mut Context,
) -> Result<Expression, Error> {
    let error = |kind, code, message: String| {
        Error::new(kind, code, message).at(mapping.origin(map))
    };
    let mapping_error =
        |code, message: String| error(ErrorKind::Mapping, code, message);
    for (property, feature) in UNSUPPORTED {
        if mapping.object(map, *property).is_some() {
            return Err(error(
                ErrorKind::Unsupported,
                "feature",
                format!("{feature} are not translated yet"),
            ));
        }
    }
    let mut values = [vocab::CONSTANT, vocab::REFERENCE, vocab::TEMPLATE]
        .into_iter()
        .flat_map(|property| {
            mapping
                .objects(map, property)
                .into_iter()
                .map(move |value| (property, value))
        });
    let value = values.next();
    if values.next().is_some() {
        return Err(mapping_error(
            "expressions",
            "a term map has one constant, reference or template".into(),
        ));
    }
    let term_type = term_type(mapping, map, role, value)
        .map_err(|(code, message)| mapping_error(code, message))?;

    // An inlined parent subject map (`same-source-joins`) takes its parent's
    // base IRI and fresh blank nodes.
    let parent = mapping.node(map, vocab::NF_PARENT_TRIPLES_MAP);
    let (property, value) = match value {
        Some((property, constant)) if property == vocab::CONSTANT => {
            let term = constant_term(constant, role).map_err(|message| {
                mapping_error("invalid-constant", message)
            })?;
            return Ok(Expression::Constant { term });
        }
        Some(value) => value,
        None if term_type == Some(vocab::BLANK_NODE) => {
            let key = parent
                .and_then(|parent| mapping.node(parent, vocab::SUBJECT_MAP))
                .unwrap_or(map);
            return Ok(Expression::FreshBlankNode {
                key: mapping.origin(key),
                index: vec![INDEX.into()],
            });
        }
        None => {
            return Err(mapping_error(
                "no-expression",
                "a term map needs a constant, a reference or a template".into(),
            ))
        }
    };

    // A reference or a template, which the term type turns into terms.
    let is_reference = property == vocab::REFERENCE;
    let code = if is_reference {
        "invalid-reference"
    } else {
        "invalid-template"
    };
    let text = string(Some(value))
        .ok_or_else(|| mapping_error(code, format!("{value} is no string")))?;
    let inner = Box::new(if is_reference {
        Expression::Reference {
            attribute: context.attribute(&text),
        }
    } else {
        let pieces = parse_template(&text)
            .map_err(|message| mapping_error(code, message))?
            .into_iter()
            .map(|piece| match piece {
                Piece::Attribute(query) => {
                    Piece::Attribute(context.attribute(&query))
                }
                text => text,
            })
            .collect();
        let encoding = match term_type {
            Some(t) if t == vocab::IRI => Encoding::IriSafe,
            Some(t) if t == vocab::URI => Encoding::UriSafe,
            _ => Encoding::None,
        };
        Expression::Template { pieces, encoding }
    });
    Ok(match term_type {
        // A language map gives tags, not terms.
        None => *inner,
        Some(t) if t == vocab::BLANK_NODE => Expression::BlankNode { inner },
        Some(t) if t == vocab::LITERAL => {
            let mut sub_map = |property, role| {
                mapping
                    .node(map, property)
                    .map(|map| expression(mapping, map, role, context))
                    .transpose()
                    .map(|expression| expression.map(Box::new))
            };
            Expression::Literal {
                inner,
                datatype: sub_map(vocab::DATATYPE_MAP, Role::Datatype)?,
                language: sub_map(vocab::LANGUAGE_MAP, Role::Language)?,
            }
        }
        Some(t) => Expression::Iri {
            inner,
            base: match parent {
                Some(parent) => base_iri(mapping, parent),
                None => context.base_iri.clone(),
            },
            kind: if t == vocab::URI || t == vocab::UNSAFE_URI {
                IriKind::Uri
            } else {
                IriKind::Iri
            },
        },
    })
}

/// The term type of a term map, checked against its position, its constant
/// and its datatype and language maps; or an error code and message.
fn term_type<'a>(
    mapping: &'a Mapping,
    map: NamedOrBlankNodeRef<'a>,
    role: Role,
    value: Option<(NamedNodeRef<'_>, TermRef<'_>)>,
) -> Result<Option<NamedNodeRef<'a>>, (&'static str, String)> {
    let invalid = |message: String| ("invalid-term-type", message);
    let term_type = match mapping.object(map, vocab::TERM_TYPE) {
        None => None,
        Some(TermRef::NamedNode(t)) if TERM_TYPES.contains(&t) => Some(t),
        Some(other) => return Err(invalid(format!("{other} is no term type"))),
    };
    let allowed: &[_] = match role {
        Role::Subject | Role::Graph => &NODE_TYPES,
        Role::Predicate | Role::Datatype => &IRI_TYPES,
        Role::Object => &TERM_TYPES,
        Role::Language => &[],
    };
    if let Some(t) = term_type.filter(|t| !allowed.contains(t)) {
        return Err(invalid(format!(
            "a {role:?} map can't have term type {t}"
        )));
    }
    let constant = value
        .filter(|(property, _)| *property == vocab::CONSTANT)
        .map(|(_, constant)| constant);
    match (constant, term_type) {
        (Some(_), Some(t)) if t == vocab::BLANK_NODE => {
            return Err((
                "constant-blank-node",
                "a constant term map can't have term type BlankNode".into(),
            ))
        }
        (Some(TermRef::NamedNode(_)), Some(t)) if !IRI_TYPES.contains(&t) => {
            return Err(invalid(format!("an IRI constant can't be a {t}")))
        }
        (Some(TermRef::Literal(_)), Some(t)) if t != vocab::LITERAL => {
            return Err(invalid(format!("a literal constant can't be a {t}")))
        }
        (None, None) if role != Role::Language => {
            return Err(invalid(format!("a {role:?} map needs a term type")))
        }
        _ => {}
    }
    let datatype = mapping.object(map, vocab::DATATYPE_MAP).is_some();
    let language = mapping.object(map, vocab::LANGUAGE_MAP).is_some();
    if datatype && language {
        return Err((
            "datatype-and-language",
            "a term map can't have both a datatype and a language map".into(),
        ));
    }
    if (datatype || language) && term_type != Some(vocab::LITERAL) {
        return Err((
            "datatype-or-language-without-literal",
            "only a term map with term type Literal has a datatype or \
             language map"
                .into(),
        ));
    }
    Ok(term_type)
}

/// The term of a constant in position `role`, or why it can't stand there.
fn constant_term(constant: TermRef<'_>, role: Role) -> Result<Term, String> {
    match (constant, role) {
        (TermRef::NamedNode(iri), Role::Language) => {
            Err(format!("the language tag {iri} is an IRI"))
        }
        (TermRef::NamedNode(iri), _) => Ok(Term::Iri {
            iri: iri.as_str().into(),
        }),
        (TermRef::Literal(literal), Role::Language) => {
            oxilangtag::LanguageTag::parse(literal.value()).map_err(|err| {
                format!("{} is no language tag: {err}", literal.value())
            })?;
            Ok(Term::Literal {
                value: literal.value().into(),
                datatype: None,
                language: None,
            })
        }
        (TermRef::Literal(literal), Role::Object) => Ok(Term::Literal {
            value: literal.value().into(),
            datatype: (literal.language().is_none()
                && literal.datatype() != xsd::STRING)
                .then(|| literal.datatype().as_str().into()),
            language: literal.language().map(str::to_string),
        }),
        (TermRef::Literal(literal), _) => Err(format!(
            "the literal {literal} can't be a {role:?} constant"
        )),
        (other, _) => Err(format!("{other} can't be a constant")),
    }
}

/// The base IRI of a triples map: its `rml:baseIRI`, which the normalizer
/// gives every triples map when there is a processor's base IRI.
pub fn base_iri(
    mapping: &Mapping,
    triples_map: NamedOrBlankNodeRef<'_>,
) -> Option<String> {
    match mapping.object(triples_map, vocab::BASE_IRI) {
        Some(TermRef::NamedNode(iri)) => Some(iri.as_str().into()),
        _ => None,
    }
}

/// Splits a template into text and the queries of its braces (core
/// `expressions.md:37-41`): `\{`, `\}` and `\\` stand for the character
/// itself; any other backslash, an unmatched brace or an empty query makes
/// the template invalid. `translator_new_rml` splits templates on its own,
/// accepting any escape, and goes with that translator (3.7).
pub fn parse_template(template: &str) -> Result<Vec<Piece>, String> {
    let mut pieces = Vec::new();
    let mut current = String::new();
    let mut in_query = false;
    let mut chars = template.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some(escaped @ ('{' | '}' | '\\')) => current.push(escaped),
                _ => return Err(format!("invalid escape in {template}")),
            },
            '{' if !in_query => {
                if !current.is_empty() {
                    pieces.push(Piece::Text(std::mem::take(&mut current)));
                }
                in_query = true;
            }
            '}' if in_query => {
                if current.is_empty() {
                    return Err(format!("empty reference in {template}"));
                }
                pieces.push(Piece::Attribute(std::mem::take(&mut current)));
                in_query = false;
            }
            '{' | '}' => return Err(format!("unmatched brace in {template}")),
            c => current.push(c),
        }
    }
    if in_query {
        return Err(format!("unmatched brace in {template}"));
    }
    if !current.is_empty() {
        pieces.push(Piece::Text(current));
    }
    Ok(pieces)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(text: &str) -> Piece {
        Piece::Text(text.into())
    }

    fn query(query: &str) -> Piece {
        Piece::Attribute(query.into())
    }

    #[test]
    fn a_template_splits_into_text_and_queries() {
        assert_eq!(
            parse_template("http://ex.com/{ID}/{Name}").unwrap(),
            vec![
                text("http://ex.com/"),
                query("ID"),
                text("/"),
                query("Name")
            ]
        );
    }

    #[test]
    fn escapes_stand_for_the_character() {
        assert_eq!(
            parse_template(r"\{{a\}b}\\").unwrap(),
            vec![text("{"), query("a}b"), text(r"\")]
        );
    }

    #[test]
    fn invalid_templates_are_errors() {
        for template in ["{a", "a}", "{a{b}}", "{}", r"\a", r"a\"] {
            assert!(parse_template(template).is_err(), "{template}");
        }
    }
}
