cd ijvm_cpp_wrapper
cargo clean
cargo build
cbindgen --crate ijvm_cpp_wrapper --output ijvm_cpp_wrapper.h
# todo: move in the right pos
cp target/debug/*.a ..
mv *.h ..
