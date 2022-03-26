# num2words

<a href="https://crates.io/crates/num2words"><img src="https://img.shields.io/crates/v/num2words"/></a> <a href="https://crates.io/crates/num2words"><img src="https://img.shields.io/crates/d/num2words"/></a> <a href="https://docs.rs/num2words"><img src="https://img.shields.io/docsrs/num2words"/></a> <a href="#license"><img src="https://img.shields.io/crates/l/num2words"/></a>

Convert number like `42` to `forty-two`

Example usage:
```rust
use num2words::num2words;
assert_eq!(num2words!(42), Ok(String::from("forty-two")));
```

The app can also be run via a command-line interface.

Example:
```sh
$ num2words 42
forty-two
$ num2words 10 --to EUR
ten euros
```

You can download the app via the following command:
```sh
$ cargo install num2words
```

For more information about the usage of `num2words` please refers to the
docs or via the following command:
```sh
$ num2words --help
```

This library is widely inspired by [Savoir-faire Linux's Python
lib](https://github.com/savoirfairelinux/num2words/).

**Warning**: this lib is not usable at its current state, we would recommend
you come back later.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
