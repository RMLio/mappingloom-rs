use clap::{arg, ArgAction, Command};

pub const TRANSLATOR_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Cli {
    pub cmd: Command,
}

impl Cli {
    pub fn new() -> Cli {
        let cmd = Command::new("AlgeMapLoom-rs translator ")
            .version(TRANSLATOR_VERSION)
            .author("Sitt Min Oo")
            .about(format!("Translates mapping documents to execution plans consisting of algebraic mapping operators.\n\
                Current version {} supports RML and ShExML mapping languages.", TRANSLATOR_VERSION))
            .subcommand_required(true)
            .propagate_version(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("file")
                         .about("translate a single mapping document")
                         .arg(arg!(<DOCUMENT> "the mapping document to be translated"))
                         .arg_required_else_help(true))
            .subcommand(Command::new("folder")
                         .about("translate the mapping documents under the given folder: its .ttl files, or its .shexml files for ShExML")
                         .arg(arg!(<FOLDER> "the folder containing several mapping documents"))
                         .arg_required_else_help(true))
            .subcommand(Command::new("stdin")
                         .about("translate the mapping document on stdin and write the plan to stdout"))
            .arg(arg!(-d --debug ...  "Turns on debugging and logging to file"))
            .arg(arg!(-j --json ... "Only write the plan as JSON, without the DOT file"))
            .arg(arg!(-p --pretty ... "Pretty print output"))
            .arg(arg!(--language <LANGUAGE> "The language of the mapping: rml-kgc, rml-io, r2rml or shexml"))
            .arg(arg!(--"base-iri" <IRI> "The base IRI of every triples map without rml:baseIRI (default: the mapping's @base)"))
            .arg(arg!(--optimizer <TOGGLE> "Switches an optimizer rule on or off, as <RULE>=true or <RULE>=false")
                         .action(ArgAction::Append))
            .arg(arg!(--options <FILE> "A JSON file with the options object; the flags above override it"))
            .arg(arg!(--report "Write the result document (the plan or the errors, and the warnings) instead of the bare plan"));

        Self { cmd }
    }
}
