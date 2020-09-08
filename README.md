# anyident
Any identifier for Rust

## Install

```toml
# Cargo.toml

[dependencies]
anyident = { git = "https://github.com/nwtgck/anyident-rust.git" }
```

## Usage

```rust
use anyident::i;

let i!(this is an identifier) = "hello, world";
assert_eq!(i!(this is an identifier), "hello, world");

// Japanese characters
let i!(こんにちは) = "hello!";
assert_eq!(i!(こんにちは).len(), 6);
```

## Current encoding in internal implementation

Currently, encoding is hex format. I have a plan to use more human readable one. `i!(こんにちは)` is encoded into `hex_e38193e38293e381abe381a1e381af`.
