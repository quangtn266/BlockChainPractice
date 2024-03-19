#       $ cargo install grcov
#       $ rustup component add llvm-tools-preview

cargo clean
rm -rf ./coverage ./target *.prof*

export RUSTFLAGS="-C instrument-coverage"

export LLVM_PROFILE_FILE="rust_blockchain-%p-%m.profraw"

cargo build
cargo test
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing --ignore "tests/*" -o ./coverage/

rm *.prof*