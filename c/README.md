# Notes

## compile with sdk

clang --target=wasm32-unknown-wasi --sysroot wasi-sysroot hello.c -o hello.wasm

./wasi-sdk-20.0/bin/clang -mexec-model=reactor -Wl,--export-all --sysroot wasi-sdk-20.0/share/wasi-sysroot/ hello.c -o hello.wasm

## todo

https://github.com/WebAssembly/wasi-sdk

go run cmd/fl/main.go fn invoke hello9 --json='{"input":"test"}'
