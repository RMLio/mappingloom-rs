//! The plans of the canonical pipeline, as a plan document with a schema and
//! a version. They name no mapping language: a translator expresses each
//! language's semantics in their operators and expressions.
//!
//! A [`Plan`] is built as a graph of [`Operator`]s and serialized as a
//! [`PlanDocument`], whose JSON Schema is [`schema`]. A new operator or
//! expression raises the minor [`VERSION`].

mod expression;
mod operator;
mod plan;

pub use expression::{Encoding, Expression, IriKind, Piece, Term};
pub use operator::{
    Access, ExtendOp, ExtendPair, Field, Operator, Position, ProjectOp,
    QuadPattern, RenameOp, RenamePair, Root, SerializerOp, SourceOp, TargetOp,
};
pub use plan::{Edge, Node, NodeIndex, Plan, PlanDocument};

/// The schema version of the plan document, `MAJOR.MINOR`. It changes only
/// when the schema does: a new operator, expression or optional field is a
/// minor bump, and a form that changes meaning or goes away is a major one.
pub const VERSION: &str = "1.0";

/// The crate and version that produce a plan, for diagnostics and for the
/// PROV-O agent.
pub const PRODUCER: &str =
    concat!("algemaploom-rs ", env!("CARGO_PKG_VERSION"));

/// The JSON Schema (2020-12) of the plan document, with an `$id` that names
/// [`VERSION`]. It is committed as `schema/plan.schema.json`.
pub fn schema() -> serde_json::Value {
    let mut schema = schemars::schema_for!(PlanDocument);
    schema.insert(
        "$id".into(),
        format!(
            "https://w3id.org/imec/knows/algemaploom/plan/{VERSION}/plan.schema.json"
        )
        .into(),
    );
    schema.to_value()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// The committed schema is the generated one, so a change to a plan type
    /// shows up as a schema diff. `UPDATE_SCHEMA=1 cargo test` rewrites it.
    #[test]
    fn committed_schema_is_generated() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("schema/plan.schema.json");
        let generated = schema();
        if std::env::var_os("UPDATE_SCHEMA").is_some() {
            let text = serde_json::to_string_pretty(&generated).unwrap();
            std::fs::write(&path, text + "\n").unwrap();
        }
        let committed: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap())
                .unwrap();
        assert_eq!(
            committed, generated,
            "schema/plan.schema.json is outdated: run UPDATE_SCHEMA=1 cargo \
             test -p algemaploom-plan, and bump VERSION if the schema changed"
        );
    }
}
