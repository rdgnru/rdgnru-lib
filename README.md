# rdgnru-lib

`rdgnru-lib` is a small, dependency-free Rust utility library.

The package name on crates.io is `rdgnru-lib`; in Rust code, import it as `rdgnru_lib`.

## Install

Add this to your `Cargo.toml` after the crate is published:

```toml
[dependencies]
rdgnru-lib = "0.1"
```

## Usage

```rust
let greeting = rdgnru_lib::greet("Rust");
assert_eq!(greeting, "Hello, Rust!");

let slug = rdgnru_lib::slugify("Hello, Rust World!");
assert_eq!(slug, "hello-rust-world");

assert!(rdgnru_lib::is_valid_slug("hello-rust-world"));
```

## API

- `greet(name)` returns a friendly greeting.
- `slugify(input)` converts text into a lowercase ASCII slug.
- `is_valid_slug(input)` validates lowercase ASCII slugs.

## Development

```bash
cargo test
cargo publish --dry-run
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.
