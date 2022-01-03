#!/bin/sh

REPRO_SUFFIX=2

NIGHTLY=nightly-2021-08-22

# END_TARGET=aarch64-apple-darwin
END_TARGET=aarch64-unknown-linux-gnu

rustup update $NIGHTLY
rustup target add --toolchain $NIGHTLY $END_TARGET

RUSTC=$(rustup which --toolchain $NIGHTLY rustc)
RUSTC=/media/pnkfelix/Rust/issue_91671/rust-91671/objdir-dbg/build/x86_64-unknown-linux-gnu/stage1/bin/rustc

# DEPS_DIR=/Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps
DEPS_DIR=/tmp/deps

mkdir -p $DEPS_DIR

RAYON_EXTRA_FILENAME=-42766ff5a1612191
A64_D_EXTRA_FILENAME=-659d1e8ecf96f9ff

set -e

echo "BUILD RAYON_CORE"
$RUSTC --crate-name rayon_core rayon-core/src/lib.rs --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C metadata=42766ff5a1612191 -C extra-filename=$RAYON_EXTRA_FILENAME --out-dir $DEPS_DIR -L dependency=$DEPS_DIR --cap-lints=allow --target $END_TARGET

echo "BUILD A64_DOCTESTFAIL"
$RUSTC --crate-name a64_doctestfail --edition=2018 src/lib.rs --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C metadata=659d1e8ecf96f9ff -C extra-filename=$A64_D_EXTRA_FILENAME --out-dir $DEPS_DIR -L dependency=$DEPS_DIR --extern rayon_core=$DEPS_DIR/librayon_core$RAYON_EXTRA_FILENAME.rmeta --cap-lints=allow --target $END_TARGET

echo "BUILD DEMO"

# Some potential prefixes to remember
# rr record -w

# Some potential suffixes to remember
# -C llvm-args=-debug
# -Z print-llvm-passes=yes
# -C llvm-args=-debug-only=instruction-select,aarch64-isel

$RUSTC --crate-type bin --edition 2018 -o /tmp/rust_out$REPRO_SUFFIX -L dependency=$DEPS_DIR -L dependency=$DEPS_DIR --extern a64_doctestfail=$DEPS_DIR/liba64_doctestfail$A64_D_EXTRA_FILENAME.rlib --extern rayon_core=$DEPS_DIR/librayon_core$RAYON_EXTRA_FILENAME.rlib -Ccodegen-units=1 -C lto -Z unstable-options --target $END_TARGET --color never tests/demo.rs
