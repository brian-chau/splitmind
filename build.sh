cargo build --release
rm -rf ./exe
mkdir ./exe
cp target/release/tree_rs ./exe/tree_rs
cargo clean