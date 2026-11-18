//! The plans of the canonical pipeline, as a plan document with a schema and a version.
//! They name no mapping language:
//! a translator expresses each language's semantics in their operators and expressions.
//!
//! A [`Plan`] is built as a graph of [`Operator`]s and serialized as a [`PlanDocument`],
//! whose JSON Schema is [`schema`].

mod expression;
mod operator;
mod plan;

pub use expression::{Encoding, Expression, Piece, Term};
pub use operator::{
    Access, ExtendOp, ExtendPair, Field, JoinKind, JoinOp, JoinPair, Operator,
    Position, ProjectOp, QuadPattern, RenameOp, RenamePair, Root, SerializerOp,
    SourceOp, TargetOp,
};
pub use plan::{Edge, InputRole, Node, NodeIndex, Plan, PlanDocument};

/// The version of the plan document: the version of algemaploom-rs, whose plans are internal to it for now.
/// From algemaploom-rs 1.0 on, the plans may get a version of their own.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The JSON Schema (2020-12) of the plan document.
/// It is committed as `schema/plan.schema.json`.
pub fn schema() -> serde_json::Value {
    let mut schema = schemars::schema_for!(PlanDocument);
    schema.insert(
        "$id".into(),
        "https://w3id.org/imec/knows/algemaploom/plan/plan.schema.json".into(),
    );
    schema.to_value()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// The committed schema is the generated one,
    /// so a change to a plan type shows up as a schema diff.
    /// `UPDATE_SCHEMA=1 cargo test` rewrites it.
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
