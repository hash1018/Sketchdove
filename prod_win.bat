pushd frontend
setx CARGO_TARGET_DIR ../target-trunk 
trunk build --release
popd

cargo run --bin backend --release -- --port 8080 --static-dir ./dist
