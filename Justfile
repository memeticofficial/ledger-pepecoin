export MIRIFLAGS := "-Zmiri-symbolic-alignment-check -Zmiri-permissive-provenance -Zmiri-backtrace=full"
export RUST_BACKTRACE := "full"

# Show this menu
@help:
    just --list --unsorted

# Initialize project: fetch dependencies (SDK etc...)
init:
    just make deps
    just make zemu_install

# Run tests with miri
miri *args='':
    cargo +nightly miri test --features "full" {{args}}

# Run rust tests first and zemu_test afterwards
tests: build-elfs
    cargo test
    just miri
    make zemu_test

# Run zemu tests specified by the given filter
ztest filter="":
    cd zemu && yarn test -t {{filter}}

# Start debugging with zemu according to debug.mjs
debug:
    make zemu_debug

alias l := lint

# Format, then run clippy and fix warnings
lint:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged --all

# Build all the elfs of the app
build-elfs:
    make

alias m := make
make *cmd='':
    make {{cmd}}

try:
    cd zemu && yarn try

alias c := cargo
cargo *cmd='':
    cargo {{cmd}}

insta: (cargo "insta test --review")
