cargo build || exit
RUST_LOG=debug target/debug/rust_bundler_cp --output /dev/shm/o.rs --input tests/testdata/input/multiple_binaries --binary a --remove_unused_mod
rustc /dev/shm/o.rs -o /dev/shm/o
/dev/shm/o
