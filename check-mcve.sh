RUSTFLAGS=--cap-lints=allow cargo +nightly-2021-10-13 test --release --doc  -v > /dev/null 2>&1

F0=$?; export F0

echo $F0


RUSTFLAGS=--cap-lints=allow cargo +nightly-2021-10-14 test --release --doc  -v > /dev/null 2>&1

F1=$?; export F1
echo $F1


if test "$F0" == "0" && test "$F1" == "101"; then
    git add . && git commit -m checkpoint-repro
else
    echo did not repro therefore not checkpointing.
fi

