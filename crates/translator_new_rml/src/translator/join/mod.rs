use std::collections::{HashMap, HashSet};

use super::extend::insert_non_constant_func;
use super::store::SearchStore;
use super::StoreOperatorTranslator;
use crate::error::NewRMLTranslationResult;
use crate::rml_model::v2::core::expression_map::BaseExpressionMapEnum;
use crate::rml_model::v2::core::{RefObjectMap, TriplesMap};
use crate::rml_model::v2::{
    AttributeAliaser, RefAttributeGetter, TermMapEnum,
};
use crate::translator::error::TranslationError;
use crate::translator::extend::extend_from_term_map;
use crate::translator::serializer::get_var_or_constant;
use operator::rename::Rename;
use operator::{Extend, Operator, Serializer, Target};
use plan::data_type::RcRefCellPlan;
use plan::states::join::join;
use plan::states::Processed;
use plan::Plan;

#[derive(Debug, Clone)]
pub struct JoinTranslator {}

const PTM_SUBJ_SUFFIX: &str = "_obj";

impl StoreOperatorTranslator for JoinTranslator {
    type Input = TriplesMap;

    type Output = ();

    fn translate_with_store(
        store: &SearchStore,
        child_trip_map: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        let parent_tms_refoms =
            child_trip_map.get_parent_tms_pred_refom_pairs();
        let child_logical_source_equality_hash =
            child_trip_map.abs_logical_source.effective_equality_hash();

        let child_plan_ref = store
            .ls_id_sourced_plan_map
            .get(&child_logical_source_equality_hash)
            .ok_or(TranslationError::JoinError(format!(
                "Search store cannot find the associated plan for the logical source equality hash {child_logical_source_equality_hash}",
            )))?;

        for (parent_tm_id, (pred_vec, ref_om, graph_vec)) in parent_tms_refoms {
            let parent_tm = store.tm_search_map.get(&parent_tm_id).ok_or(
                TranslationError::JoinError(format!(
                    "Given triples map id {:?} does not exist!",
                    parent_tm_id
                )),
            )?;

            let parent_abs_source = &parent_tm.abs_logical_source;
            let mut extended_plan;

            if ref_om.join_condition.is_empty() {
                // This means the logical sources should be effectively equal and thus a self-join.
                // If not, the mapping is wrong.
                let parent_logical_source_hash = parent_abs_source.effective_equality_hash();
                let parent_plan_ref = store
                    .ls_id_sourced_plan_map
                    .get(&parent_logical_source_hash)
                    .ok_or(TranslationError::JoinError(format!(
                        "Search store cannot find the associated plan for the logical source id: {:?}",
                        parent_logical_source_hash
                    )))?;

                let extend_op = extend_op_from_join(
                    &child_trip_map.subject_map,
                    &ref_om,
                    &pred_vec,
                    &child_trip_map.base_iri,
                    &graph_vec,
                    None,
                    store,
                )?;

                let mut child_plan = child_plan_ref.borrow_mut();

                if let Ok(parent_plan) = parent_plan_ref.try_borrow() {
                    // TODO: this code probably never gets executed. Test!
                    // This means parent and child plans are different.
                    // If they are the same, try_borrow causes an error.
                    child_plan.merge_source(&parent_plan.get_first_source().unwrap())?;
                }
                extended_plan = child_plan.apply(&extend_op, "Extend")?;
            } else {
                extended_plan = join_related_plan_processing(
                    store,
                    child_trip_map,
                    child_logical_source_equality_hash,
                    child_plan_ref,
                    &pred_vec,
                    &ref_om,
                    &graph_vec,
                    parent_tm,
                )?;
            }
            let serializer = Serializer {
                template: serializer_template_from_join(
                    &child_trip_map.subject_map,
                    &pred_vec,
                    &ref_om,
                    &graph_vec,
                    store,
                ),
                options:  None,
                format:   operator::formats::DataFormat::NQuads,
            };

            extended_plan
                .serialize(serializer)?
                .sink(&Target::default())?;
        }
        Ok(())
    }
}

fn join_related_plan_processing(
    store: &SearchStore<'_>,
    child_trip_map: &TriplesMap,
    child_logical_source_hash: u64,
    child_plan: &RcRefCellPlan<Processed>,
    pred_vec: &[TermMapEnum],
    ref_om: &RefObjectMap,
    graph_vec: &[TermMapEnum],
    parent_tm: &&TriplesMap,
) -> Result<Plan<Processed>, crate::error::NewRMLTranslationError> {
    let parent_logical_source_hash =
        parent_tm.abs_logical_source.effective_equality_hash();
    let parent_plan = store
        .ls_id_sourced_plan_map
        .get(&parent_logical_source_hash)
        .ok_or(TranslationError::JoinError(format!(
            "Search store cannot found the associated plan for the logical source with hash: {:?}",
            parent_logical_source_hash
        )))?;
    let alias = "join_alias";
    let mut aliased_plan =
        join(child_plan.clone(), parent_plan.clone())?.alias(alias)?;
    let mut joined: Plan<Processed>;
    let join_conditions = &ref_om.join_condition;
    let child_attributes: Vec<_> = join_conditions
        .iter()
        .flat_map(|jc| jc.child.get_ref_attributes())
        .collect();
    let parent_attributes: Vec<_> = join_conditions
        .iter()
        .flat_map(|jc| jc.parent.get_ref_attributes())
        .map(|val| format!("{}.{}", alias, val))
        .collect();
    let extend_op;
    if (!child_attributes.is_empty() && !parent_attributes.is_empty())
        || child_logical_source_hash != parent_logical_source_hash
    {
        let ptm_rename_op = Rename {
            alias:        Some(alias.to_string()),
            rename_pairs: HashMap::new(),
        };

        aliased_plan = aliased_plan.apply_to_right(
            Operator::RenameOp {
                config: ptm_rename_op,
            },
            "RenameOp".into(),
        )?;

        joined = aliased_plan
            .where_by(child_attributes)?
            .equal_to(parent_attributes)?;
        extend_op = extend_op_from_join(
            &child_trip_map.subject_map,
            ref_om,
            pred_vec,
            &child_trip_map.base_iri,
            graph_vec,
            Some(alias),
            store,
        )?;
    } else {
        joined = aliased_plan.natural_join()?;

        extend_op = extend_op_from_join(
            &child_trip_map.subject_map,
            ref_om,
            pred_vec,
            &child_trip_map.base_iri,
            graph_vec,
            None,
            store,
        )?;
    }
    Ok(joined.apply(&extend_op, "ExtendOp")?)
}

pub fn extend_op_from_join(
    child_subj_map: &TermMapEnum,
    ref_objmap: &RefObjectMap,
    pred_vec: &[TermMapEnum],
    child_base_iri: &str,
    graph_maps: &[TermMapEnum],
    alias_opt: Option<&str>,
    store: &SearchStore,
) -> NewRMLTranslationResult<Operator> {
    let (subj_var, subj_func) =
        extend_from_term_map(store, child_base_iri, child_subj_map.as_ref())?;

    let extension_func_predicates_res: NewRMLTranslationResult<Vec<_>> =
        pred_vec
            .iter()
            .map(|pm| extend_from_term_map(store, child_base_iri, pm.as_ref()))
            .collect();
    let extension_func_predicates = extension_func_predicates_res?;

    let extension_func_graphs_res: NewRMLTranslationResult<Vec<_>> = graph_maps
        .iter()
        .map(|gm| extend_from_term_map(store, child_base_iri, gm.as_ref()))
        .collect();
    let extension_func_graphs = extension_func_graphs_res?;
    log::debug!(
        "Subject map varible search map: {:#?}",
        store.termm_id_quad_var_map
    );

    let ptm = store.tm_search_map.get(&ref_objmap.ptm_iri).ok_or(
        TranslationError::JoinError(
            "Reference object map's parent triples maps IRI cannot be found/searched"
                .to_string(),
        ),
    )?;
    log::debug!(
        "Before alias parent triples map's subject term map is {:#?}",
        ptm.subject_map
    );
    let aliased_ptm_subj_term_map = match alias_opt {
        Some(alias) => ptm.subject_map.as_ref().alias_attribute(alias),
        None => ptm.subject_map.as_ref().clone(),
    };
    log::debug!(
        "Aliased parent triples map's subject term map is {:#?}",
        aliased_ptm_subj_term_map
    );
    let (mut ptm_subj_var, ptm_subj_func) =
        extend_from_term_map(store, &ptm.base_iri, &aliased_ptm_subj_term_map)?;
    // Require renaming variable to ensure there is no conflict when both
    // child and parent triples maps are the same
    ptm_subj_var = format!("{}{}", ptm_subj_var, PTM_SUBJ_SUFFIX);

    let mut extend_pairs = HashMap::new();

    insert_non_constant_func(&mut extend_pairs, subj_var, subj_func);
    insert_non_constant_func(&mut extend_pairs, ptm_subj_var, ptm_subj_func);

    extension_func_predicates
        .into_iter()
        .for_each(|(var, func)| {
            insert_non_constant_func(&mut extend_pairs, var, func);
        });

    extension_func_graphs.into_iter().for_each(|(var, func)| {
        insert_non_constant_func(&mut extend_pairs, var, func);
    });

    let extend = Extend { extend_pairs };
    Ok(Operator::ExtendOp { config: extend })
}

pub fn serializer_template_from_join(
    subj_map: &TermMapEnum,
    pred_vec: &[TermMapEnum],
    ref_objmap: &RefObjectMap,
    graph_vec: &[TermMapEnum],
    store: &SearchStore,
) -> String {
    let subj_pattern = get_var_or_constant(store, subj_map.as_ref());
    let pred_patterns = pred_vec
        .iter()
        .map(|pm| get_var_or_constant(store, pm.as_ref()));

    let ptm = store.tm_search_map.get(&ref_objmap.ptm_iri).unwrap();
    let ptm_sm = store
        .sm_search_map
        .get(&ptm.subject_map.as_ref().identifier)
        .unwrap();
    let ptm_tm_info = &ptm_sm
        .try_unwrap_subject_map_ref()
        .unwrap()
        .term_map_info
        .expression;

    let mut ptm_sm_var = get_var_or_constant(store, ptm_sm.as_ref());
    // Only prefix the variable of the subject map of parent triples map's
    // if it is not a constant-valued term map of a blank node term type without expression map.
    if let Some(ptm_tm_info) = ptm_tm_info {
        match ptm_tm_info.try_unwrap_base_expression_map_ref() {
            Ok(BaseExpressionMapEnum::Constant(_)) => {}
            _ => {
                ptm_sm_var = format!("{}{}", ptm_sm_var, PTM_SUBJ_SUFFIX);
            }
        }
    }


    let mut statement_patterns: HashSet<_> = pred_patterns
        .map(|pred| format!("{} {} {}", subj_pattern, pred, ptm_sm_var))
        .collect();

    let graph_patterns: Vec<_> = graph_vec
        .iter()
        .map(|gm| get_var_or_constant(store, gm.as_ref()))
        .collect();
    if !graph_patterns.is_empty() {
        statement_patterns = graph_patterns
            .into_iter()
            .flat_map(|graph_var| {
                statement_patterns
                    .iter()
                    .map(move |pattern| format!("{} {}", pattern, graph_var))
            })
            .collect();
    }

    let statement_patterns: Vec<_> = statement_patterns
        .iter()
        .map(|pattern| format!("{} .", pattern))
        .collect();

    statement_patterns.join("\n")
}
