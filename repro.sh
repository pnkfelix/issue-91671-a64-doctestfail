#!/bin/sh

NIGHTLY=nightly-2021-08-21

rustup update $NIGHTLY

rustc +$NIGHTLY --crate-name rayon_core rayon-core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C metadata=42766ff5a1612191 -C extra-filename=-42766ff5a1612191 --out-dir /Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps -L dependency=/Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps --cap-lints=allow

rustc +$NIGHTLY --crate-name a64_doctestfail --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto -C metadata=659d1e8ecf96f9ff -C extra-filename=-659d1e8ecf96f9ff --out-dir /Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps -L dependency=/Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps --extern rayon_core=/Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps/librayon_core-42766ff5a1612191.rmeta --cap-lints=allow

rustc +$NIGHTLY --crate-type bin --edition 2018 -o /tmp/rust_out -L dependency=/Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps -L dependency=/Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps --extern a64_doctestfail=/Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps/liba64_doctestfail-659d1e8ecf96f9ff.rlib --extern rayon_core=/Users/pnkfelix/Dev/Rust/a64_doctestfail/target/release/deps/librayon_core-42766ff5a1612191.rlib -Ccodegen-units=1 -C lto -Z unstable-options --target aarch64-apple-darwin --color never tests/demo.rs
