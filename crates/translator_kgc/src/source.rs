//! The Source of a triples map: its logical source and the fields of its
//! queries (Algorithm 2).

use std::collections::BTreeMap;

use algemaploom_plan::{Access, Field, Root, SourceOp};
use oxigraph::model::vocab::rdf;
use oxigraph::model::{NamedNodeRef, NamedOrBlankNodeRef, TermRef};

use crate::mapping::{string, Mapping};
use crate::{vocab, Error, ErrorKind};

/// The Source that reads the logical source of `triples_map`, with one
/// field per query. A triples map without logical source reads one record
/// (K7).
pub fn source(
    mapping: &Mapping,
    triples_map: NamedOrBlankNodeRef<'_>,
    queries: &BTreeMap<String, String>,
) -> Result<(SourceOp, Vec<String>), Error> {
    let Some(logical_source) = mapping.node(triples_map, vocab::LOGICAL_SOURCE)
    else {
        if let Some(query) = queries.keys().next() {
            return Err(Error::new(
                ErrorKind::Mapping,
                "reference-without-logical-source",
                format!("{query} reads no logical source"),
            )
            .at(mapping.origin(triples_map)));
        }
        let source = SourceOp {
            access: Access::OneRecord,
            query_language: None,
            iterator: None,
            null_values: vec![],
            fields: vec![],
        };
        return Ok((source, vec![]));
    };
    let origin = mapping.origin(logical_source);
    let unsupported = |message: String| {
        Error::new(ErrorKind::Unsupported, "source", message).at(origin.clone())
    };
    if mapping.object(logical_source, vocab::VIEW_ON).is_some() {
        return Err(unsupported("logical views are not translated yet".into()));
    }
    only_known_properties(
        mapping,
        logical_source,
        &[vocab::SOURCE, vocab::REFERENCE_FORMULATION, vocab::ITERATOR],
    )
    .map_err(unsupported)?;
    let source = mapping
        .node(logical_source, vocab::SOURCE)
        .filter(|source| {
            mapping
                .objects(*source, rdf::TYPE)
                .contains(&vocab::RELATIVE_PATH_SOURCE.into())
        })
        .ok_or_else(|| {
            unsupported("only a relative path source is translated yet".into())
        })?;
    only_known_properties(
        mapping,
        source,
        &[vocab::PATH, vocab::ROOT, vocab::NULL],
    )
    .map_err(unsupported)?;
    let path =
        string(mapping.object(source, vocab::PATH)).ok_or_else(|| {
            unsupported("a relative path source has a path".into())
        })?;
    let root = match mapping.object(source, vocab::ROOT) {
        Some(root) if root == vocab::MAPPING_DIRECTORY.into() => {
            Root::MappingDirectory
        }
        // rml-io's default root is the current working directory.
        None => Root::CurrentWorkingDirectory,
        Some(root) if root == vocab::CURRENT_WORKING_DIRECTORY.into() => {
            Root::CurrentWorkingDirectory
        }
        Some(root) => return Err(unsupported(format!("unknown root {root}"))),
    };
    let query_language = match mapping
        .object(logical_source, vocab::REFERENCE_FORMULATION)
    {
        Some(TermRef::NamedNode(language))
            if [vocab::CSV, vocab::JSON_PATH, vocab::XPATH]
                .contains(&language) =>
        {
            language
        }
        other => {
            return Err(unsupported(format!("reference formulation {other:?}")))
        }
    };
    let mut null_values: Vec<String> = mapping
        .objects(source, vocab::NULL)
        .into_iter()
        .filter_map(|null| string(Some(null)))
        .collect();
    null_values.sort();
    let fields = queries
        .iter()
        .map(|(query, attribute)| Field {
            attribute: attribute.clone(),
            reference: query.clone(),
            single_valued: single_valued(query_language, query),
        })
        .collect();
    let source = SourceOp {
        access: Access::File { path, root },
        query_language: Some(query_language.as_str().into()),
        iterator: string(mapping.object(logical_source, vocab::ITERATOR)),
        null_values,
        fields,
    };
    Ok((source, vec![origin]))
}

/// Whether a query gives at most one value per record: a CSV column, or a
/// JSONPath without wildcard, recursive descent, filter, union or slice.
/// An XPath may select several nodes.
fn single_valued(query_language: NamedNodeRef<'_>, query: &str) -> bool {
    if query_language == vocab::CSV {
        true
    } else if query_language == vocab::JSON_PATH {
        !["*", "..", "?", ",", ":"]
            .iter()
            .any(|part| query.contains(part))
    } else {
        false
    }
}

/// Fails on a property that the translator doesn't read, so that none is
/// dropped silently.
fn only_known_properties(
    mapping: &Mapping,
    node: NamedOrBlankNodeRef<'_>,
    known: &[NamedNodeRef<'_>],
) -> Result<(), String> {
    match mapping
        .predicates(node)
        .into_iter()
        .find(|p| *p != rdf::TYPE && !known.contains(p))
    {
        Some(property) => {
            Err(format!("the property {property} is not translated yet"))
        }
        None => Ok(()),
    }
}
