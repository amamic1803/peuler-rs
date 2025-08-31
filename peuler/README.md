# peuler

[<img alt="GitHub Repository Static Badge" src="https://img.shields.io/badge/GitHub-peuler-blue?logo=github">](https://github.com/amamic1803/peuler-rs/tree/main/peuler)
[<img alt="Crates.io Version" src="https://img.shields.io/crates/v/peuler?logo=rust">](https://crates.io/crates/peuler)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/peuler?logo=docs.rs&label=docs.rs">](https://docs.rs/peuler)
[<img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/amamic1803/peuler-rs/test.yml">](https://github.com/amamic1803/peuler-rs/actions/workflows/test.yml)
[<img alt="GitHub License" src="https://img.shields.io/github/license/amamic1803/peuler-rs">](https://github.com/amamic1803/peuler-rs/blob/main/LICENSE)

_peuler_ is a Rust crate with solutions to the [*Project Euler*](https://projecteuler.net) problems.

It contains solutions to the problems that I have solved so far,
and I plan to add more solutions as I solve more problems.

Many general-purpose mathematical functions are offloaded to the [pmath](https://crates.io/crates/pmath) crate.


## Example
```rust
use peuler::{PEuler, ProjectEuler};

let peuler = PEuler::new();
assert_eq!(peuler.solve(1).unwrap(), "233168");
```


## No-std support
There is currently no support for `no-std` environments.


## WebAssembly support
Since this crate depends on [pmath](https://crates.io/crates/pmath),
which in turn depends on [getrandom](https://crates.io/crates/getrandom),
`wasm32-unknown-unknown` target is supported, but requires `--cfg getrandom_backend="wasm_js"` flag to be passed to the compiler.
See [getrandom crate's documentation](https://docs.rs/getrandom/#webassembly-support) for more details.

Other WASM targets should work out of the box.


## Command Line Interface
This crate also provides a command line interface (CLI) binary named `peuler`.

You can install it via `cargo`:
```sh
cargo install --features=cli peuler
```

Then, to solve the [*first Project Euler problem*](https://projecteuler.net/problem=1), run:
```sh
peuler 1
```

For more information, run:
```sh
peuler --help
```


## Features
This crate has the following optional (disabled by default) features:
- `cli`: Enables the optional dependencies for building the CLI binary.


## License
This project is licensed under the [MIT License](https://github.com/amamic1803/peuler-rs/blob/main/LICENSE).


## Contributing
Contributions to the currently available problems are welcome!

Please open an issue or a pull request in the [GitHub repository](https://github.com/amamic1803/peuler-rs).

Don't submit solutions to the currently unsolved problems, as I plan to solve them on my own.
