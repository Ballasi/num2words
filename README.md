# num2words

<a href="https://crates.io/crates/num2words"><img src="https://img.shields.io/crates/v/num2words"/></a> <a href="https://crates.io/crates/num2words"><img src="https://img.shields.io/crates/d/num2words"/></a> <a href="https://docs.rs/num2words"><img src="https://img.shields.io/docsrs/num2words"/></a> <a href="#license"><img src="https://img.shields.io/crates/l/num2words"/></a>

Convert number like `42` to `forty-two`

## Usage

This crate can be either used as a library or a binary.

### Library

Example usage:

```rust
use num2words::Num2Words;
assert_eq!(Num2Words::new(42).to_words(), Ok(String::from("forty-two")));
```

The builder Num2Words can take any of these arguments: `lang`, `cardinal`,
`ordinal`, `ordinal_num`, `year`, and `currency`.

```rust
use num2words::*;
assert_eq!(
    Num2Words::new(42).lang(Lang::English).to_words(),
    Ok(String::from("forty-two"))
);
assert_eq!(
    Num2Words::new(42).ordinal().to_words(),
    Ok(String::from("forty-second"))
);
assert_eq!(
    Num2Words::new(42.01).currency(Currency::DOLLAR).to_words(),
    Ok(String::from("forty-two dollars and one cent"))
);
```

These arguments can be chained.

For more information about the available languages, outputs types and
currencies, see [Information](#information).

### Binary

This crate provides a command-line interface to run requests on `num2words`.

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

You can also change the language via the CLI argument `--lang [ISO 639-1
code]` and provide a specific output type or a currency with the argument
`--to [cardinal|ordinal|ordinal_num|year|ISO 4217]`.

For more information about the usage of `num2words` please refer to the docs
or via the following command:
```sh
$ num2words --help
```

## Information

### Supported languages

Here is a list of all of the supported languages:

| Flag | Code            | ISO 639-1       | Language | 42        |
| ---- | --------------- | --------------- | -------- | --------- |
| ???????????????? | `Lang::English` | `en`            | English  | forty-two |

This list can be expanded! Contributions are welcomed.

### Supported output types

Here is a list of all of the supported outputs types (with the command-line
interface code):

- `.cardinal()` (`cardinal`): forty-two (42)
- `.ordinal()` (`ordinal`): forty-second (42)
- `.ordinal_num()` (`ordinal_num`): 42nd (42)
- `.year()` (`year`): nineteen oh-one (1901)
- any currency: forty-two dollars and one cent (42.01)

### Supported currencies

Here is a list of all of the supported currencies (with the ISO 4217 code):

- `Currency::AUD` (`AUD`): australian dollar
- `Currency::CAD` (`CAD`): canadian dollar
- `Currency::DOLLAR` (`DOLLAR`): dollar
- `Currency::EUR` (`EUR`): euro
- `Currency::GBP` (`GBP`): pound
- `Currency::USD` (`USD`): US dollar

### About

This library is widely inspired by [Savoir-faire Linux's Python
lib](https://github.com/savoirfairelinux/num2words/).

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
