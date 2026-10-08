# Releasing algemaploom-rs (MappingLoom)

Step-by-step instructions for publishing a release. [HANDBOOK.md](HANDBOOK.md) (Release process) explains how the tooling works.

## Before you start

- bash (Git Bash on Windows), Maven, Rust with `cargo` (rustup installs the toolchain pinned in `rust-toolchain.toml`; the script runs `cargo check` to update `Cargo.lock`), Java 21, and `changefrog` (`npm install -g changefrog`), which writes the version section of `CHANGELOG.md`.
- Push access to `origin` (https://gitlab.ilabt.imec.be/rml/proc/algemaploom-rs).
- You are on `development`, up to date with `origin/development`, with a clean working tree.
- The tests pass: `cargo test --workspace` and `./test_java.sh` (the Java binding). `./test_java.sh` builds the native libraries with `build_native.sh`, so run it on Linux or WSL.
- `## Unreleased` in `CHANGELOG.md` lists everything since the last release.
- Pick the version with [Semantic Versioning](https://semver.org/). The Java `pom.xml` holds the next patch as `-SNAPSHOT` (e.g. `0.8.1-SNAPSHOT`): release that patch, or a higher minor or major version when the changelog has new features or breaking changes.

## Release

1. Run `./bump-version.sh 0.8.1` (with your version) and answer `y` to both questions. The script
   - sets the version in `Cargo.toml`, `Cargo.lock`, the Java `pom.xml` and `package.json`;
   - turns `## Unreleased` into the version section of `CHANGELOG.md`;
   - commits "Update version to <version>", pushes `development`, and creates and pushes the tag `v<version>` (e.g. `v0.8.1`);
   - moves the Java `pom.xml` to the next patch `-SNAPSHOT`, and commits and pushes "Prepare for next development cycle".
2. Move `main` to the release: `git push origin v0.8.1^{commit}:main`. `main` always points at the latest release; the push succeeds only as a fast-forward.
3. The tag pipeline (https://gitlab.ilabt.imec.be/rml/proc/algemaploom-rs/-/pipelines) runs the `Maven Central Deployment` job, which builds the Java binding with the native libraries for Linux and Windows (`build_native.sh` builds the macOS ones only on macOS), signs it and deploys it to Maven Central. That job may fail without failing the pipeline, so check the job itself; the new version then appears at https://repo1.maven.org/maven2/be/ugent/idlab/knows/MappingLoom/ (this can take up to an hour). This pipeline publishes the Java binding only.

## After the release

- GitLab mirrors the branches and tags to GitHub (https://github.com/RMLio/algemaploom-rs); check that the tag is there.
- Update the consumers: MappingWeaver-java: set the default of `mappingloom.version` to the new release.

## When something goes wrong

- The script stops at the first failing command. When it stops before pushing, fix the cause, discard its changes (`git reset --hard origin/development`) and run it again. When it stops between pushing `development` and pushing the tag, finish by hand: create and push the tag on the version commit, then set the next `-SNAPSHOT` in `crates/translator/src/java/algemaploom` (`mvn versions:set -DnewVersion=<next>-SNAPSHOT -DgenerateBackupPoms=false`), commit and push.
- Once the tag is pushed, keep it: fix the cause and retry the failed pipeline job. When the released code itself is broken, release the next patch version.
