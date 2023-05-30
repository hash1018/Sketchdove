pushd client
setx CARGO_TARGET_DIR ../target-trunk 
trunk build --release
popd

cargo run --bin server --release -- --port 8080 --static-dir ./dist
