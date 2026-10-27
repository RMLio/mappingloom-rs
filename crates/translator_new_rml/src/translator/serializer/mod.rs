use std::collections::HashSet;
use std::marker::{PhantomData, PhantomPinned};

use operator::Serializer;
use vocab::ToString;

use super::store::SearchStore;
use super::StoreOperatorTranslator;
use crate::error::NewRMLTranslationResult;
use crate::extractors::stringify_term;
use crate::rml_model::v2::core::expression_map::term_map::CommonTermMapInfo;
use crate::rml_model::v2::core::{PredicateObjectMap, TriplesMap};
use crate::rml_model::v2::TermMapEnum;

#[derive(Debug, Clone)]
pub struct SerializerOperatorTranslator<'a> {
    _phantom: PhantomData<&'a PhantomPinned>,
}

fn format_var(var: &str) -> String {
    format!("?{}", var)
}

impl<'a> StoreOperatorTranslator for SerializerOperatorTranslator<'a> {
    type Input = Vec<&'a TriplesMap>;

    type Output = Serializer;

    fn translate_with_store(
        store: &SearchStore,
        tm_vec: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        let mut graph_pattern: HashSet<String> = HashSet::new();
        for tm in tm_vec {
            let sm_var = store
                .termm_id_quad_var_map
                .get(&tm.subject_map.as_ref().identifier)
                .map(|var| format_var(var))
                .unwrap();

            let sm = tm
                .subject_map
                .as_ref()
                .get_constant_value()
                .unwrap_or_else(|| sm_var.to_string());

            let sm_graphs = tm
                .subject_map
                .try_unwrap_subject_map_ref()
                .map(|sm| graph_suffixes(store, &sm.graph_maps))
                .unwrap_or_default();

            for pom in &tm.predicate_object_map_vec {
                // TODO: Handles reference object maps too <15-04-25, Min Oo> //

                let triples: Vec<String> = cproduct_pm_om_vars(store, pom)
                    .map(|(pm_var, om_var)| {
                        format!("{} {} {}", sm, pm_var, om_var)
                    })
                    .collect();

                // Reference object maps with their own graph maps are handled
                // by the join translator, which leaves no triples here.
                let mut graphs = sm_graphs.clone();
                graphs.extend(graph_suffixes(store, &pom.graph_map_vec));
                add_triples_to_graphs(&mut graph_pattern, &triples, graphs);
            }

            let class_triples = tm
                .subject_map
                .unwrap_subject_map_ref()
                .classes
                .iter()
                .map(|class_iri| {
                    format!(
                        "{} <{}> <{}>",
                        sm,
                        vocab::rdf::property::TYPE.to_string(),
                        stringify_term(class_iri).unwrap()
                    )
                })
                .collect::<Vec<_>>();
            add_triples_to_graphs(
                &mut graph_pattern,
                &class_triples,
                sm_graphs,
            );
        }

        Ok(Serializer {
            template: graph_pattern.into_iter().collect::<Vec<_>>().join("\n"),
            options: None,
            format: operator::formats::DataFormat::NQuads,
        })
    }
}

/// The suffixes that put a triple into each of the given graph maps; the
/// default graph is the suffix without a graph term.
fn graph_suffixes(
    store: &SearchStore,
    graph_map_vec: &[TermMapEnum],
) -> Vec<String> {
    graph_map_vec
        .iter()
        .map(|gm_enum| {
            let gm = gm_enum.unwrap_graph_map_ref();
            if gm.is_default_graph() {
                return " .".to_string();
            }
            let gm_part =
                gm.term_map_info.get_constant_value().unwrap_or_else(|| {
                    store
                        .termm_id_quad_var_map
                        .get(&gm_enum.as_ref().identifier)
                        .map(|var| format_var(var))
                        .unwrap()
                });
            format!(" {} .", gm_part)
        })
        .collect()
}

/// Puts the triples into each of the graphs, or into the default graph when
/// there are none.
fn add_triples_to_graphs(
    graph_pattern: &mut HashSet<String>,
    triples: &[String],
    mut graphs: Vec<String>,
) {
    if graphs.is_empty() {
        graphs.push(" .".to_string());
    }
    for suffix in &graphs {
        for triple in triples {
            graph_pattern.insert(format!("{}{}", triple, suffix));
        }
    }
}

fn cproduct_pm_om_vars<'a>(
    store: &'a SearchStore,
    pom: &'a PredicateObjectMap,
) -> impl Iterator<Item = (String, String)> + 'a {
    let pm_var_iter = pom
        .predicate_map_vec
        .iter()
        .map(|pm| get_var_or_constant(store, pm.as_ref()));

    let om_var_iter = pom
        .object_map_vec
        .iter()
        .map(|om| get_var_or_constant(store, om.as_ref()));

    pm_var_iter.flat_map(move |pm_var| {
        om_var_iter
            .clone()
            .map(move |om_var| (pm_var.clone(), om_var))
    })
}

pub fn get_var_or_constant(
    store: &SearchStore<'_>,
    term_map_info: &CommonTermMapInfo,
) -> String {
    let var = store
        .termm_id_quad_var_map
        .get(&term_map_info.identifier)
        .map(|var| format_var(var))
        .unwrap();

    term_map_info
        .get_constant_value()
        .unwrap_or_else(|| var.to_string())
}

#[cfg(test)]
mod tests {
    use translator_api::LanguageTranslator;

    use crate::extractors::io::parse_str;
    use crate::translator::NewRMLDocumentTranslator;

    const NAME_POM: &str = r#"rml:predicateObjectMap [
        rml:predicate ex:name; rml:objectMap [ rml:constant "N" ] ]"#;
    const AGE_POM: &str = r#"rml:predicateObjectMap [
        rml:predicate ex:age; rml:objectMap [ rml:constant "A" ];
        rml:graphMap [ rml:constant ex:G1 ] ]"#;

    /// The sorted serializer template lines of a triples map with the given
    /// subject map graph map (or none) and predicate-object maps.
    fn template_lines(subject_graph: &str, poms: [&str; 2]) -> Vec<String> {
        let mapping = format!(
            r#"@prefix ex: <http://example.com/> .
@prefix rml: <http://w3id.org/rml/> .
<http://example.com/TM> a rml:TriplesMap;
  rml:logicalSource [ a rml:LogicalSource;
      rml:iterator "$.students[*]";
      rml:referenceFormulation rml:JSONPath;
      rml:source [ a rml:RelativePathSource;
          rml:root rml:MappingDirectory; rml:path "student.json" ] ];
  rml:subjectMap [ rml:template "http://example.com/{{$.ID}}" {subject_graph} ];
  {}; {} ."#,
            poms[0], poms[1]
        );
        plan_template_lines(&mapping)
    }

    /// The sorted serializer template lines of the plan of the mapping.
    fn plan_template_lines(mapping: &str) -> Vec<String> {
        let plan = NewRMLDocumentTranslator::translate_to_plan(
            parse_str(mapping, None).unwrap(),
        )
        .unwrap();
        let json: serde_json::Value =
            serde_json::from_str(&plan.to_json_string().unwrap()).unwrap();
        let mut lines = vec![];
        collect_templates(&json, &mut lines);
        lines.sort();
        lines
    }

    fn collect_templates(json: &serde_json::Value, lines: &mut Vec<String>) {
        match json {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    match (key.as_str(), value) {
                        ("template", serde_json::Value::String(template)) => {
                            lines.extend(template.lines().map(String::from));
                        }
                        _ => collect_templates(value, lines),
                    }
                }
            }
            serde_json::Value::Array(vec) => {
                vec.iter().for_each(|v| collect_templates(v, lines))
            }
            _ => {}
        }
    }

    const AGE_G1: &str =
        r#"?sm_0 <http://example.com/age> "A" <http://example.com/G1> ."#;
    const NAME_DEFAULT: &str = r#"?sm_0 <http://example.com/name> "N" ."#;

    #[test]
    fn graph_map_of_a_later_pom_leaves_earlier_triples_in_default_graph() {
        assert_eq!(
            template_lines("", [NAME_POM, AGE_POM]),
            [AGE_G1, NAME_DEFAULT]
        );
    }

    #[test]
    fn graph_map_of_an_earlier_pom_leaves_later_triples_in_default_graph() {
        assert_eq!(
            template_lines("", [AGE_POM, NAME_POM]),
            [AGE_G1, NAME_DEFAULT]
        );
    }

    #[test]
    fn subject_graph_map_adds_to_the_graph_maps_of_each_pom() {
        assert_eq!(
            template_lines(
                "; rml:graphMap [ rml:constant ex:SG ]",
                [NAME_POM, AGE_POM]
            ),
            [
                AGE_G1,
                r#"?sm_0 <http://example.com/age> "A" <http://example.com/SG> ."#,
                r#"?sm_0 <http://example.com/name> "N" <http://example.com/SG> ."#,
            ]
        );
    }

    // No upstream RML-KGC test covers this case yet; it is pending a proposal
    // to the RML-KGC test suite (kg-construct).
    #[test]
    fn graph_map_of_a_pom_with_plain_and_referencing_object_maps() {
        let mapping = r#"@prefix ex: <http://example.com/> .
@prefix rml: <http://w3id.org/rml/> .
<http://example.com/Parent> a rml:TriplesMap;
  rml:logicalSource <http://example.com/LS>;
  rml:subjectMap [ rml:template "http://example.com/p/{$.ID}" ] .
<http://example.com/TM> a rml:TriplesMap;
  rml:logicalSource <http://example.com/LS>;
  rml:subjectMap [ rml:template "http://example.com/{$.ID}" ];
  rml:predicateObjectMap [ rml:predicate ex:p;
      rml:object "N";
      rml:objectMap [ rml:parentTriplesMap <http://example.com/Parent> ];
      rml:graphMap [ rml:constant ex:G1 ] ] .
<http://example.com/LS> a rml:LogicalSource;
  rml:iterator "$.students[*]";
  rml:referenceFormulation rml:JSONPath;
  rml:source [ a rml:RelativePathSource;
      rml:root rml:MappingDirectory; rml:path "student.json" ] ."#;
        assert_eq!(
            plan_template_lines(mapping),
            [
                r#"?sm_1 <http://example.com/p> "N" <http://example.com/G1> ."#,
                "?sm_1 <http://example.com/p> ?sm_0_obj <http://example.com/G1> .",
            ]
        );
    }

    /// The sorted serializer template lines of a triples map with one
    /// predicate-object map, and optional extra subject map content.
    fn one_pom_lines(subject_map_extra: &str, pom: &str) -> Vec<String> {
        let mapping = format!(
            r#"@prefix ex: <http://example.com/> .
@prefix rml: <http://w3id.org/rml/> .
ex:TM a rml:TriplesMap;
  rml:logicalSource [ rml:iterator "$.s[*]"; rml:referenceFormulation rml:JSONPath;
      rml:source [ a rml:RelativePathSource;
          rml:root rml:MappingDirectory; rml:path "s.json" ] ];
  rml:subjectMap [ rml:template "http://example.com/{{$.ID}}" {} ];
  rml:predicateObjectMap [ {} ] ."#,
            subject_map_extra, pom
        );
        plan_template_lines(&mapping)
    }

    #[test]
    fn pom_with_several_object_maps_keeps_each() {
        assert_eq!(
            one_pom_lines(
                "",
                r#"rml:predicate ex:p;
      rml:objectMap [ rml:constant "A" ];
      rml:objectMap [ rml:constant "B" ]"#
            ),
            [
                r#"?sm_0 <http://example.com/p> "A" ."#,
                r#"?sm_0 <http://example.com/p> "B" ."#,
            ]
        );
    }

    #[test]
    fn pom_with_object_shortcuts_and_object_maps_keeps_each() {
        assert_eq!(
            one_pom_lines(
                "",
                r#"rml:predicate ex:p;
      rml:object "A"; rml:object "B";
      rml:objectMap [ rml:constant "C" ];
      rml:objectMap [ rml:constant "D" ]"#
            ),
            [
                r#"?sm_0 <http://example.com/p> "A" ."#,
                r#"?sm_0 <http://example.com/p> "B" ."#,
                r#"?sm_0 <http://example.com/p> "C" ."#,
                r#"?sm_0 <http://example.com/p> "D" ."#,
            ]
        );
    }

    #[test]
    fn pom_with_several_predicate_maps_keeps_each() {
        assert_eq!(
            one_pom_lines(
                "",
                r#"rml:predicateMap [ rml:constant ex:p1 ];
      rml:predicateMap [ rml:constant ex:p2 ];
      rml:objectMap [ rml:constant "A" ]"#
            ),
            [
                r#"?sm_0 <http://example.com/p1> "A" ."#,
                r#"?sm_0 <http://example.com/p2> "A" ."#,
            ]
        );
    }

    #[test]
    fn pom_with_several_graph_maps_keeps_each() {
        assert_eq!(
            one_pom_lines(
                "",
                r#"rml:predicate ex:p; rml:objectMap [ rml:constant "A" ];
      rml:graphMap [ rml:constant ex:G1 ];
      rml:graphMap [ rml:constant ex:G2 ]"#
            ),
            [
                r#"?sm_0 <http://example.com/p> "A" <http://example.com/G1> ."#,
                r#"?sm_0 <http://example.com/p> "A" <http://example.com/G2> ."#,
            ]
        );
    }

    #[test]
    fn subject_map_with_several_graph_maps_keeps_each() {
        assert_eq!(
            one_pom_lines(
                "; rml:graphMap [ rml:constant ex:SG1 ], [ rml:constant ex:SG2 ]",
                r#"rml:predicate ex:p; rml:objectMap [ rml:constant "A" ]"#
            ),
            [
                r#"?sm_0 <http://example.com/p> "A" <http://example.com/SG1> ."#,
                r#"?sm_0 <http://example.com/p> "A" <http://example.com/SG2> ."#,
            ]
        );
    }
}
