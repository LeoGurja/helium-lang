export RUSTC_BOOTSTRAP=1
export RUSTFLAGS="-Zinstrument-coverage"
export LLVM_PROFILE_FILE="lgurgel-%p-%m.profraw"
cargo build && cargo test && grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/
