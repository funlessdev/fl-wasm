# fl-wasm-rs - write FunLess functions in Rust

This repository contains a Rust library that allows you to write FunLess functions in Rust. It's composed of:

- macros: the derive macro that allows you to write WebAssembly functions 
compatible with the FunLess platform. They are not imported directly, but used
through the `fl_wasm-rs` module.
- fl-wasm-rs: the Rust library for FunLess functions. Besides exporting the macro,
it also makes integrate the compatibility layer with the FunLess platform. 
For debugging purposes, it can also provide a `console_log` function that
can be called within the wasm function to log messages to the worker console.