//! The optimizer of the canonical pipeline: independent rewrite rules on an
//! `algemaploom-plan` plan, each with its own toggle.
//!
//! A rule keeps the plan's output equal: it is applied only where its
//! preconditions hold. The rules run in the order of [`rules`], each once.

use std::collections::BTreeMap;
use std::fmt::Display;

use algemaploom_plan::Plan;

/// A rewrite rule of the optimizer.
pub trait Rule {
    /// The rule's id, the key of its toggle (e.g. `R1`).
    fn id(&self) -> &'static str;

    /// Whether the rule runs when its toggle is absent.
    fn enabled_by_default(&self) -> bool;

    /// Rewrites the plan into an equivalent one.
    fn apply(&self, plan: Plan) -> Plan;
}

/// Every rule of the optimizer, in the order they run.
pub fn rules() -> Vec<Box<dyn Rule>> {
    Vec::new()
}

/// A toggle that names no rule of the optimizer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownRule(pub String);

impl Display for UnknownRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the optimizer has no rule {}", self.0)
    }
}

impl std::error::Error for UnknownRule {}

/// The rules that are switched on, in the order they run.
pub struct Optimizer {
    rules: Vec<Box<dyn Rule>>,
}

impl Optimizer {
    /// The optimizer with the rules of [`rules`], each switched on or off by
    /// its toggle, or by its default when it has none.
    pub fn new(toggles: &BTreeMap<String, bool>) -> Result<Self, UnknownRule> {
        Self::with_rules(rules(), toggles)
    }

    fn with_rules(
        rules: Vec<Box<dyn Rule>>,
        toggles: &BTreeMap<String, bool>,
    ) -> Result<Self, UnknownRule> {
        if let Some(unknown) = toggles
            .keys()
            .find(|id| !rules.iter().any(|rule| rule.id() == id.as_str()))
        {
            return Err(UnknownRule(unknown.clone()));
        }
        let rules = rules
            .into_iter()
            .filter(|rule| {
                toggles
                    .get(rule.id())
                    .copied()
                    .unwrap_or_else(|| rule.enabled_by_default())
            })
            .collect();
        Ok(Self { rules })
    }

    pub fn optimize(&self, plan: Plan) -> Plan {
        self.rules.iter().fold(plan, |plan, rule| rule.apply(plan))
    }
}

#[cfg(test)]
mod tests {
    use algemaploom_plan::Operator;

    use super::*;

    /// Adds a Union, so its effect shows in the plan document.
    struct AddUnion(&'static str, bool);

    impl Rule for AddUnion {
        fn id(&self) -> &'static str {
            self.0
        }

        fn enabled_by_default(&self) -> bool {
            self.1
        }

        fn apply(&self, mut plan: Plan) -> Plan {
            plan.add(Operator::UnionOp, vec![self.0.to_string()]);
            plan
        }
    }

    fn applied(toggles: &[(&str, bool)]) -> Vec<Vec<String>> {
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(AddUnion("on", true)),
            Box::new(AddUnion("off", false)),
        ];
        let toggles = toggles
            .iter()
            .map(|(id, on)| (id.to_string(), *on))
            .collect();
        let optimizer = Optimizer::with_rules(rules, &toggles).unwrap();
        let document = optimizer.optimize(Plan::new()).document();
        document.nodes.into_iter().map(|node| node.origin).collect()
    }

    #[test]
    fn without_rules_a_plan_passes_unchanged() {
        let mut plan = Plan::new();
        plan.add(Operator::UnionOp, vec![]);
        let optimizer = Optimizer::new(&BTreeMap::new()).unwrap();
        assert_eq!(
            optimizer.optimize(plan.clone()).document(),
            plan.document()
        );
    }

    #[test]
    fn a_toggle_overrides_the_default() {
        assert_eq!(applied(&[]), vec![vec!["on".to_string()]]);
        assert_eq!(
            applied(&[("on", false), ("off", true)]),
            vec![vec!["off".to_string()]]
        );
    }

    #[test]
    fn an_unknown_rule_is_an_error() {
        let toggles = BTreeMap::from([("R9".to_string(), true)]);
        assert_eq!(
            Optimizer::new(&toggles).err(),
            Some(UnknownRule("R9".to_string()))
        );
    }
}
