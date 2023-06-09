cd ijvm_cpp_wrapper
cargo clean
cargo build
cbindgen --crate ijvm_cpp_wrapper --output ijvm_cpp_wrapper.h
cp target/debug/*.a ../ijvm_inspector/
mv *.h ../ijvm_inspector/
cd ../ijvm_inspector/
rm -rf build
mkdir build
cd build
qmake ..
make -j$(grep -c ^processor /proc/cpuinfo)
