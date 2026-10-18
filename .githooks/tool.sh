# Sourced by each hook (EN-108): `tool <crate> <binary> [arguments…]` runs a binary of the Rust
# workspace by its path, built by `npm install`'s `prepare`, or through Cargo when it is missing.
tool() {
  crate=$1
  binary=$2
  shift 2
  if [ -x "target/release/$binary" ]; then
    "target/release/$binary" "$@"
  else
    cargo run --release -q -p "$crate" --bin "$binary" -- "$@"
  fi
}
