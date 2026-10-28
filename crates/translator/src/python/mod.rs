use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::api;
use crate::report::Record;
use crate::request::Options;

create_exception!(
    ltranslator,
    TranslationError,
    PyException,
    "A failed translation: `errors` and `warnings` hold the error records."
);

/// A successful translation: the plan as JSON text, and the warnings.
#[pyclass(frozen, get_all)]
struct Translation {
    plan: String,
    warnings: Py<PyAny>,
}

/// Translates a mapping document with the options of a dict, such as
/// `{"language": "rml-kgc"}`. Raises `TranslationError` on failure.
#[pyfunction]
fn translate(
    py: Python<'_>,
    mapping: &str,
    options: &Bound<'_, PyAny>,
) -> PyResult<Translation> {
    let json = py.import("json")?;
    let options: String = json.call_method1("dumps", (options,))?.extract()?;
    let records = |records: &[Record]| {
        json.call_method1("loads", (serde_json::to_string(records).unwrap(),))
    };
    match Options::from_json(&options)
        .and_then(|options| api::translate(mapping, &options))
    {
        Ok(translation) => Ok(Translation {
            plan: translation.plan.to_json_string(),
            warnings: records(&translation.warnings)?.unbind(),
        }),
        Err(errors) => {
            let err = TranslationError::new_err(errors.to_string());
            err.value(py).setattr("errors", records(&errors.errors)?)?;
            err.value(py)
                .setattr("warnings", records(&errors.warnings)?)?;
            Err(err)
        }
    }
}

/// The native module, which `build_python.sh` installs as `ltranslator`.
#[pymodule]
fn ltranslator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(translate, m)?)?;
    m.add_class::<Translation>()?;
    m.add("TranslationError", m.py().get_type::<TranslationError>())?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
