use std::collections::HashMap;
use std::rc::Rc;

use super::error::TranslationError;
use super::store::SearchStore;
use super::StoreOperatorTranslator;
use crate::error::{NewRMLTranslationError, NewRMLTranslationResult};
use crate::extractors::stringify_term;
use crate::rml_model::v2::core::expression_map::term_map::{
    CommonTermMapInfo, ObjectMap, RMLTermTypeKind,
};
use crate::rml_model::v2::core::expression_map::{
    BaseExpressionMapEnum, ExpressionMapEnum,
};
use crate::rml_model::v2::core::{TemplateSubString, TriplesMap};
use crate::rml_model::v2::fnml::FunctionExpressionMap;
use operator::extend::function::Function;
use operator::extend::Extend;
use sophia_api::term::Term;

pub fn func_is_not_constant(func: &Function) -> bool {
    match func {
        Function::Iri {
            base_iri: _,
            inner_function,
        } => func_is_not_constant(inner_function),
        Function::Literal {
            inner_function,
            dtype_function: _,
            langtype_function: _,
        } => func_is_not_constant(inner_function),
        Function::BlankNode { inner_function } => {
            func_is_not_constant(inner_function)
        }
        Function::TypedConstant {
            value: _,
            term_type: _,
        } => false,
        Function::Constant { value: _ } => false,
        Function::Nop => false,
        _ => true,
    }
}

#[derive(Debug, Clone)]
pub struct ExtendOperatorTranslator {}

impl StoreOperatorTranslator for ExtendOperatorTranslator {
    type Input = TriplesMap;

    type Output = Extend;

    fn translate_with_store(
        store: &SearchStore,
        tm: &Self::Input,
    ) -> NewRMLTranslationResult<Self::Output> {
        let base_iri = &tm.base_iri;
        let mut extend_pairs: HashMap<String, Function> = HashMap::new();

        // Extend function for the subject map
        let (var, func) =
            extend_from_term_map(store, base_iri, tm.subject_map.as_ref())?;
        insert_non_constant_func(&mut extend_pairs, var, func);

        // Extend functions for each graph map of a subject map
        if let Ok(sm) = tm.subject_map.try_unwrap_subject_map_ref() {
            for gm in &sm.graph_maps {
                let (var, func) =
                    extend_from_term_map(store, base_iri, gm.as_ref())?;
                insert_non_constant_func(&mut extend_pairs, var, func);
            }
        }

        // Extend functions for each predicate object map of the given triples map
        for pom in &tm.predicate_object_map_vec {
            for gm in &pom.graph_map_vec {
                let (var, func) =
                    extend_from_term_map(store, base_iri, gm.as_ref())?;
                insert_non_constant_func(&mut extend_pairs, var, func);
            }

            for pm in &pom.predicate_map_vec {
                let (var, func) =
                    extend_from_term_map(store, base_iri, pm.as_ref())?;
                insert_non_constant_func(&mut extend_pairs, var, func);
            }

            for om_enum in &pom.object_map_vec {
                let (var, mut func) =
                    extend_from_term_map(store, base_iri, om_enum.as_ref())?;

                if let Ok(om) = om_enum.try_unwrap_object_map_ref() {
                    func = extend_lang_dtype_function_for_om(
                        base_iri, om, func,
                    )?;
                }
                insert_non_constant_func(&mut extend_pairs, var, func);
            }
        }
        Ok(Extend { extend_pairs })
    }
}

pub fn insert_non_constant_func(
    extend_pairs: &mut HashMap<String, Function>,
    var: String,
    func: Function,
) {
    if func_is_not_constant(&func) {
        extend_pairs.insert(var, func);
    }
}

fn extend_lang_dtype_function_for_om(
    base_iri: &str,
    om: &ObjectMap,
    func: Function,
) -> Result<Function, NewRMLTranslationError> {
    let term_type = &om.term_map_info.get_term_type_enum();
    log::debug!(
        "Term expression {:?} is a literal",
        om.term_map_info.expression
    );
    Ok(match &func {
        Function::Literal {
            inner_function,
            dtype_function: _,
            langtype_function: _,
        } => {
            if om.language_map.is_some() {
                let langtype_function = Some(Rc::new(
                    extension_func_from_exp_map(&om.language_map, term_type)?,
                ));
                Function::Literal {
                    inner_function: inner_function.clone(),
                    dtype_function: None,
                    langtype_function,
                }
            } else if om.datatype_map.is_some() {
                let dtype_function = Some(Rc::new(Function::Iri {
                    base_iri:       Some(base_iri.to_string()),
                    inner_function: extension_func_from_exp_map(
                        &om.datatype_map, term_type,
                    )?
                    .into(),
                }));
                Function::Literal {
                    inner_function: inner_function.clone(),
                    dtype_function,
                    langtype_function: None,
                }
            } else {
                func
            }
        }
        _ => func,
    })
}

pub fn extend_from_term_map(
    store: &SearchStore,
    base_iri: &str,
    term_map_info: &CommonTermMapInfo,
) -> NewRMLTranslationResult<(String, Function)> {
    let inner_func = extension_func_from_exp_map(
        &term_map_info.expression,
        &term_map_info.get_term_type_enum(),
    )?;

    let term_type = term_map_info.get_term_type_enum();
    let function = match term_type {
        RMLTermTypeKind::BlankNode => {
            Ok(Function::BlankNode {
                inner_function: inner_func.into(),
            })
        }
        RMLTermTypeKind::UnsafeIRI
        | RMLTermTypeKind::UnsafeURI
        | RMLTermTypeKind::URI
        | RMLTermTypeKind::IRI => {
            let mut base_iri_opt = Some(base_iri.to_string());
            if term_type == RMLTermTypeKind::URI
                || term_type == RMLTermTypeKind::UnsafeURI
            {
                base_iri_opt = None;
            }
            Ok(Function::Iri {
                base_iri:       base_iri_opt,
                inner_function: inner_func.into(),
            })
        }
        RMLTermTypeKind::Literal => {
            log::debug!(
                "Term expression {:?} is a literal",
                term_map_info.expression
            );
            let mut dtype_function = None;
            let mut langtype_function = None;
            if let Ok(BaseExpressionMapEnum::Constant(term)) = term_map_info
                .expression.as_ref().unwrap()
                .try_unwrap_base_expression_map_ref()
            {
                if let Some(lt) = term.language_tag() {
                    langtype_function = Some(
                        Function::Constant {
                            value: lt.as_str().to_string(),
                        }
                        .into(),
                    );
                }
                if let Some(dt) = term.datatype() {
                    let dtype_inner = Function::Constant {
                        value: dt.as_str().to_string(),
                    }
                    .into();
                    dtype_function = Some(
                        Function::Iri {
                            base_iri:       None,
                            inner_function: dtype_inner,
                        }
                        .into(),
                    );
                }
            }

            Ok(Function::Literal {
                inner_function: inner_func.into(),
                dtype_function,
                langtype_function,
            })
        }

        _ => {
            Err(TranslationError::ExtendError(format!(
                "Given term type is unsupported: {:?}",
                stringify_term(term_map_info.term_type.clone())
            )))
        }
    }?;

    let var = store
        .termm_id_quad_var_map
        .get(&term_map_info.identifier)
        .unwrap()
        .to_string();
    Ok((var, function))
}

pub fn extension_func_from_exp_map(
    exp_map_opt: &Option<ExpressionMapEnum>,
    term_type: &RMLTermTypeKind,
) -> NewRMLTranslationResult<Function> {
    if let Some(exp_map) = exp_map_opt {
        match exp_map {
            ExpressionMapEnum::BaseExpressionMap(base_expression_map_enum) => {
                extend_func_from_base_expr_map(base_expression_map_enum, term_type)
            }
            ExpressionMapEnum::FunctionExpressionMap(function_expression_map) => {
                extend_func_from_func_expr_map(
                    function_expression_map,
                    term_type,
                )
            }
        }
    } else {
        // Term type is Blank node => return function that tells to generate blank nodes
        Ok(Function::GenerateBlankNode)
    }
}
fn extend_func_from_base_expr_map(
    base_expr_map: &BaseExpressionMapEnum,
    term_type: &RMLTermTypeKind,
) -> NewRMLTranslationResult<Function> {
    match base_expr_map {
        BaseExpressionMapEnum::Template(_) => {
            Ok(template_extend_function(base_expr_map, term_type))
        }
        BaseExpressionMapEnum::Reference(reference) => {
            Ok(extend_func_from_ref_attr(reference))
        }
        BaseExpressionMapEnum::Constant(constant) => {
            let value = stringify_term(constant)
                .ok_or(TranslationError::ExtendError(
                        format!("Empty string returned while trying to get the string representation of the term {:?}", constant)
                        )
                    )?;
            Ok(Function::Constant {
                value ,
            })
        }
        BaseExpressionMapEnum::Unknown { type_iri, term_val } =>  {
            Err(TranslationError::ExtendError(
                format!("Cannot translate extension function for expression map with type: {:?} and value {:?}", type_iri, term_val)
                ).into())
        },
    }
}

fn template_extend_function(
    exp_map: &BaseExpressionMapEnum,
    term_type: &RMLTermTypeKind,
) -> Function {
    let template_splits = exp_map.get_template_string_split();
    let mut template_function = Function::Nop;

    for split in template_splits {
        let right_func_opt = match &split {
            TemplateSubString::Attribute(attr) => {
                let inner_func = extend_func_from_ref_attr(attr);
                let func = match term_type {
                    RMLTermTypeKind::BlankNode
                    | RMLTermTypeKind::IRI
                    | RMLTermTypeKind::URI => {
                        Function::UriEncode {
                            inner_function: Rc::new(inner_func),
                        }
                    }
                    _ => inner_func,
                };

                Some(func)
            }
            TemplateSubString::NormalString(norm) => {
                if norm.is_empty() {
                    None
                } else {
                    Some(Function::Constant {
                        value: norm.to_string(),
                    })
                }
            }
        };

        if let Some(right_func) = right_func_opt {
            template_function = Function::Concatenate {
                left_value:  template_function.into(),
                separator:   "".to_string(),
                right_value: right_func.into(),
            };
        }
    }
    template_function
}

fn extend_func_from_ref_attr(attr: &str) -> Function {
    Function::Reference {
        value: attr.to_string(),
    }
}

fn extend_func_from_func_expr_map(
    func_exp_map: &FunctionExpressionMap,
    term_type: &RMLTermTypeKind,
) -> NewRMLTranslationResult<Function> {
    let execution = &func_exp_map.func_execution;

    // Extract function identifier
    let fno_identifier = func_exp_map
        .func_execution
        .function_map
        .term_map_info
        .get_constant_value()
        .ok_or_else(|| {
            TranslationError::ExtendError(
                "Function map does not have a constant value".to_string(),
            )
        })?;
    // Remove surrounding angle brackets if present (e.g., <http://example.com/fn>)
    let fno_identifier = strip_angle_brackets(&fno_identifier);

    // Build parameters HashMap from input maps
    let mut parameters = HashMap::with_capacity(execution.input.len());
    for input in &execution.input {
        let param_name =
            input.parameter_map.get_constant_value().ok_or_else(|| {
                TranslationError::ExtendError(
                    "Parameter map does not have a constant value".to_string(),
                )
            })?;
        let param_name = strip_angle_brackets(&param_name);
        // If the input value map is a plain reference expression, do not
        // wrap it with a UriEncode; return a Reference function directly.
        let input_func = if let Ok(base_expr) = input
            .input_value_map
            .expression.as_ref().unwrap()
            .try_unwrap_base_expression_map_ref()
        {
            match base_expr {
                BaseExpressionMapEnum::Reference(ref_attr) => {
                    Function::Reference { value: ref_attr.to_string() }
                }
                _ => extension_func_from_exp_map(
                    &input.input_value_map.expression,
                    term_type,
                )?,
            }
        } else {
            extension_func_from_exp_map(
                &input.input_value_map.expression,
                term_type,
            )?
        };

        parameters.insert(param_name, input_func.into());
    }

    // Extract optional rml:return
    let return_type = func_exp_map
        .return_map
        .as_ref()
        .and_then(|rm| rm.get_constant_value())
        .map(|v| strip_angle_brackets(&v));

    Ok(Function::FnO {
        fno_identifier,
        parameters,
        return_type,
    })
}

// Helper to remove surrounding angle brackets from turtle-stringified IRIs
fn strip_angle_brackets(value: &str) -> String {
    if value.starts_with('<') && value.ends_with('>') && value.len() >= 2 {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}
