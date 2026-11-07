#!/usr/bin/env bash
# Usage: ./update_js_bundle.sh. Takes macroquad's JS bundle of the version in Cargo.lock from the
# cargo registry and writes pkg/mq_js_bundle.js. Run a build first if the crate is not there yet.
set -euo pipefail
cd "$(dirname "$0")"

version=$(awk '/name = "macroquad"/ {getline; gsub(/"/, "", $3); print $3}' ../Cargo.lock)
bundle=("${CARGO_HOME:-$HOME/.cargo}"/registry/src/*/macroquad-"$version"/js/mq_js_bundle.js)
[[ -f "${bundle[0]}" ]] || { echo "No bundle for macroquad $version in the cargo registry" >&2; exit 1; }
tmp=$(mktemp --suffix=.js)
trap 'rm -f "$tmp"' EXIT
cp "${bundle[0]}" "$tmp"

# The net plugin is the last part. The game does not use it, and it throws on load.
perl -0pi -e 's/,function\(\)\{function \w+\(\)\{\}register_plugin=.*\}\(\)\s*$//s' "$tmp"
# This line runs on the first input, when the browser allows sound.
perl -pi -e 's/console\.log\("fix"\)/console.log("Sound is on after the first input")/' "$tmp"

# The bundle is minified: a new version can break the patterns above.
fail() { echo "Patch failed: $1" >&2; exit 1; }
grep -q ws_connect "$tmp" && fail "the net plugin is still there"
grep -q 'console.log("fix")' "$tmp" && fail "the fix log is still there"
grep -q 'name:"sapp_jsutils"' "$tmp" || fail "sapp_jsutils is lost"
grep -q audio_init "$tmp" || fail "the audio plugin is lost"
node --check "$tmp" || fail "the result is not valid JS"
cp "$tmp" pkg/mq_js_bundle.js
