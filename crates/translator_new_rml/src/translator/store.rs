use std::collections::HashMap;

use plan::data_type::RcRefCellPlan;
use plan::states::{Init, Processed};
use plan::Plan;
use sophia_term::RcTerm;

use super::source::AbstractLogicalSourceTranslator;
use super::OperatorTranslator;
use crate::error::NewRMLTranslationResult;
use crate::rml_model::v2::core::{
    AbstractLogicalSourceEnum, TriplesMap,
};
use crate::rml_model::v2::TermMapEnum;
use crate::rml_model::Document;

#[derive(Debug, Clone, Default)]
pub struct SearchStore<'a> {
    pub root_plan:              Option<Plan<Init>>,
    pub termm_id_quad_var_map:  HashMap<RcTerm, String>,
    pub ls_id_sourced_plan_map: HashMap<u64, RcRefCellPlan<Processed>>,
    /// SubjectMap search map
    pub sm_search_map:          HashMap<RcTerm, &'a TermMapEnum>,

    /// TriplesMap search map
    pub tm_search_map:          HashMap<RcTerm, &'a TriplesMap>,
}

impl SearchStore<'_> {
    /// Returns a vector containing pairs where the left value
    /// is the identifier of the [`AbstractLogicalSource`]
    /// and the right value is a vector of the associated
    /// [`TriplesMap`]'s identifiers.
    ///
    pub fn partition_lsid_tmid(&self) -> Vec<(u64, Vec<RcTerm>)> {
        let mut result: HashMap<u64, Vec<RcTerm>> = HashMap::new();

        for tm in self.tm_search_map.values() {
            let abs_ls_hash = tm.abs_logical_source.effective_equality_hash();
            let value = &tm.identifier;

            result
                .entry(abs_ls_hash)
                .and_modify(|tms| tms.push(value.clone()))
                // RcTerm's cloning (low cost ref counter addition)
                .or_insert(vec![value.clone()]);
        }

        result.into_iter().collect()
    }

    pub fn from_document(
        document: &Document,
    ) -> NewRMLTranslationResult<SearchStore<'_>> {
        let mut tm_search_map = HashMap::new();
        let mut abs_ls_search_map: HashMap<u64, AbstractLogicalSourceEnum> = HashMap::new();
        let mut sm_search_map = HashMap::new();
        let mut termm_id_quad_var_map = HashMap::new();

        for (tm_count, tm) in document.triples_maps.iter().enumerate(){
            // We use the effective equality hash of the logical source as the key in the abs_ls_search_map to
            // ensure that logically equivalent sources are treated as the same source, even if they have different identifiers.
            // If a source with the same effective equality hash already exists in the map,
            // we merge their fields to ensure that all relevant information is retained.
            let source_equality_hash = tm.abs_logical_source.effective_equality_hash();
            abs_ls_search_map
                .entry(source_equality_hash)
                .and_modify(|existing_source|
                    existing_source.merge_fields(&tm.abs_logical_source)
                )
                .or_insert(tm.abs_logical_source.clone());
            
            let tm_id = &tm.identifier;
            tm_search_map.insert(tm_id.clone(), tm);

            let sm = &tm.subject_map;
            let sm_ident = sm.as_ref().identifier.clone();
            sm_search_map.insert(sm_ident.clone(), sm);

            termm_id_quad_var_map
                .insert(sm_ident.clone(), format!("sm_{}", tm_count));

            if sm.is_subject_map() {
                let sm_gms_var_iter = sm
                    .unwrap_subject_map_ref()
                    .graph_maps
                    .iter()
                    .enumerate()
                    .map(|(gm_idx, gm)| {
                        (
                            gm.as_ref().identifier.clone(),
                            format!("sm_{}_gm_{}", tm_count, gm_idx),
                        )
                    });

                termm_id_quad_var_map.extend(sm_gms_var_iter);
            }

            for (pom_idx, pom) in tm.predicate_object_map_vec.iter().enumerate()
            {
                let pom_gms_var_iter = pom.graph_map_vec.iter().enumerate().map(
                    |(gm_idx, gm)| {
                        (
                            gm.as_ref().identifier.clone(),
                            format!(
                                "pom_{}_{}_gm_{}",
                                tm_count, pom_idx, gm_idx
                            ),
                        )
                    },
                );

                termm_id_quad_var_map.extend(pom_gms_var_iter);

                let pm_var_iter = pom.predicate_map_vec.iter().enumerate().map(
                    |(pm_idx, pm)| {
                        (
                            pm.as_ref().identifier.clone(),
                            format!(
                                "pom_{}_{}_pm_{}",
                                tm_count, pom_idx, pm_idx
                            ),
                        )
                    },
                );

                let om_var_iter = pom.object_map_vec.iter().enumerate().map(
                    |(om_idx, om)| {
                        (
                            om.as_ref().identifier.clone(),
                            format!(
                                "pom_{}_{}_om_{}",
                                tm_count, pom_idx, om_idx
                            ),
                        )
                    },
                );

                let pm_om_id_var_chain = pm_var_iter.chain(om_var_iter);
                termm_id_quad_var_map.extend(pm_om_id_var_chain);
            }
        }

        let mut root_plan = Plan::new();
        let ls_id_sourced_plan_map =
            create_ls_id_sourced_plan_map(&mut root_plan, &abs_ls_search_map)?;

        Ok(SearchStore {
            termm_id_quad_var_map,
            sm_search_map,
            tm_search_map,
            ls_id_sourced_plan_map,
            root_plan: Some(root_plan),
        })
    }
}

fn create_ls_id_sourced_plan_map(
    plan: &mut Plan<Init>,
    abs_ls_search_map: &HashMap<u64, AbstractLogicalSourceEnum>,
) -> NewRMLTranslationResult<HashMap<u64, RcRefCellPlan<Processed>>> {
    let mut abs_ls_id_sourced_plan_map = HashMap::new();
    for abs_ls in abs_ls_search_map.values() {
        let source = AbstractLogicalSourceTranslator::translate(abs_ls)?;
        let sourced_plan: RcRefCellPlan<Processed> = plan.source(source).into();

        abs_ls_id_sourced_plan_map.insert(abs_ls.effective_equality_hash(), sourced_plan);
    }
    Ok(abs_ls_id_sourced_plan_map)
}
