# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed
- Java lib: don't build fat jar.
- Java lib: between releases the Maven version is the next patch `-SNAPSHOT` (now `0.8.1-SNAPSHOT`); `bump-version.sh` sets it after each release.
- `translator_new_rml`: an operator translator that looks up other mapping parts implements `StoreOperatorTranslator::translate_with_store` (extend, join, serializer); `OperatorTranslator::translate` is required for the others (see HANDBOOK: How a mapping becomes a plan)

### Removed
- `SearchStore` fields and methods that nothing used: `reference_attr_map`, `add_reference_name`, `abs_ls_search_map`, `pm_search_map`, `om_search_map` and `gm_search_map`
- Commented-out `FileTranslatorHandler` implementation in `translator::rml`
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
