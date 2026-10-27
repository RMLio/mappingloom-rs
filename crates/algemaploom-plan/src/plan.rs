pub use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use schemars::JsonSchema;
use serde::Serialize;

use crate::operator::Operator;
use crate::{PRODUCER, VERSION};

/// A plan under construction: a directed acyclic graph of operators. The
/// builder adds what it is given and fuses nothing.
#[derive(Debug, Clone, Default)]
pub struct Plan {
    graph: StableDiGraph<(Operator, Vec<String>), ()>,
}

impl Plan {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an operator with its origins in the mapping document.
    pub fn add(
        &mut self,
        operator: Operator,
        origin: Vec<String>,
    ) -> NodeIndex {
        self.graph.add_node((operator, origin))
    }

    /// Makes `from` an input of `to`.
    pub fn connect(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, ());
    }

    /// The plan document, with nodes and edges in the order they were added.
    pub fn document(&self) -> PlanDocument {
        let id = |index: NodeIndex| {
            let (operator, _) = &self.graph[index];
            let name = match operator {
                Operator::SourceOp(_) => "Source",
                Operator::ExtendOp(_) => "Extend",
                Operator::ProjectOp(_) => "Project",
                Operator::RenameOp(_) => "Rename",
                Operator::UnionOp => "Union",
                Operator::SerializerOp(_) => "Serializer",
                Operator::TargetOp(_) => "Target",
            };
            format!("{name}_{}", index.index())
        };
        let nodes = self
            .graph
            .node_indices()
            .map(|index| {
                let (operator, origin) = &self.graph[index];
                Node {
                    id: id(index),
                    operator: operator.clone(),
                    origin: origin.clone(),
                }
            })
            .collect();
        let edges = self
            .graph
            .edge_references()
            .map(|edge| Edge {
                from: id(edge.source()),
                to: id(edge.target()),
            })
            .collect();
        PlanDocument {
            version: VERSION.to_string(),
            producer: PRODUCER.to_string(),
            nodes,
            edges,
        }
    }
}

/// A plan as it travels to an engine.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct PlanDocument {
    /// The schema version, `MAJOR.MINOR`.
    pub version: String,
    pub producer: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct Node {
    /// Unique within the plan; it names the node for its edges only.
    pub id: String,
    pub operator: Operator,
    /// The IRIs of the mapping nodes this operator comes from, such as the
    /// data descriptions that a Source reads. An origin is for provenance
    /// only; it never changes what a plan produces.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<String>,
}

/// An edge from an operator to one that takes its output as input.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct Edge {
    pub from: String,
    pub to: String,
}
