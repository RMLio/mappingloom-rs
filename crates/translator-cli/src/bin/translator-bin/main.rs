mod cli;

use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::ArgMatches;
use colored::Colorize;
use common::logger::init_logger;
use log::{debug, error, info, warn};
use serde_json::{Map, Value};
use translator::api::translate;
use translator::report::{result_document, Translation, TranslationErrors};
use translator::request::{Language, Options};
use walkdir::WalkDir;

/// The exit code of a usage error or a request error.
const USAGE: u8 = 2;

/// How the plans are written.
struct Output {
    json_only: bool,
    pretty: bool,
    report: bool,
}

pub fn main() -> ExitCode {
    let matches = cli::Cli::new().cmd.get_matches();
    let debug_flag_count = *matches.get_one::<u8>("debug").unwrap();
    if let Err(err) = init_logger(debug_flag_count >= 1) {
        eprintln!("{err}");
        return ExitCode::from(USAGE);
    }
    let output = Output {
        json_only: *matches.get_one::<u8>("json").unwrap() > 0,
        pretty: *matches.get_one::<u8>("pretty").unwrap() > 0,
        report: matches.get_flag("report"),
    };
    let options = match options(&matches).map(Options::from_value) {
        Ok(Ok(options)) => options,
        Ok(Err(errors)) => {
            return ExitCode::from(emit(&Err(errors), &output, print));
        }
        Err(message) => {
            error!("{message}");
            return ExitCode::from(USAGE);
        }
    };

    let code = if let Some(file_matches) = matches.subcommand_matches("file") {
        let file_path: PathBuf =
            file_matches.get_one::<String>("DOCUMENT").unwrap().into();
        let output_prefix =
            file_path.file_stem().map_or("output".to_string(), |stem| {
                stem.to_string_lossy().to_string()
            });
        translate_file(&file_path, &output_prefix, &options, &output)
    } else if let Some(folder_matches) = matches.subcommand_matches("folder") {
        let folder_path: &String = folder_matches.get_one("FOLDER").unwrap();
        let extension = match options.language {
            Language::Shexml => "shexml",
            _ => "ttl",
        };
        WalkDir::new(folder_path)
            .max_depth(4)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_type().is_file()
                    && entry.path().extension().is_some_and(|e| e == extension)
            })
            .map(|entry| {
                let output_prefix = entry.path().with_extension("");
                translate_file(
                    entry.path(),
                    &output_prefix.to_string_lossy(),
                    &options,
                    &output,
                )
            })
            .max()
            .unwrap_or(0)
    } else {
        match io::read_to_string(io::stdin()) {
            Ok(mapping) => {
                debug!("Attempting to translate from stdin");
                emit(&translate(&mapping, &options), &output, print)
            }
            Err(err) => {
                error!("Errored while reading stdin: {err}");
                USAGE
            }
        }
    };
    ExitCode::from(code)
}

/// The options object of the `--options` file, with the flags applied.
fn options(matches: &ArgMatches) -> Result<Value, String> {
    let mut options = match matches.get_one::<String>("options") {
        Some(path) => {
            let text = std::fs::read_to_string(path).map_err(|err| {
                format!("Errored while reading {path}: {err}")
            })?;
            match serde_json::from_str(&text) {
                Ok(Value::Object(options)) => options,
                _ => return Err(format!("{path} holds no JSON object")),
            }
        }
        None => Map::new(),
    };
    if let Some(language) = matches.get_one::<String>("language") {
        options.insert("language".into(), language.clone().into());
    }
    if let Some(base_iri) = matches.get_one::<String>("base-iri") {
        options.insert("base_iri".into(), base_iri.clone().into());
    }
    if let Some(toggles) = matches.get_many::<String>("optimizer") {
        let Value::Object(optimizer) = options
            .entry("optimizer")
            .or_insert_with(|| Value::Object(Map::new()))
        else {
            return Err("the optimizer option is no JSON object".to_string());
        };
        for toggle in toggles {
            let (rule, on) = toggle
                .split_once('=')
                .and_then(|(rule, on)| Some((rule, on.parse::<bool>().ok()?)))
                .ok_or_else(|| {
                    format!(
                        "--optimizer {toggle} is not <RULE>=true or <RULE>=false"
                    )
                })?;
            optimizer.insert(rule.to_string(), on.into());
        }
    }
    Ok(Value::Object(options))
}

fn print(json: String) -> io::Result<()> {
    println!("{json}");
    Ok(())
}

/// Writes the JSON output with `write` (the result document with
/// `--report`, else the plan), logs the warnings and errors, and returns the
/// exit code: 0 for a plan, else that of the errors' kind.
fn emit(
    result: &Result<Translation, TranslationErrors>,
    output: &Output,
    write: impl FnOnce(String) -> io::Result<()>,
) -> u8 {
    let (warnings, errors, code) = match result {
        Ok(translation) => (&translation.warnings[..], &[][..], 0),
        Err(errors) => (
            &errors.warnings[..],
            &errors.errors[..],
            errors.kind().exit_code(),
        ),
    };
    let json = if output.report {
        let document = result_document(result);
        Some(if output.pretty {
            serde_json::to_string_pretty(&document).unwrap()
        } else {
            document.to_string()
        })
    } else if let Ok(translation) = result {
        Some(if output.pretty {
            translation.plan.to_pretty_json_string().unwrap()
        } else {
            translation.plan.to_json_string().unwrap()
        })
    } else {
        None
    };
    if let Some(json) = json {
        if let Err(err) = write(json) {
            error!("Errored while writing the JSON output: {err}");
            return USAGE;
        }
    }
    warnings.iter().for_each(|warning| warn!("{warning}"));
    errors.iter().for_each(|error| error!("{error}"));
    code
}

/// Translates one file and writes `<output_prefix>.json` and, for a plan
/// without `json_only`, `<output_prefix>.dot`. Returns the exit code.
fn translate_file(
    file_path: &Path,
    output_prefix: &str,
    options: &Options,
    output: &Output,
) -> u8 {
    debug!("Attempting to translate: {}", file_path.display());
    let mapping = match std::fs::read_to_string(file_path) {
        Ok(mapping) => mapping,
        Err(err) => {
            error!("Errored while reading {}: {err}", file_path.display());
            return USAGE;
        }
    };
    let result = translate(&mapping, options);
    let json_path = format!("{output_prefix}.json");
    let code = emit(&result, output, |json| std::fs::write(&json_path, json));
    let Ok(translation) = result else {
        error!("Errored while translating {}", file_path.display());
        return code;
    };
    if code != 0 {
        return code;
    }
    info!("Generated json file: {}", json_path.yellow());
    if !output.json_only {
        let dot_path = format!("{output_prefix}.dot");
        let written = if output.pretty {
            translation.plan.write_pretty(dot_path.clone().into())
        } else {
            translation.plan.write(dot_path.clone().into())
        };
        if let Err(err) = written {
            error!("Errored while writing {dot_path}: {err:?}");
            return USAGE;
        }
        info!("Generated dot file: {}", dot_path.yellow());
    }
    0
}
