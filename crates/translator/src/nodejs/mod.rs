use neon::prelude::*;

use crate::api;
use crate::report::Record;
use crate::request::Options;

/// `translate(mapping, options)` with the options as an object, such as
/// `{language: "rml-kgc"}`. Returns `{plan, warnings}`, the plan as JSON
/// text, or throws an `Error` named `TranslationError` with `errors` and
/// `warnings`.
fn translate(mut cx: FunctionContext) -> JsResult<JsObject> {
    let mapping = cx.argument::<JsString>(0)?.value(&mut cx);
    let options = cx.argument::<JsValue>(1)?;
    let json = cx.global::<JsObject>("JSON")?;
    let stringify = json.get::<JsFunction, _, _>(&mut cx, "stringify")?;
    let options = stringify
        .call(&mut cx, json, [options])?
        .downcast_or_throw::<JsString, _>(&mut cx)?
        .value(&mut cx);
    match Options::from_json(&options)
        .and_then(|options| api::translate(&mapping, &options))
    {
        Ok(translation) => {
            let result = cx.empty_object();
            let plan = cx.string(translation.plan.to_json_string().unwrap());
            result.set(&mut cx, "plan", plan)?;
            let warnings = records(&mut cx, &translation.warnings)?;
            result.set(&mut cx, "warnings", warnings)?;
            Ok(result)
        }
        Err(errors) => {
            let err = cx.error(errors.to_string())?;
            let name = cx.string("TranslationError");
            err.set(&mut cx, "name", name)?;
            let records_of_errors = records(&mut cx, &errors.errors)?;
            err.set(&mut cx, "errors", records_of_errors)?;
            let warnings = records(&mut cx, &errors.warnings)?;
            err.set(&mut cx, "warnings", warnings)?;
            cx.throw(err)
        }
    }
}

/// The records as JS objects.
fn records<'a>(
    cx: &mut FunctionContext<'a>,
    records: &[Record],
) -> JsResult<'a, JsValue> {
    let json = cx.global::<JsObject>("JSON")?;
    let parse = json.get::<JsFunction, _, _>(cx, "parse")?;
    let text = cx.string(serde_json::to_string(records).unwrap());
    parse.call(cx, json, [text.upcast()])
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("translate", translate)?;
    Ok(())
}
