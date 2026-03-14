#!/bin/bash

set -e

# release assistant

# on windows:
# CARGO_COMMAND="rustup run stable cargo" ./release.sh
if [ -z "$CARGO_COMMAND" ]; then
    CARGO_COMMAND="cargo"
fi

branch="$(git rev-parse --abbrev-ref HEAD)"

if [ "$branch" != "main" ]; then
    echo "new releases should be committed to main only!"
    exit 1
fi

set -x
git fetch
set +x

echo -n "latest release: "
read latest_release

echo "checking for occurrences of '${latest_release}'"

set +e
set -x
rg "${latest_release}"
set +x
set -e

echo -n "new release: "
read new_release

echo "replace the above occurrences of '${latest_release}' with '${new_release}'"
echo "and press enter to continue..."
read

echo "checking for occurrences of '${new_release}'"

set -x
rg "${new_release}"
set +x

echo "press enter to continue if this looks okay"
read

echo "building"

set -x
$CARGO_COMMAND build --release
set +x

echo "committing"

set -x
git add .
git status
set +x

echo "review staged files and press enter to continue"

set -x
git commit -m "release ${new_release}"
set +x

echo "publishing"

set -x
$CARGO_COMMAND publish
git push origin main
set +x

echo "tagging"

set -x
git tag "${new_release}"
git push origin "${new_release}"
set +x

echo "to author this release, open the page:"
echo "https://github.com/miller-time/rust-warrior/releases"
