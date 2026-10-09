# TODO

## Fix now

## Fix later

- [ ] Decide whether `sparql-sat-checker` and its placeholder binary `sparql-rml-pruner` (`unimplemented!`) are rewritten, for instance as an optional optimizer rule, or removed. Until then they stay as they are, with the known defects listed in the crate documentation.
- [ ] Replace the heuristic in `RMLHandler` (`crates/translator/src/rml.rs`) that picks the RML translator by searching the text for the R2RML namespace (marked TODO in the code).
- [ ] Add a `cargo clippy --workspace` job to CI; the CI lint stage only checks `CHANGELOG.md`.

## To triage
