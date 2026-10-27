use jni::objects::{JClass, JString};
use jni::EnvUnowned;

use crate::api::translate_json;

/// `Translator.translateJson(mapping, options)`: the options object and the
/// result document as JSON text. The Java wrapper turns errors into a
/// `TranslationException`.
#[unsafe(no_mangle)]
pub extern "system" fn Java_be_ugent_idlab_knows_mappingLoom_Translator_translateJson<
    'local,
>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    mapping: JString<'local>,
    options: JString<'local>,
) -> JString<'local> {
    let outcome =
        unowned_env.with_env(|env| -> Result<_, jni::errors::Error> {
            let result =
                translate_json(&mapping.to_string(), &options.to_string());
            JString::from_str(env, result.as_str())
        });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
