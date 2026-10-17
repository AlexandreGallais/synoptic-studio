#!/bin/sh
# Runs a Rust tool of tools/ from its release binary (EN-068): `scripts/rust-tool.sh <binary>
# [arguments…]`, its exit code kept. A missing binary fails naming the command that builds it.
# A shell launcher, not a Rust tool: it must run before any binary of the workspace is built.
binary="$(dirname "$0")/../target/release/$1"

if [ ! -x "$binary" ]; then
  echo "target/release/$1 is missing: build the Rust tools with \`npm run rust:build\`" >&2
  exit 127
fi

shift
exec "$binary" "$@"
