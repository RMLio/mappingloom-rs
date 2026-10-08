# MappingLoom (algemaploom-rs) Handbook

Written for a CS student who wants to understand MappingLoom as code.

## Contents

1. [Preface](#preface)
2. [Agent request contract (for AI agents/LLMs)](#agent-request-contract-for-ai-agentsllms)
3. [Workspace layout](#workspace-layout)
4. [How a mapping becomes a plan](#how-a-mapping-becomes-a-plan)
5. [Build, format and test](#build-format-and-test)
6. [Test resources](#test-resources)
7. [Language bindings](#language-bindings)
8. [Release process](#release-process)

## Preface

MappingLoom translates mapping documents (RML and ShExML) into a mapping
plan: a graph of algebraic mapping operators that gives the mapping process
operational semantics. MappingLoom is a translator; it produces the plan and
leaves executing it (reading data, generating RDF) to an engine that consumes
the plan.

- Source: a Cargo workspace in Rust (toolchain pinned to 1.87.0 in
  `rust-toolchain.toml`), with every crate under `crates/`. The workspace
  version (`0.8.0`) lives in the root `Cargo.toml`; the Java binding has its own
  Maven version (see Release process).
- CLIs: the `translator` crate builds `translator-bin`
  (`crates/translator/src/bin/translator-bin/`) with the subcommands `file`,
  `folder` and `stdin`, and the flags `-d` (debug logging), `-j` (JSON only)
  and `-p` (pretty-print). It also declares `sparql-rml-pruner`, which is a
  placeholder (`unimplemented!`). The `normalizer` crate has an example CLI,
  `cli_normalizer`.
- Tests: Rust unit and regression tests inside the crates (`#[test]`), with
  test resources under each crate's `resources/` directory; the Java binding
  has Maven tests under `crates/translator/src/java/algemaploom`.
- The project keeps a `README.md` (usage, covered specs, bindings) and a
  `CHANGELOG.md` in Keep a Changelog format.

## Agent request contract (for AI agents/LLMs)

<!-- software-handbook contract: 2026-10-08 -->

Every implementation request handled by an AI agent/LLM follows these constraints:

- If the request is a feature or bugfix:
  - fix the specific failing case or issue named in the request;
  - preserve existing passing behavior unless explicitly asked not to;
  - add or update a regression test when needed.
- Make the smallest coherent patch. A documentation error found along the way is fixed in the same patch.
- Leave the code leaner after every request: remove what the change makes redundant (duplicate tests, parameters and options that no longer do anything, helpers that duplicate each other, comments that only repeat the code), and reuse shared functionality instead of adding a local variant. Run `cargo clippy --workspace` to find unused code.
- Fix a transient environment problem (a stale PATH, a shell or editor that needs a restart) in the environment, by restarting or reconfiguring it; add no code that works around it.
- **Push back** when a request would violate an established principle (e.g. breaking test hermeticity). Explain the principle and suggest a documentation-only fix instead of silently implementing the harmful change.
- Update this handbook so the change is documented as well as implemented.
  - Document only the latest state, integrated in the surrounding narrative (principles, behavior, rationale), including the choices made and why.
  - This contract holds only general rules for handling a request; project-specific guidance goes in the chapter on that topic.
- Do not stop at making tests green; align the implementation with the specification or intended design, and document the semantic reason in this handbook.
- Never remove or change existing tests (code or fixtures) without explicit permission. A change to an existing fixture (expected output, input, or data) is validated by the maintainer before it is kept, also when a tool writes it: propose the change with its reason, and keep it only after approval.
- Update `CHANGELOG.md` for every change, internal ones included (tests, CI, refactoring, removed code): keep `## Unreleased` a short summary of what changed since the last release. A feature that is new since the last release is one Added line, which later fixes update instead of getting lines of their own.
- Check whether `README.md` needs updates for user-visible behavior or workflow changes, and update it when needed.
- Write documentation (this handbook, READMEs, `TODO.md`, `CHANGELOG.md`, code comments) as plain positive statements: say what is true and leave out the contrast ("X, not Y"). Keep a negative only when it is the point itself, such as a prohibition, a warning, or a known limitation.
- If there are difficulties during fulfillment, document them in the most appropriate existing handbook location (create a new chapter only when truly necessary) so future requests start with better context.
- A preference or principle that the maintainer states while handling a request is documented so that every later request follows it: a general one in this contract (and in the software-handbook skill it comes from), a project-specific one in the handbook chapter it belongs to. When it is unclear which, ask.
- When a request is a list of feedback (such as a `TODO.md`), clean up after handling it: remove the items that are done, keep every open item as a clear task (an open question or an offered follow-up is an open item), and remove temporary files created along the way.

## Workspace layout

All crates live under `crates/` and are workspace members (`members = ["crates/*"]`).

| Crate | Role |
|---|---|
| `operator` | The mapping algebra: operator definitions (source, extend, join, projection, rename, serializer, target I/O, formats). |
| `plan` | The execution plan as a graph of operators, built through typed states (`plan/src/states/`), serialized to JSON and DOT. |
| `vocab` | IRI constants for the vocabularies MappingLoom reads (RML Core/IO/LV/CC/FNML, R2RML, CSVW, D2RQ, FnO, ...). |
| `translator_api` | The `LanguageTranslator` trait every language translator implements (`translate_to_plan`). |
| `translator_rml` | Translator for RML v1 (rml.io spec, R2RML-based). |
| `translator_new_rml` | Translator for the current RML spec (kg-construct RML modules), split into `extractors/` (RDF graph to RML model) and `translator/` (RML model to plan). |
| `translator_shexml` | Translator for ShExML (parser combinators in `parcombi/`), with a best-effort translation of unsupported features. |
| `translator` | The front end: the CLI binaries, the `api` module (`process_one_file`, `process_one_str`) and the Java/Python/Node.js bindings. |
| `common` | Shared utilities such as the logger. |
| `normalizer` | Normalizes an RML document with SPARQL queries over an Oxigraph store. |
| `translator_normalized_rml` | Translator for normalized RML documents. |
| `sparql-sat-checker` | Checks SPARQL/triples-map satisfiability on plans; uses `translator_normalized_rml`. It carries its own version (`0.1.0`) and edition 2024. |

## How a mapping becomes a plan

`translator::api::process` tries each `TranslatorHandler` in turn (`RMLHandler`,
then `ShExMLHandler`) and keeps the first plan that succeeds; when all fail,
it logs the error chain of every handler.

`RMLHandler` (`crates/translator/src/rml.rs`) chooses the RML translator by
looking at the document text: a document that contains the R2RML namespace
(`<http://www.w3.org/ns/r2rml#>`) goes to `translator_rml` (RML v1); every
other document goes to `translator_new_rml` (current RML spec). This detection
is a heuristic, marked as a TODO in the code.

`translator_new_rml` first indexes the document in a `SearchStore`
(`translator/store.rs`): triples maps and subject maps by identifier, a quad
variable per term map, and one sourced plan per group of effectively equal
logical sources. Each operator translator implements one of two traits:
`OperatorTranslator::translate` builds an operator from its input alone (source,
iterator), and `StoreOperatorTranslator::translate_with_store` also looks up
other mapping parts in the store (extend, join, serializer).

The CLI writes the plan next to the input (or with the derived output prefix)
as DOT and JSON; `-j` limits output to JSON. `stdin` prints the JSON plan to
standard output.

## Build, format and test

- Build: `cargo build` (CI) or `cargo build --release` for the CLI in
  `target/release/translator-bin`.
- Test: `cargo test --verbose --jobs 1 --workspace` (the CI command).
- Lint: `cargo clippy --workspace`. No clippy configuration file is checked in.
- Format: `cargo fmt`. The format rules (80-column width, module-granular
  imports grouped std/external/crate, aligned struct fields and enum
  discriminants) live in `rustfmt.toml`.
- Java binding tests: `./test_java.sh` (runs `mvn clean test` in
  `crates/translator/src/java/algemaploom`).

CI (`.gitlab-ci.yml`) runs the stages lint, build, test and deploy. The lint
stage comes from the included `rml/util/ci-templates` `CHANGELOG.gitlab-ci.yml`
template, which checks that `CHANGELOG.md` is updated. The build stage builds
Rust and the Java, Python and Node.js bindings; the test stage runs the Rust
workspace tests and the Java tests.

Commit messages follow the conventional `type (scope): subject` format
described in `.cmt` (types feat, fix, docs, style, refactor, test, chore).
`.cmt` is the configuration of [cmt](https://github.com/smallhadroncollider/cmt),
a commit formatting tool that prompts for these parts.

## Test resources

`translator_new_rml` carries the regression suite against the official RML
conformance test cases. `resources/test/` has one directory per spec module
(`rml-core-tests`, `rml-lv-tests`, `rml-io-tests`, `rml-cc-tests`,
`rml-fnml-tests`, `rml-star-tests`, plus `rml-lv-fnml` and `rmlstar`), and
inside it one directory per test case (e.g. `RMLTC0000-JSON/`) holding the
test case files (`mapping.ttl`, input data, `output.nq`, `README.md`) and the
expected plan `mapping.json`.

The tests live in `src/translator/regression_tests/` (one file per module:
`core.rs`, `logical_view.rs`, `io.rs`, `cc.rs`, `fnml.rs`, `star.rs`,
`rml_lv_fnml.rs`). Each test calls
`TestExecutor::new(<module dir>).execute(<case>, <positive>)`: a positive case
translates `mapping.ttl` and compares the plan with `mapping.json`; a negative
case expects the translation to fail. The comparison hashes the sorted
characters of the re-serialized JSON, so it is independent of key order and of
the order of strings inside templates. The `mapping.json` files are fixtures:
a change to one goes through the maintainer as stated in the contract.

`translator_rml/resources/` holds RML v1 test mappings (`csv-testcases/`,
`rml/`, `rmlmapper-custom/`, `rmlstreamer/`), and `translator_shexml/resources/test/`
holds ShExML inputs. Each translator crate has a `test_case!` macro that
resolves a file name against its `resources/` directory via
`CARGO_MANIFEST_DIR`.

## Language bindings

The `translator` crate builds as `cdylib` and `rlib`. Cargo features enable the
bindings: `jni` (Java), `pyo3` (Python) and `neon` (Node.js). The binding
code lives in `crates/translator/src/java/` (Maven project `algemaploom`),
`crates/translator/src/python/` and `crates/translator/src/nodejs/`. The
scripts `build_java.sh`, `build_python.sh` and `build_nodejs.sh` build each
binding; the Java build cross-compiles for Linux, Windows (MinGW) and macOS
targets.

## Release process

Step-by-step instructions are in [RELEASE.md](RELEASE.md); this section explains the tooling.

`./bump-version.sh <version>` performs a release:

1. sets the version in `Cargo.toml` and runs `cargo check` to update `Cargo.lock`;
2. sets the version in the Java `pom.xml` (`mvn versions:set`) and in `package.json`;
3. optionally adds the version to `CHANGELOG.md` with `changefrog`;
4. optionally commits, tags `v<version>` (or the bare version for `testrelease-*`) and pushes;
5. after a pushed release, sets the Java `pom.xml` to the next patch `-SNAPSHOT` (e.g. `0.8.1-SNAPSHOT` after `0.8.0`), and commits and pushes that as "Prepare for next development cycle".

Between releases the Java binding carries that `-SNAPSHOT` version, as the
KNoWS Java libraries do, so a locally installed MappingLoom stays apart from
the released artifact in `~/.m2`. `Cargo.toml` and `package.json` keep the
released version until the next release; development versions for the Python
and Node.js packages are still to be worked out.

A pushed tag triggers the `Maven Central Deployment` CI job, which signs and
deploys the Java package from `crates/translator/src/java/algemaploom`.
