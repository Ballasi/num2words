# num2words

Convert number like `42` to `forty-two`

Example usage:
```rust
use num2words::num2words;
assert_eq!(num2words!(42), Ok(String::from("forty-two")));
```

This lib will also be a downloadable binary in the near future.

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
