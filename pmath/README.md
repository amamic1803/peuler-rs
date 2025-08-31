# pmath

[<img alt="GitHub Repository Static Badge" src="https://img.shields.io/badge/GitHub-pmath-blue?logo=github">](https://github.com/amamic1803/peuler-rs/tree/main/pmath)
[<img alt="Crates.io Version" src="https://img.shields.io/crates/v/pmath?logo=rust">](https://crates.io/crates/pmath)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/pmath?logo=docs.rs&label=docs.rs">](https://docs.rs/pmath)
[<img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/amamic1803/peuler-rs/test.yml">](https://github.com/amamic1803/peuler-rs/actions/workflows/test.yml)
[<img alt="GitHub License" src="https://img.shields.io/github/license/amamic1803/peuler-rs">](https://github.com/amamic1803/peuler-rs/blob/main/LICENSE)

_pmath_ is a general-purpose mathematics crate for Rust.

It provides a wide range of mathematical functions and algorithms
which heavily use Rust's idiomatic features such as traits, generics, and iterators.

This crate started as a mathematical backend for the [peuler](https://crates.io/crates/peuler) crate,
but has since evolved into a standalone crate.


## Example
```rust
use pmath::{gcd, ord};
use pmath::primes::is_prime;

assert_eq!(gcd(48, 18), 6);
assert_eq!(ord(3, 7), 6);
assert!(is_prime(29).0);
```


## No-std support
There is currently no support for `no-std` environments.


## WebAssembly support
`wasm32-unknown-unknown` target is supported, but requires `--cfg getrandom_backend="wasm_js"` flag to be passed to the compiler.
See [getrandom crate's documentation](https://docs.rs/getrandom/#webassembly-support) for more details.

Other WASM targets should work out of the box.


## License
This project is licensed under the [MIT License](https://github.com/amamic1803/peuler-rs/blob/main/LICENSE).


## Contributing
Contributions are welcome!

Please open an issue or a pull request in the [GitHub repository](https://github.com/amamic1803/peuler-rs).
