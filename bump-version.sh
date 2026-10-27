#!/usr/bin/env bash

# exit if command fails
set -e

if [ -z "$1" ]
then
	echo 'Version parameter not given. Invoke as e.g. ./bump-version.sh 1.0.0'.
	exit 1
fi

# function to read `y` (yes) or `n` (no).
function yes_or_no {
	while true; do
		read -p "$* [y/n]: " yn
		case $yn in
			[Yy]*) return 1  ;;
			[Nn]*) echo "Aborted" ; return 0 ;;
		esac
	done
}

SCRIPTDIR=$(dirname $(readlink -f $0))

VERSION=$1

# A release version is X.Y.Z; the next development version is derived from it.
if [[ ! $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && $VERSION != testrelease-* ]]; then
	echo "Version must be X.Y.Z or testrelease-*, got: $VERSION"
	exit 1
fi

# Sets the version of the crates (Cargo.toml, Cargo.lock), the Java binding
# (pom.xml) and the Node.js package (package.json, package-lock.json).
function set_version {
	echo "Changing version to $1"
	sed -i -e "s|^version = \".*\"|version = \"$1\"|" Cargo.toml
	cargo update --workspace
	cd crates/translator/src/java/algemaploom/
	mvn versions:set -DnewVersion=$1 -DgenerateBackupPoms=false
	cd $SCRIPTDIR
	npm version $1 --no-git-tag-version --allow-same-version
}

set_version $VERSION

if [ ! "$(yes_or_no 'Do you also want to add the version to CHANGELOG.md?')" ]
then
	changefrog -n $VERSION
fi

if [[ $VERSION == testrelease-* ]] ; then
	tagname=$VERSION
else
	tagname="v$VERSION"
fi

if [ ! "$(yes_or_no Do you also want to commit the changes, create a git tag $tagname and push it?)" ]
then
	git add Cargo.toml Cargo.lock package.json package-lock.json crates/translator/src/java/algemaploom/pom.xml CHANGELOG.md
	git commit -m "Update version to $VERSION"
	git push origin
	git tag $tagname
	git push origin $tagname

	# Prepare the next development cycle, so a local build is
	# distinguishable from the release.
	if [[ $VERSION != testrelease-* ]] ; then
		set_version "${VERSION%.*}.$((${VERSION##*.} + 1))-SNAPSHOT"
		git add Cargo.toml Cargo.lock package.json package-lock.json crates/translator/src/java/algemaploom/pom.xml
		git commit -m "Prepare for next development cycle"
		git push origin
	fi
fi

