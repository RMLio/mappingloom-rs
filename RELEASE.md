# Releasing algemaploom-rs (MappingLoom)

Step-by-step instructions for publishing a release. [HANDBOOK.md](HANDBOOK.md) (Release process) explains how the tooling works.

## Before you start

- Everything changed since the previous release is reviewed: the code for correctness, and the documentation and changelog for accuracy and brevity.
- bash (Git Bash on Windows), Maven, Rust with `cargo` (rustup installs the toolchain pinned in `rust-toolchain.toml`; the script runs `cargo update --workspace` to update `Cargo.lock`), Java 21, and `changefrog` (`npm install -g changefrog`), which writes the version section of `CHANGELOG.md`.
- Push access to `origin` (https://gitlab.ilabt.imec.be/rml/proc/algemaploom-rs).
- You are on `development`, up to date with `origin/development`, with a clean working tree.
- The tests pass: `cargo test --workspace` and `./test_java.sh` (the Java binding). `./test_java.sh` builds the native libraries with `build_native.sh`, so run it on Linux or WSL, with the toolchains of the README (Bindings).
- `## Unreleased` in `CHANGELOG.md` lists everything since the last release.
- Pick the version with [Semantic Versioning](https://semver.org/). The versions in the tree hold the next patch as `-SNAPSHOT` (`<version>-SNAPSHOT`): release that patch, or a higher minor or major version when the changelog has new features or breaking changes.

## Release

1. Run `./bump-version.sh <version>` (format `X.Y.Z`) and answer `y` to both questions. The script
   - sets the version in `Cargo.toml`, `Cargo.lock`, the Java `pom.xml`, `package.json` and `package-lock.json`;
   - turns `## Unreleased` into the version section of `CHANGELOG.md`;
   - commits "Update version to <version>", pushes `development`, and creates and pushes the tag `v<version>`;
   - moves those files to the next patch `-SNAPSHOT`, and commits and pushes "Prepare for next development cycle".
2. Move `main` to the release: `git push origin v<version>^{commit}:main`. `main` always points at the latest release; the push succeeds only as a fast-forward.
3. The tag pipeline (https://gitlab.ilabt.imec.be/rml/proc/algemaploom-rs/-/pipelines) runs the `Maven Central Deployment` job, which builds the Java binding with the native libraries for Linux, Windows and macOS, signs it and deploys it to Maven Central. That job may fail without failing the pipeline, so check the job itself; the new version then appears at https://repo1.maven.org/maven2/be/ugent/idlab/knows/MappingLoom/ (this can take up to an hour). This pipeline publishes the Java binding only.

## After the release

- GitLab mirrors the branches and tags to GitHub (https://github.com/RMLio/algemaploom-rs); check that the tag is there.
- Update the consumers: MappingWeaver-java: set the default of `mappingloom.version` to the new release.

## When something goes wrong

- The script stops at the first failing command. When it stops before pushing, fix the cause, discard its changes (`git reset --hard origin/development`) and run it again. When it stops after pushing `development`, finish by hand: create the tag `v<version>` on the version commit if it is missing, push it (`git push origin v<version>`), then set the next `-SNAPSHOT` as `set_version` in `bump-version.sh` does, commit and push.
- Once the tag is pushed, keep it: fix the cause and retry the failed pipeline job. When the released code itself is broken, release the next patch version.
- When no GitLab runner is available, run the `Maven Central Deployment` job by hand (see below).

## Deploying to Maven Central by hand

These steps do what the `Maven Central Deployment` job does, in bash on Debian or Ubuntu (natively, in a VM or in WSL). The secrets are the job's CI variables (GitLab → Settings → CI/CD → Variables).

### 1. Install the tools (once)

`java -version` must be 17 or newer.

```bash
sudo apt-get update
sudo apt-get install -y gpg ca-certificates default-jdk maven gcc-mingw-w64 python3-pip build-essential git curl
pip3 install --user --break-system-packages cargo-zigbuild ziglang
export PATH="$HOME/.local/bin:$PATH"
# Rust, when `cargo --version` fails:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
```

### 2. Clone the tag and add the targets

Clone with Unix line endings, which `build_native.sh` needs, and add the targets to the pinned toolchain from the repository root:

```bash
git clone -c core.autocrlf=false --depth 1 --branch v<version> https://gitlab.ilabt.imec.be/rml/proc/algemaploom-rs.git
cd algemaploom-rs
rustup target add x86_64-unknown-linux-gnu x86_64-pc-windows-gnu x86_64-apple-darwin aarch64-apple-darwin
```

### 3. Set the signing key and the secrets

Import the signing key (`GPG_SECRET_KEY`, saved as a file) into a temporary keyring, and set the Central Portal token (`MAVEN_REPO_USER`, `MAVEN_REPO_PASS`) and the key's passphrase, if it has one:

```bash
export GNUPGHOME=$(mktemp -d)
gpg --batch --import path/to/secret.asc
gpg --list-secret-keys   # lists 6861488FA42A028D5378AF8833C4D74DDB5F1B28
read -p  "Central user: " MAVEN_REPO_USER; export MAVEN_REPO_USER
read -sp "Central password: " MAVEN_REPO_PASS; export MAVEN_REPO_PASS; echo
read -sp "GPG passphrase (empty if none): " MAVEN_GPG_PASSPHRASE; export MAVEN_GPG_PASSPHRASE; echo
```

### 4. Build, sign and deploy

```bash
cd crates/translator/src/java/algemaploom
mvn -s .m2/settings.xml --batch-mode --errors --fail-at-end --show-version -DinstallAtEnd=true -DdeployAtEnd=true clean deploy -P release -DskipTests=true
```

When a native library fails to build, its log is `target/<target>/log.txt` in the repository root.

### 5. Remove the key and the secrets

Delete the saved key file as well.

```bash
rm -rf "$GNUPGHOME"; unset GNUPGHOME MAVEN_REPO_USER MAVEN_REPO_PASS MAVEN_GPG_PASSPHRASE
```

### 6. Check the deployment

The deployment shows at https://central.sonatype.com; the version then appears at https://repo1.maven.org/maven2/be/ugent/idlab/knows/MappingLoom/.
