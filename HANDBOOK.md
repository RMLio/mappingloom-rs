# MappingLoom (algemaploom-rs) Handbook

Written for a CS student who wants to understand MappingLoom as code.

## Contents

- [MappingLoom (algemaploom-rs) Handbook](#mappingloom-algemaploom-rs-handbook)
  - [Contents](#contents)
  - [Preface](#preface)
  - [Agent request contract (for AI agents/LLMs)](#agent-request-contract-for-ai-agentsllms)
  - [Workspace layout](#workspace-layout)
  - [How a mapping becomes a plan](#how-a-mapping-becomes-a-plan)
  - [Requests, results and errors](#requests-results-and-errors)
  - [The canonical pipeline](#the-canonical-pipeline)
  - [Build, format and test](#build-format-and-test)
  - [Test resources](#test-resources)
  - [Conformance](#conformance)
  - [Language bindings](#language-bindings)
  - [Release process](#release-process)

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
- CLIs: the `translator-cli` crate builds `translator-bin`
  (`crates/translator-cli/src/bin/translator-bin/`) with the subcommands `file`,
  `folder` and `stdin`, the request options `--language`, `--base-iri`,
  `--optimizer` and `--options`, and the flags `-d` (debug logging), `-j`
  (JSON only), `-p` (pretty-print) and `--report` (see Requests, results and
  errors). It also declares `sparql-rml-pruner`, which is a placeholder
  (`unimplemented!`). The `normalizer` crate has an example CLI,
  `cli_normalizer`.
- Tests: Rust unit tests and plan snapshots inside the crates (`#[test]`), with
  test resources under each crate's `resources/` directory; tests that run
  `translator-bin` itself are in `crates/translator-cli/tests/`; the Java binding
  has Maven tests under `crates/translator/src/java/algemaploom`.
- The project keeps a `README.md` (usage, covered specs, bindings) and a
  `CHANGELOG.md` in Keep a Changelog format.

## Agent request contract (for AI agents/LLMs)

<!-- software-handbook contract: 2026-11-18 -->

Every implementation request handled by an AI agent/LLM follows these constraints:

- If the request is a feature or bugfix:
  - fix the specific failing case or issue named in the request;
  - preserve existing passing behavior unless explicitly asked not to;
  - cover the change with a test: extend an existing test where one fits, add one only where none does, and remove the tests of removed or deprecated behavior.
- Leave staging and committing to the maintainer. Edit staged files freely: the maintainer stages what they reviewed, so the unstaged diff shows the new changes.
- Make one coherent patch per request: everything the request needs (code, tests, docs, changelog) and nothing unrelated. A documentation error found along the way is fixed in the same patch.
- Leave the code leaner after every request: remove what the change makes redundant (duplicate tests, parameters and options that no longer do anything, helpers that duplicate each other, comments that only repeat the code), and reuse shared functionality instead of adding a local variant. Run `cargo clippy --workspace` to find unused code.
- Fix a transient environment problem (a stale PATH, a shell or editor that needs a restart) in the environment, by restarting or reconfiguring it; add no code that works around it.
- **Push back** when a request would violate an established principle (e.g. breaking test hermeticity). Explain the principle and suggest a documentation-only fix instead of silently implementing the harmful change.
- Keep necessary internal implementation guidance and rationale in this handbook; put end-user and developer-facing setup, usage and workflows in the README. Give each fact one home in the repo and cross-reference rather than duplicate it. Code and docs stand on their own: they reference published sources (such as specifications) and no planning notes or files outside the repo.
  - Update only the documentation affected by the change, integrated into the relevant existing section.
  - This contract holds only general rules for handling a request; project-specific guidance goes in the chapter on that topic.
- Do not stop at making tests green; align the implementation with the specification or intended design, and document necessary semantic rationale in this handbook.
- Update existing tests (code or fixtures) in the same patch when the change requires it, and list each update with its reason in the hand-over. Never weaken a test only to make it pass.
- Update `CHANGELOG.md` for every change, internal ones included (tests, CI, refactoring, removed code): keep `## Unreleased` a short summary of what changed since the last release. A feature that is new since the last release is one Added line, which later fixes update instead of getting lines of their own. Each line covers one concern; changes of the same concern share a line (such as editorial changes, or updated dependency versions). A line states what is true now, without the old behavior; a fix that only removes a divergence that should not have existed (one part behaving unlike the rest) gets a few words in the line of its concern at most.
- Before handing over, review the patch as the maintainer would (correctness, leaner code and docs, changelog) and fix what you find, so the maintainer's review is the last round.
- Before a release, review everything changed since the previous release (the code for correctness, the documentation and changelog for accuracy and brevity), check the dependencies (undeclared, outdated, vulnerable), and list the open items that change the released artifact, so they ship in this release.
- Hand over in one short message: what is ready (with a commit message), what needs the maintainer's judgement (each non-obvious removal or changed test, with its evidence), what was verified (commands and results), and the maintainer's next actions in order. Details belong in the changelog and docs.
- Check whether `README.md` needs updates for user-visible behavior or workflow changes, and update it when needed.
- Write documentation (this handbook, READMEs, `TODO.md`, `CHANGELOG.md`, code comments) as plain positive statements: say what is true and leave out the contrast ("X, not Y"). Keep a negative only when it is the point itself, such as a prohibition, a warning, or a known limitation. Break the lines of code comments and Markdown at sentence and clause boundaries; a line is as long as its clause.
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
| `translator` | The front end: the `api` module (`translate`, `translate_json`), the request and result types, and the Java/Python/Node.js bindings. |
| `translator-cli` | The command line: `translator-bin` and the placeholder `sparql-rml-pruner`, with the CLI's dependencies (clap, the logger of `common`). |
| `common` | Shared utilities such as the logger. |
| `normalizer` | Normalizes a mapping with SPARQL Update rules over an Oxigraph store: the RML-KGC rule set of the canonical pipeline, and the paper's RML.io rule set (see The canonical pipeline). |
| `translator_normalized_rml` | Translator for normalized RML documents. |
| `translator_kgc` | The canonical RML-KGC translator: normalizes a mapping and translates the normal form into an `algemaploom-plan` plan (see The canonical pipeline). |
| `algemaploom-plan` | The plans of the canonical pipeline: their operators and expressions, the plan document and its JSON Schema (see The canonical pipeline). |
| `optimizer` | The optimizer of the canonical pipeline: the `Rule` trait, the rules in the order they run, and their toggles (see The canonical pipeline). |
| `pipeline-test` | Test harness of the canonical pipeline: graph cases for normalizer rules, and plan snapshots validated against the plan schema. A dev-dependency only. |
| `sparql-sat-checker` | Checks SPARQL/triples-map satisfiability on plans; uses `translator_normalized_rml`. It carries its own version (`0.1.0`) and edition 2024. Unmaintained and kept for reference: nothing calls it, and its crate documentation lists its known defects. |

## How a mapping becomes a plan

`translator::api::translate` takes the mapping and the request options, and
the language that the options name picks the translator: `rml-kgc` goes to
`translator_new_rml` (current RML spec), `rml-io` goes to `translator_rml`
(RML v1), and `shexml` goes to `translator_shexml`. `r2rml` is
*unsupported*, since `translator_rml` reads no logical tables. The caller
always names the language, since the languages differ in subtle ways that a
guess from the text would hide.

A Turtle mapping first passes a syntax check (`oxttl`), which reports every
syntax error with its location (the translators' own parsers report none)
and reads the document's `@base`. Relative IRIs in the mapping document
resolve against `@base` alone, as Turtle defines, so a relative IRI in a
document without `@base` is a syntax error.

The base IRI option is the processor's base IRI: every triples map without
`rml:baseIRI` takes it, and without it the document's `@base`. With neither,
the plan carries an empty base IRI, and the engine fails only on a generated
IRI that needs one.

`translator_new_rml` first indexes the document in a `SearchStore`
(`translator/store.rs`): triples maps and subject maps by identifier, a quad
variable per term map, and one sourced plan per group of effectively equal
logical sources. Each operator translator implements one of two traits:
`OperatorTranslator::translate` builds an operator from its input alone (source,
iterator), and `StoreOperatorTranslator::translate_with_store` also looks up
other mapping parts in the store (extend, join, serializer).

The serializer of one source holds the triple patterns of all its triples maps,
as a template of N-Quads lines. Graph membership is decided per
predicate-object map, as RML-KGC defines it: a map's triples go to every graph
map of the subject map together with every graph map of that predicate-object
map, and to the default graph when there is none (`rml:defaultGraph` also names
the default graph). The class triples of a subject map follow the subject map's
graph maps only. `serializer/mod.rs` tests this with a mapping whose
predicate-object maps differ in their graph maps.

The CLI writes the plan next to the input (or with the derived output prefix)
as DOT and JSON; `-j` limits output to JSON. `folder` translates the `.ttl`
files under the folder, or the `.shexml` files for ShExML. `stdin` prints the
JSON plan to standard output. The Java binding's `Cli` does the same for a
file argument or standard input. Both read the mapping as a whole, line breaks
included, because a Turtle comment ends at the end of its line.

## Requests, results and errors

Every entry point makes the same request: the mapping text and an options
object (`request::Options`), which is JSON in the bindings and in
`translator-bin --options`:

| Key | Value |
|---|---|
| `language` | `rml-kgc`, `rml-io`, `r2rml` or `shexml`; required |
| `base_iri` | the processor's base IRI (see How a mapping becomes a plan) |
| `optimizer` | an object from rule id to `true` or `false`; a rule without a toggle takes its default |
| `translator` | `legacy` (default) or `canonical`, the canonical pipeline; `canonical` for `rml-kgc` only |

`api::translate` returns a `Translation` (the plan and the warnings)
or `TranslationErrors` (at least one error, and the warnings found before).
A panic inside it becomes an *internal* error, so no binding sees a panic.
`api::translate_json` is the entry point of the bindings:
it takes the options as JSON text and returns the result document as JSON text,
either `{"version", "plan", "warnings"}` or `{"version", "errors", "warnings"}`,
where `version` is the plan's version (`algemaploom_plan::VERSION`).
One JSON encoding, tested once in Rust, serves the three bindings,
and a new option or error field changes no native signature.

An error or a warning is a `report::Record`: `kind`, `code` (stable within
its kind), `message`, and where known `origin` (the mapping node, as an IRI
or skolem IRI), `location` (line and column, from 1) and `causes` (the
underlying messages, outermost first). The kinds and their codes:

| Kind | Code | Meaning |
|---|---|---|
| request | `invalid-options` | the options are no JSON object, or have an unknown key or value, or no language, or name the canonical translator for a language other than `rml-kgc` |
| request | `invalid-base-iri` | `base_iri` is no absolute IRI |
| request | `unknown-rule` | an optimizer toggle names no rule |
| request | `ignored-option` (warning) | `base_iri` for ShExML, which has no base IRI |
| syntax | `invalid-turtle` | the mapping is not valid Turtle; one record per error, with its location (`api` checks the syntax first; `translator_kgc` called directly reports it without location) |
| syntax | `invalid-shexml` | the mapping is not valid ShExML |
| mapping | `invalid-mapping` | a legacy translator rejected the mapping; its messages are the causes |
| mapping | `subject-maps`, `predicate-object-map`, `expressions`, `invalid-template`, `invalid-constant`, `constant-blank-node`, `invalid-term-type`, `no-expression`, `invalid-reference`, `datatype-and-language`, `datatype-or-language-without-literal`, `reference-without-logical-source` | the canonical translator rejected the mapping; the record's `origin` is the term map or triples map |
| unsupported | `feature`, `source` | the canonical translator doesn't translate a join, function execution, gather map, star map, logical target, logical view or source other than a relative path yet |
| unsupported | `unsupported-language` | the language has no translator yet (`r2rml`) |
| internal | `panic` | the translator panicked |
| internal | `normalizer` | the normalizer failed on a valid Turtle document |

The *unsupported* kind (a valid mapping that the translator doesn't handle
yet) stays apart from *mapping*, so that a conformance run can tell a real
pass of an error case from a translator that gave up. `translator-bin` exits
with 0 for a plan, 1 for *syntax*, *mapping* and *unsupported*, 2 for usage
errors and *request*, and 3 for *internal*; `folder` exits with the highest
code among its files.

## The canonical pipeline

The canonical pipeline replaces `translator_new_rml` for RML-KGC in three
stages: normalize, translate, optimize. It is built next to the legacy
translators, and a request with `"translator": "canonical"` takes it. Its normalizer follows the normal form
of Min & Hartig (Appendix A), adapted to RML-KGC.

**Normalizer.** `normalizer::normalize_kgc(mapping, base_iri)`, with the
processor's base IRI if there is one, applies the
rules of `kgc::RULES` in order and returns a `NormalForm`: the normalized
graph, and the skolem IRI of each blank node of the original document. Each
rule is one SPARQL Update request, except `same-source-joins`, which compares
logical sources by value recursively, something SPARQL can't do to any
depth. A rule whose id starts with a number is that step of the paper. The
rules, in order:

| Rule | What it does |
|---|---|
| `2-constant-shortcuts`, `datatype-and-language-shortcuts`, `join-condition-shortcuts` | expand the shortcuts: `rml:subject`, `rml:predicate`, `rml:object`, `rml:graph`, `rml:datatype`, `rml:language`, `rml:child`, `rml:parent` |
| `term-types` | make every term type explicit: IRI, or Literal for an object map that is reference-valued, function-valued or has a datatype or language map; a constant keeps none |
| `1-classes` | each class of a subject map becomes a predicate-object map |
| `same-source-joins` | a referencing object map without join condition over an effectively equal logical source takes the parent subject map's expression and term type |
| `3-split-predicate-object-maps` | one predicate-object map per predicate map, object map and graph map, the subject map's graph maps included |
| `subject-graph-maps` | removes the subject map's graph maps, which `3-split-predicate-object-maps` gave every predicate-object map |
| `base-iris` | every triples map gets an `rml:baseIRI`, by default the processor's |
| `5-split-triples-maps` | one triples map per predicate-object map; the original keeps its logical source and subject map for the joins that name it |

`3-split-predicate-object-maps` takes the subject map's graph maps per
triples map, so a predicate-object map that two triples maps share gets the
graphs of each one only, and it removes the predicate-object maps it splits.
`same-source-joins` inlines an object map only when every triples map that
uses it reads a logical source equal to its parent's: the same node, or one
whose description is equal ignoring `rdf:type`. It links equal sources with
`nf:sameSource` while it runs and removes the links before it ends. A
normalized triples map is a node with one `rml:predicateObjectMap`; a
triples map without one produces no triples, and stays for the joins that
name it as parent.

The rules add links in the namespace `NF`
(`https://w3id.org/imec/knows/ns/rml/normal-form#`), which only the
pipeline reads:
- `nf:origin` links a node that a rule creates to the node of the original
  document it stands for: a copy to its original, a shortcut's term map to
  the node that held the shortcut, a class's predicate-object map to its
  subject map. A copy of a created node links to the original node, so one
  hop always reaches the original document. A blank node of the original
  document is named by its skolem IRI in `GENID`
  (`https://github.com/RMLio/algemaploom-rs/.well-known/genid/`), from its
  canonical label (RDFC-1.0), so the IRI depends on the document alone and
  means the same in every file and plan. The skolem IRI carries the blank
  node's label as `nf:label`, for reading only; an anonymous node's label is
  the parser's random id, so a test case labels every blank node that an
  `nf:origin` link names.
- `nf:parentTriplesMap` marks an object map that `same-source-joins`
  inlined, so it is evaluated as its parent's subject map: with the parent's
  base IRI and the parent's fresh blank nodes.

**Translator.** `translator_kgc::translate` normalizes the mapping
and translates each normalized triples map into one branch:
a Source,
an Extend that binds `subject`, `predicate`, `object` and, unless the graph is the default one, `graph`,
a Project
and a Serializer with one quad pattern, which writes to the default Target (Algorithm 1).
A branch has its own Serializer,
so that a quad pattern without graph is the default graph
and a branch can route to its own targets.
The Source has one field per distinct query of the branch's term maps (Algorithm 2), named `q:<query>`;
a CSV column and a JSONPath without wildcard, recursive descent, filter, union or slice are single-valued.
A term map is an expression (Algorithm 3):
a constant is the complete term,
a template keeps its pieces and the encoding of its term type (IRI-safe for IRI, URI-safe for URI, none otherwise),
a term map with term type BlankNode and no expression is a fresh blank node keyed by the term map's origin,
and the term type wraps the rest:
`Iri` or `Uri` with the triples map's base IRI, `BlankNode`, or `Literal` with its datatype or language map.
An object map that `same-source-joins` inlined takes the base IRI and the fresh blank node key of its parent.
A term map with an RML property that the translator doesn't read is an *unsupported* error,
so that no feature is dropped silently
(properties of other vocabularies are annotations);
a triples map without predicate-object map gives no branch, but its subject map is checked too.

Algorithms 1–3 give one attribute per query and one value per attribute,
so a query that feeds two term maps correlates their values,
and a query without value drops the record.
RML-KGC asks for more, and the plans differ from the paper in these points:
- An attribute is bound to the *list* of its query's values,
  and a Serializer takes the product of its positions' lists:
  several subjects per record give subjects × predicates × objects (core `termmap.md:71`, RMLTC0025c).
  A query without value binds the empty list, which empties only the term maps that read it.
- A template takes the product per brace,
  so a repeated reference takes every combination of its values.
  Each value enters in its natural RDF lexical form (core `expressions.md:54`),
  encoded by the term type (core `termmap.md:124-138`).
  `\{`, `\}` and `\\` are literal characters;
  any other brace or backslash makes the template invalid, a mapping error.
- A language or datatype map gives one literal per tag or datatype.
- A generated IRI that is not valid is a data error (core `output.md:131`).
- Join keys compare by lexical form,
  as RMLTC0030c needs (it joins the JSON number 100 with the constant `"100"`);
  an absent key never matches,
  and a multi-valued key matches when any of its values does.
- An empty string is a value;
  only the format's nulls and the `rml:null` values are absent (RMLSTC0004a, RMLFNMLTC0007-CSV).

A referencing object map that `same-source-joins` left
(one with join conditions, or over another logical source)
becomes a Join, as in Algorithm 1:
the child's Extend binds the subject, predicate and graph and the child map of each join condition (`key:<i>`);
the parent's Source, with its attributes prefixed `parent:` (its fields by name, `#` by a Rename),
and an Extend of the parent maps (`parent:key:<i>`) are its Right input;
the Inner Join compares the keys pairwise, by lexical form,
and an Extend after it binds the object from the parent's subject map,
with the parent's base IRI and its fresh blank nodes indexed by `parent:#`,
so that they are the parent's own subjects.
A join condition's maps are expression maps without term type.
Without join conditions the Join has no pairs: the cartesian product of child and parent records.
The normal form has no order,
so the join conditions, the branches and the errors are sorted by content,
which gives the same plan on every run.
The translator checks every term map and source of every triples map,
and reports each error once, also when the copies of a triples map share it.
A term map stops at its first error.

**Optimizer.** The `optimizer` crate rewrites an `algemaploom-plan` plan with independent rules,
each an implementation of `Rule` with an id (its toggle's key, such as `R1`) and a default.
`optimizer::rules` lists them in the order they run,
and `Optimizer::new` switches each on or off from the `optimizer` option, rejecting an unknown id.
It has no rules yet, so a plan passes unchanged;
`api::translate` optimizes every canonical plan.

The processor's base IRI enters the store as `nf:processor rml:baseIRI <…>` for `base-iris`,
and leaves it after the last rule.
The RML.io rule set (`normalize_rml_str`, `normalize_rml_path`, the `cli_normalizer` example)
stays as it is for when RML.io is taken up.

**Plans.** The `algemaploom-plan` crate holds the core of the plans:
a `Plan` is a petgraph graph of `Operator`s that the builder adds without fusing anything,
and `Plan::document` gives the `PlanDocument` that travels to an engine,
with its `version` and edges that name their nodes by id.
The plans name no mapping language:
an Extend binds attributes to `Expression`s (references, templates, constants and term constructors),
a quad pattern without graph goes to the default graph,
and an origin is the list of IRIs of the mapping nodes an element comes from (skolem IRIs for blank nodes),
for provenance only.
A translator expresses its language's semantics in these terms.
The plans are internal to algemaploom-rs for now,
so `VERSION`, the plan's `version`, is the crate version.
From algemaploom-rs 1.0 on, the plans may get a version of their own,
once they need to evolve apart from the crate.
The document says what an element is by its keys where those differ, and by `type` where they don't:
a quad position is `{"attribute": …}` or `{"term": …}`,
a term `{"iri": …}` or `{"literal": …}`,
and an operator, an expression or an access carries its `type`.
`schema()` generates the JSON Schema from the types (`schemars`, which follows their serde attributes),
and `schema/plan.schema.json` is that schema, committed.
An optional field carries `#[serde(default, skip_serializing_if = …)]`,
since schemars marks a field without `default` as required.

## Build, format and test


- Build: `cargo build` (CI) or `cargo build --release` for the CLI in
  `target/release/translator-bin`.
- Test: `cargo test --verbose --jobs 1 --workspace` (the CI command).
- The dev profile builds the dependencies and `translator` with
  `opt-level = 1`: unoptimized, they share their generic instantiations, and
  the bindings' library then exports more symbols than a Windows (GNU) DLL
  can hold, so the workspace build fails to link it.
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
workspace tests and the Java tests, and holds the manual `conformance` job
(see Conformance).

Commit messages follow the conventional `type (scope): subject` format
described in `.cmt` (types feat, fix, docs, style, refactor, test, chore).
`.cmt` is the configuration of [cmt](https://github.com/smallhadroncollider/cmt),
a commit formatting tool that prompts for these parts.

## Test resources

`translator_new_rml` carries plan snapshots of the kg-construct conformance
test cases. `resources/test/` has one directory per spec module
(`rml-core-tests`, `rml-lv-tests`, `rml-io-tests`, `rml-cc-tests`,
`rml-fnml-tests`, `rml-star-tests`, plus `rml-lv-fnml` and `rmlstar`), and
inside it one directory per test case (e.g. `RMLTC0000-JSON/`) holding the
test case files (`mapping.ttl`, input data, `output.nq`, `README.md`) and the
recorded plan `mapping.json`.

The snapshot tests live in `src/translator/plan_snapshots/` (one file per
module: `core.rs`, `logical_view.rs`, `io.rs`, `cc.rs`, `fnml.rs`, `star.rs`,
`rml_lv_fnml.rs`). Each test calls
`TestExecutor::new(<module dir>).execute(<case>, <positive>)`: a positive case
translates `mapping.ttl` and compares the plan with `mapping.json`; a negative
case expects the translation to fail. The comparison hashes the sorted
characters of the re-serialized JSON, so it is independent of key order and of
the order of strings inside templates, which `translator_new_rml` varies
between runs; it is also blind to any change that keeps the same characters.

A snapshot records what the translator emitted when it was taken, so a
passing snapshot test says that the plan did not change, never that the plan
is right. Whether a plan is right is decided by running it: the conformance
run of MappingWeaver on the kg-construct suites (see Conformance). The
`mapping.json` files are fixtures: a change to one goes through the maintainer
as stated in the contract, with the reason the plan changed.

The canonical pipeline's tests use the `pipeline-test` harness:
- A normalizer rule has its cases in `normalizer/resources/test/kgc/<rule
  id>/`: a case is a folder with `input.ttl` and `expected.ttl`, and a rule
  with several cases has one subfolder per case. The rule's test applies the
  rule alone to the input and compares the result with the expected graph
  by isomorphism; `kgc/rule-set/` runs the whole rule set. The Turtle inlines
  every blank node it can, and labels only those that are named more than
  once or by an `nf:origin` link, whose `nf:label` is that label.
- A plan snapshot (`assert_plan_snapshot`) is the plan document as JSON
  without `version`, compared as JSON, after the whole document is
  validated against the plan schema. `UPDATE_SNAPSHOTS=1` rewrites the
  snapshots, and `UPDATE_SCHEMA=1 cargo test -p
  algemaploom-plan` rewrites the committed schema; review either diff as a
  fixture change.

`translator_kgc/tests/core.rs` translates every RML-Core case of
`translator_new_rml`'s `resources/test/rml-core-tests/` with the canonical
translator, and compares the plan, or the errors, with its snapshot in
`translator_kgc/tests/core/<case>.json` (`UPDATE_SNAPSHOTS=1` rewrites
them).

`translator_rml/resources/` holds RML v1 test mappings (`csv-testcases/`,
`rml/`, `rmlmapper-custom/`, `rmlstreamer/`), and `translator_shexml/resources/test/`
holds ShExML inputs. Each translator crate has a `test_case!` macro that
resolves a file name against its `resources/` directory via
`CARGO_MANIFEST_DIR`.

## Conformance

A plan is right when executing it gives the RDF that the specification
requires, so conformance is measured on an engine: MappingWeaver, which
executes the plans of this translator. MappingWeaver's
`conformance/run.sh` runs the kg-construct test suites with
mapping-test-runner on its jar, and compares the outcome of every case with
its `conformance/expected-results.csv` (MappingWeaver's HANDBOOK, Conformance).

The manual `conformance` CI job runs that script with this translator: it
builds and installs the Java binding (`build_java.sh`), clones MappingWeaver
from its GitHub mirror at `MAPPINGWEAVER_REF` (default `development`), builds
it with `-Dmappingloom.version=<the binding's version>`, and runs the suites,
with the database cases on Docker-in-Docker. A case that does worse than
MappingWeaver's expectations is a regression and fails the job; a case that
does better is reported as a warning (`MTR_UNEXPECTED_PASS=warn`), since the
expectations belong to MappingWeaver: the fix that makes it pass also removes
its line there, in a MappingWeaver change that follows this translator's
release. The job keeps the reports (`report.md`, JUnit XML) and the engine
logs of each case.

Run it for a change that affects the plans of RML-KGC mappings, and put the
cases whose outcome changed in the merge request. Locally, the same run is
`./build_java.sh`, then `conformance/run.sh -Dmappingloom.version=<version>`
in a MappingWeaver checkout.

## Language bindings

The `translator` crate builds as `cdylib` and `rlib`. Cargo features enable the
bindings: `jni` (Java), `pyo3` (Python) and `neon` (Node.js). The CLI lives
in `translator-cli`, so the library leaves out its logger (log4rs), which
links Apple's CoreFoundation framework through chrono: the library links only
system libraries. The binding code lives in `crates/translator/src/java/` (Maven project
`algemaploom`), `crates/translator/src/python/` and
`crates/translator/src/nodejs/`, and each calls `api::translate`. The
scripts `build_java.sh`, `build_python.sh` and `build_nodejs.sh` build each
binding.

The Java build (`build_native.sh`, run by Maven) cross-compiles the library
for Linux, Windows (MinGW) and macOS (x86_64 and aarch64), and stops at the
first failure, so a jar never lacks one. Off a Mac it builds the macOS
libraries with cargo-zigbuild, which links them with zig and needs no Apple
SDK, since the library links no Apple framework; `NATIVE_APPLE=false` skips
them, for a jar that is only tested on Linux. CI installs these toolchains in
the hidden jobs `.native-targets` (MinGW) and `.apple-targets` (zig), which
the jobs that build the Java binding reference; only the jobs whose jar
ships reference `.apple-targets`. The Java wrapper calls the native method
`translateJson`; a jar and a native library of different versions then fail
on the first call with an `UnsatisfiedLinkError`, since a native method of
another name has another JNI symbol.

The Python module is `ltranslator` (`build_python.sh` installs it under that
name, next to the `translator.py` script), with `__version__`. The Node.js
binding throws an `Error` whose `name` is `TranslationError`. On Windows, the
Node.js module loads but exports nothing; it is built and tested on Linux. The Java jar also carries the plan schema, as
`algemaploom-plan/plan.schema.json`, for consumers that validate plans.

## Release process

Step-by-step instructions are in [RELEASE.md](RELEASE.md); this section explains the tooling.

`./bump-version.sh <version>` performs a release:

1. sets the version in `Cargo.toml` and runs `cargo update --workspace` to update `Cargo.lock`;
2. sets the version in the Java `pom.xml` (`mvn versions:set`), and in `package.json` and `package-lock.json` (`npm version`);
3. optionally adds the version to `CHANGELOG.md` with `changefrog`;
4. optionally commits, tags `v<version>` (or the bare version for `testrelease-*`) and pushes;
5. after a pushed release, sets the same files to the next patch `-SNAPSHOT` (e.g. `0.9.1-SNAPSHOT` after `0.9.0`), and commits and pushes that as "Prepare for next development cycle".

Between releases the crates, the Java binding and the Node.js package carry
that `-SNAPSHOT` version, as the KNoWS Java libraries do, so a locally
installed MappingLoom stays apart from the released artifact in `~/.m2`, and
a plan's `version`, `translator-bin --version` and the Python module's
`__version__` name a development build.

A pushed tag triggers the `Maven Central Deployment` CI job, which signs and
deploys the Java package from `crates/translator/src/java/algemaploom`.
