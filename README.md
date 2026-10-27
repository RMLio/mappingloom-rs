<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->

<a name="readme-top"></a>

<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->

<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">MappingLoom: Weaving Mapping Languages with Algebraic Operators</h3>
</div>

<!-- ABOUT THE PROJECT -->

## About The Project

Mapping algebra provides operational semantics to the mapping process, opening
the door to study of complexity and expressiveness of existing mapping languages.
This project provides the CLI translator from RML and ShExML to mapping algebra.

<!-- GETTING STARTED -->

## Prerequisites

To compile the project on your own, you'll need to have
[Rust toolchain](https://www.rust-lang.org/tools/install) installed.

For Linux-based users:

- Rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

For the visualization of the generated mapping plans, you'll need
python version >= 3.10 and the following packages:

1. graphviz

## Running

1. Download this repo
2. Run cargo build at the root this repo
   ```sh
   cd {repo dir}
   cargo build --release
   ```
3. Run the CLI translator app from the compiled translator binary, with the
   language of the mapping (`rml-kgc`, `rml-io`, `r2rml` or `shexml`)
   ```sh
   cd ./target/release/
   ./translator-bin --language rml-kgc file <RML_DOCUMENT>
   ```
   `--base-iri` sets the base IRI of every triples map without `rml:baseIRI`
   (by default the mapping's `@base`), and `--options <FILE>` reads these
   options from a JSON object such as `{"language": "rml-kgc", "base_iri":
   "http://example.com/"}`. The exit code is 0 for a plan, 1 when the mapping
   can't be translated, 2 for usage errors and invalid options, and 3 for a
   bug in the translator. The errors go to standard error; with `--report`,
   the JSON output is the result document, with the plan or the errors, and
   the warnings.
   For more information/options of CLI app:
   ```sh
   ./translator-bin  -h
   ```
4. Visualize the created mapping plan
   ```sh
   dot -Tpng {generated dot file} > output.png
   ```
5. Simple plain text format of the mapping plan for parsing
   ```sh
   dot -Tplain {generated dot file} > output.txt
   ```
   <p align="right">(<a href="#readme-top">back to top</a>)</p>

## Test cases


### RML 
The translator keeps a plan snapshot of each official RML conformance
[test case](https://github.com/kg-construct) it covers: a test translates the
case's mapping and compares the plan with the stored one, so a change in the
plan shows up. The following specifications are covered: RML-Core, RML-LV
(logical views), RML-IO, RML-CC (collections & containers), RML-FNML
(functions) and RML-STAR (RDF-star). Whether the plans are right is checked by
running them in MappingWeaver on the conformance suites (the `conformance` CI
job, see [HANDBOOK.md](HANDBOOK.md#conformance)).

The test cases and their plan snapshots live under
[crates/translator_new_rml/resources/test](/crates/translator_new_rml/resources/test),
one directory per spec module (`rml-core-tests`, `rml-lv-tests`, `rml-io-tests`,
`rml-cc-tests`, `rml-fnml-tests`, `rml-star-tests`). The snapshot tests
themselves are in
[crates/translator_new_rml/src/translator/plan_snapshots](/crates/translator_new_rml/src/translator/plan_snapshots).

### ShExML
The translator can *partially* translate ShExML documents. 
The translator will make a **best-effort** translation if the ShExML document
uses the following *unsupported* features. 
It will still generate a mapping plan which could be executed but the 
results won't be complete. 

The following features are not supported in translation yet: 

1) Autoincrements
2) Query statements
3) Joins 
4) Functions 
5) Conditionals

## Bindings
MappingLoom provides bindings for Java, Python and Node.js.
These can be enabled with the features `jni`, `pyo3` and `neon` respectively.

Each binding takes the mapping and the options, with the language required,
and returns the plan as JSON text and the warnings, or raises an error that
holds the error records:

| Binding | Call | Error |
|---|---|---|
| Java (`be.ugent.idlab.knows:MappingLoom`) | `ITranslator.getInstance().translate(mapping, new Options("rml-kgc"))` returns a `Translation` (`plan()`, `warnings()`) | `TranslationException` (`errors()`, `warnings()`) |
| Python (`ltranslator`) | `translate(mapping, {"language": "rml-kgc"})` returns a `Translation` (`.plan`, `.warnings`) | `TranslationError` (`.errors`, `.warnings`) |
| Node.js | `translate(mapping, {language: "rml-kgc"})` returns `{plan, warnings}` | an `Error` named `TranslationError` (`.errors`, `.warnings`) |

If you build from source, you can run the `build_java.sh`, `build_python.sh` and `build_nodejs.sh`
scripts respectively.

The Java jar carries the native library for Linux, Windows and macOS (x86_64
and aarch64). To cross-compile them from Linux you need the MinGW
cross-compiler, [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
with zig (`pip install cargo-zigbuild ziglang`), and these build targets:
- `x86_64-unknown-linux-gnu`
- `x86_64-pc-windows-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

For examples, check out the command line scripts in [crates/translator/src/java](crates/translator/src/java), [crates/translator/src/python](crates/translator/src/python), and [crates/translator/src/nodejs](crates/translator/src/nodejs).

## Acknowledgement

This software makes use of [sophia_rs](https://github.com/pchampin/sophia_rs) crate!

<p align="right">(<a href="#readme-top">back to top</a>)</p>
