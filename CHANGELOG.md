# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
- Release tooling: `RELEASE.md` documents the steps; `bump-version.sh` checks the version format.
- Manual `conformance` CI job: MappingWeaver, built with this translator, runs the kg-construct test suites; a regression against MappingWeaver's expected results fails it (see HANDBOOK: Conformance).
- `algemaploom-plan` crate: the plans of the canonical pipeline, independent of any mapping language, serialized with a schema version and producer, and their JSON Schema in `schema/plan.schema.json`, which the Java jar also ships (see HANDBOOK: The canonical pipeline).
- `normalizer`: the RML-KGC core rule set, `normalize_kgc`, with links from the normal form to the original document (`nf:origin`), which name its blank nodes by skolem IRIs from their canonical labels.
- `pipeline-test` crate: test harness of the canonical pipeline, with graph cases for normalizer rules and plan snapshots validated against the plan schema.
- Translation requests and results (see HANDBOOK: Requests, results and errors): the caller names the language (`rml-kgc`, `rml-io` or `shexml`; `r2rml` has no translator yet) and may set the processor's base IRI and optimizer toggles; a translation gives a plan with warnings, or error records with a kind, a code and, for Turtle syntax errors, a location, also for a panic. `translator-bin` takes `--language`, `--base-iri`, `--optimizer`, `--options` and `--report`, and exits with 1 when the mapping can't be translated, 2 for usage and request errors, 3 for internal ones.
- `optimizer` crate: the optimizer framework of the canonical pipeline, with a `Rule` trait and per-rule toggles; it has no rules yet.
- `translator_kgc` crate: the canonical RML-KGC translator, which normalizes a mapping and translates its triples maps into an `algemaploom-plan` plan, with templates, constants, references, term types, datatype and language maps, fresh blank nodes, graphs and file sources (see HANDBOOK: The canonical pipeline). The request option `"translator": "canonical"` (`translator-bin --translator canonical`, Java `Options.withTranslator`) selects it; joins, function executions, gather maps, star maps, logical targets, logical views and other sources are *unsupported* errors for now.
- The Java jar carries the native libraries for macOS (x86_64 and aarch64), cross-compiled with cargo-zigbuild.

### Changed
- **Breaking:** the Java binding's `ITranslator.translate(mapping, Options)` returns a `Translation` (`plan()`, `warnings()`) and throws a `TranslationException` with the error records, through the native method `translateJson`; its `Cli` takes the language as its first argument.
- **Breaking:** the Python binding's `translate(mapping, options)` returns a `Translation` (`.plan`, `.warnings`) and raises `TranslationError` (`.errors`, `.warnings`); the Node.js binding's `translate(mapping, options)` returns `{plan, warnings}` and throws an `Error` named `TranslationError`.
- `translator-bin` is in its own crate, `translator-cli`, so the bindings' libraries leave out the CLI's dependencies; `normalizer` uses the logger in its example only.
- `translator-bin folder` translates the `.ttl` files, or the `.shexml` files for ShExML.
- `translator_new_rml::extractors::io::parse_str` takes the processor's base IRI.
- The dev profile builds the dependencies and `translator` with `opt-level = 1`, so the bindings' debug library links on Windows (GNU).
- CI: the jobs that build the Java binding share the native toolchains (`.native-targets`, `.apple-targets` for the jobs whose jar ships), and `build_native.sh` stops at the first failure; `bump-version.sh` updates `Cargo.lock` with `cargo update --workspace`.
- Java lib: don't build fat jar.
- Between releases the crates, the Java binding and the Node.js package carry the next patch `-SNAPSHOT` version; `bump-version.sh` sets it after each release.
- `translator_new_rml`'s tests against stored plans are plan snapshots (`plan_snapshots`, was `regression_tests`): they detect a change in a plan, and the conformance job checks whether a plan is right.
- `sparql-sat-checker` is marked unmaintained, and its crate documentation lists its known defects.
- `translator_new_rml`: an operator translator that looks up other mapping parts implements `StoreOperatorTranslator::translate_with_store` (extend, join, serializer); `OperatorTranslator::translate` is required for the others (see HANDBOOK: How a mapping becomes a plan)

### Fixed
- The Python module is `ltranslator`, the name that `build_python.sh` installs it under.
- `translator-bin stdin` and the Java binding's `Cli` read the mapping with its line breaks, so a Turtle comment ends at its line.
- `translator_new_rml`: the serializer assigns graphs per predicate-object map (the subject map's graphs plus its own, the default graph when there are none), so a graph map of one predicate-object map no longer moves the triples of the others.
- `translator_new_rml`: a predicate-object map with several `rml:objectMap` or `rml:object` values keeps all of them in the plan.

### Removed
- `translator`: `api::process_one_file`, `api::process_one_str`, `TranslatorHandler`, the `error` and `util` modules, and the guess of the RML translator from the R2RML namespace; `translator-bin -o/--outputFolderSuffix`, which had no effect.
- `SearchStore` fields and methods that nothing used: `reference_attr_map`, `add_reference_name`, `abs_ls_search_map`, `pm_search_map`, `om_search_map` and `gm_search_map`
- Commented-out `FileTranslatorHandler` implementation in `translator::rml`
- `normalizer.py`, a Python copy of the RML.io rule set
- `architecture/README.md`, a future-work note on the crate split, and the unused `websocket_example.ttl`

## [0.8.0] - 2026-08-19

### Added
- A CSV on the Web table (`csvw:Table`) is understood as a source. The table says where the data is (`csvw:url`) and which values stand for no value (`csvw:null`); the dialect it points at says how the rows are written, and the delimiter, quote character, encoding, header, trimming and the rest travel to the plan as the source's parse configuration. A table is read as a file, the way the RML v1 translator already reads one, so what is particular to it is the dialect and not where the data comes from. The nine CSVW test cases of the RML-IO registry are added as regression tests.
- `csvw:quoteChar` and `csvw:null` in the vocabulary. It held `csvw:quoteChars`, which CSVW does not define, so a dialect naming a quote character was passed over.

### Changed
- Translator API: remove `FileTranslatorHandler` and `StringTranslatorHandler` traits, this is now one `TranslatorHandler` trait.
- Added `-p` parameter to pretty-print the generated mapping plan in the CLI translator app.

### Fixed
- `rml:SQL2008Table` and `rml:SQL2008Query` are recognised as reference formulations. RML-IO gives a relational source's reference formulation those names, but only the earlier `rml:SQLTable` and `rml:SQLQuery` were known, so a mapping reading from a database was rejected with "Unsupported reference formulation". Of the RML-IO registry test cases that MappingLoom could not translate, 77 translate now; test case RMLSTC0006a is no longer ignored.
- Test case RMLLVTC0010d, a logical view join on a template-valued field, is no longer ignored: it translates, and the reason it carried no longer holds.
- Base IRI was missing in [RMLTC0002a-CSV.ttl](crates/translator_rml/resources/csv-testcases/RMLTC0002a-CSV.ttl).

## [0.7.1] - 2026-07-16

### Fixed
- Source operator: inner fields of an expression field are also added now.
- bump-version: run `cargo check` after updating `Cargo.toml` to ensure `Cargo.lock` is also updated.

## [0.7.0] - 2026-07-16

### Changed
- The `reference` and `constant` properties in a source's Field is replaced by a single `expression` property, whose value is a function similar to the function in the Extend operator.
- Removed unused crate `weaver`

### Added
- CI: check if CHANGELOG.md is updated when committing to a new branch.

## [0.6.9] - 2026-07-15

### Added
- Regression tests for the RML-IO, RML-CC, RML-FNML and RML-STAR specifications.

### Fixed
- Base IRI extraction from an `@base` statement: the line is now trimmed before parsing, so the base IRI is extracted correctly on CRLF line endings (previously a trailing `" ."` could leak into it).

## [0.6.8] - 2026-07-07

### Fixed
- `bump-version.sh` script to include `Cargo.lock`

## [0.6.7] - 2026-07-07

### Fixed
- RML: Self-joins are detected and removed when logical sources are "effectively equal".
- RML template parsing.
- RML: Don't require an expression map if the term type is `rml:BlankNode`.

### Changed
- Refactored crate `operator` to be more modular and easier to maintain.

## [0.6.6] - 2026-03-18

### Fixed
- `SerializerOperatorTranslator`: don't apply graph mapping in case of a join because that's already handled by the `JoinTranslator`. Fixes <https://gitlab.ilabt.imec.be/rml/proc/algemaploom-rs/-/issues/47>
- Use official rust Docker image in GitLab CI

[0.8.0]: https://github.com/RMLio/mappingloom-rs/compare/v0.7.1...v0.8.0
[0.7.1]: https://github.com/RMLio/mappingloom-rs/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/RMLio/mappingloom-rs/compare/v0.6.9...v0.7.0
[0.6.9]: https://github.com/RMLio/mappingloom-rs/compare/v0.6.8...v0.6.9
[0.6.8]: https://github.com/RMLio/mappingloom-rs/compare/v0.6.7...v0.6.8
[0.6.7]: https://github.com/RMLio/mappingloom-rs/compare/v0.6.6...v0.6.7
[0.6.6]: https://github.com/RMLio/mappingloom-rs/compare/v0.6.6...v0.6.5
