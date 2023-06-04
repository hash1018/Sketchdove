pushd frontend
setx CARGO_TARGET_DIR ../target-trunk 
trunk build --release
popd

cargo run --bin backend --release -- --static-dir ./dist
