#!/bin/sh

NIGHTLY=nightly-2021-08-22

END_TARGET=aarch64-apple-darwin

rustup update $NIGHTLY
rustup target add --toolchain $NIGHTLY $END_TARGET

# DEPS_DIR=/Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps
DEPS_DIR=/tmp/deps

mkdir -p $DEPS_DIR

rustc +$NIGHTLY --crate-name rayon_core rayon-core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C metadata=42766ff5a1612191 -C extra-filename=-42766ff5a1612191 --out-dir $DEPS_DIR -L dependency=$DEPS_DIR --cap-lints=allow --target $END_TARGET

rustc +$NIGHTLY --crate-name a64_doctestfail --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C metadata=659d1e8ecf96f9ff -C extra-filename=-659d1e8ecf96f9ff --out-dir $DEPS_DIR -L dependency=$DEPS_DIR --extern rayon_core=$DEPS_DIR/librayon_core-42766ff5a1612191.rmeta --cap-lints=allow --target $END_TARGET

rustc +$NIGHTLY --crate-type bin --edition 2018 -o /tmp/rust_out -L dependency=$DEPS_DIR -L dependency=$DEPS_DIR --extern a64_doctestfail=$DEPS_DIR/liba64_doctestfail-659d1e8ecf96f9ff.rlib --extern rayon_core=$DEPS_DIR/librayon_core-42766ff5a1612191.rlib -Ccodegen-units=1 -C lto -Z unstable-options --target $END_TARGET --color never tests/demo.rs
