//! Query-driven pruning: keeps the triples maps of a plan whose triples can match a triple
//! pattern of a SPARQL query.
//!
//! # Warning
//!
//! This crate is unmaintained and kept for reference only; nothing in the workspace calls it,
//! and it has no tests. Known defects:
//!
//! - It expects one Extend per attribute (`subject_attr`, `predicate_attr`, `object_attr`,
//!   `graph_attr`). `Plan<Processed>::apply` fuses consecutive Extends, so
//!   [`extract_trmap_sub_expressions_from_plan`] fails on every plan of
//!   `translate_normalized_rml` with `SpecialAttributesIncorrectAmount`.
//! - A triple pattern with a literal object panics on an object map without a datatype.
//! - Pruning drops triples maps that a query needs in these cases: a negated property set is
//!   read as its listed predicates, a sequence path pairs every step with the outer subject and
//!   object, and a template is matched without its base IRI or percent-encoding.
//! - [`serialize_trmap_expr_to_rml`] writes RML.io vocabulary and supports file sources only,
//!   without language tags or blank nodes.

use operator::Operator;

mod checker;
mod extractor;
mod serializer;
mod tests;

fn is_graph(op: &Operator) -> bool {
    match op {
        Operator::ExtendOp { config } => config.extend_pairs.contains_key(GRAPH_ATTR),
        _ => false,
    }
}
pub use checker::maybe_satisfiable_trmaps_sub_expressions;
pub use extractor::rml::{
    extract_trmap_sub_expressions_from_plan, trmap_sub_expression::log_delta_trmap_sub_expressions,
};
pub use extractor::sparql::{
    extract_triple_patterns_from_sparql_file, extract_triple_patterns_from_sparql_str,
};
pub use serializer::util as plan_util;
pub use serializer::{prune_graph_using_trmap_subexprs, reverse_rml::serialize_trmap_expr_to_rml};
use translator_normalized_rml::GRAPH_ATTR;
