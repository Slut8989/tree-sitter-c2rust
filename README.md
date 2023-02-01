# tree-sitter-c2rust
[![Crates.io](https://img.shields.io/crates/v/tree-sitter-c2rust)](https://crates.io/crates/tree-sitter-c2rust)

This is a fork of [Tree Sitter](https://github.com/tree-sitter/tree-sitter), but with the runtime component automatically converted to Rust using [c2rust](https://c2rust.com). This enables use of Tree Sitter parsers without libc, useful for targets like `wasm32-unknown-unknown`.

Because the conversion is automatic, the resulting code is not idiomatic Rust. But we preserve 1:1 correspondence with the original C code, so that future changes to Tree Sitter can be easily merged into this fork. Furthermore, the Rust bindings are still supported, so this can be used as a drop-in replacement for the regular Tree Sitter crate.
