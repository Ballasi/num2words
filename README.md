# num2words

<a href="https://crates.io/crates/num2words"><img src="https://img.shields.io/crates/v/num2words"/></a> <a href="https://crates.io/crates/num2words"><img src="https://img.shields.io/crates/d/num2words"/></a> <a href="https://docs.rs/num2words"><img src="https://img.shields.io/docsrs/num2words"/></a> <a href="#license"><img src="https://img.shields.io/crates/l/num2words"/></a>

Convert numbers like `42` to `forty-two`

## Usage

This crate can be either used as a library or a binary.

### Library

Example usage:

```rust
use num2words::Num2Words;
assert_eq!(Num2Words::new(42).to_words(), Ok(String::from("forty-two")));
```

The builder `Num2Words` can take any of these methods: `lang`, `cardinal`,
`ordinal`, `ordinal_num`, `year`, and `currency`.

```rust
use num2words::*;
assert_eq!(
    Num2Words::new(42).lang(Lang::French).to_words(),
    Ok(String::from("quarante-deux"))
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
$ num2words 10 --to UAH --lang uk
десять гривень
```

You can download the app via the following command:

```sh
$ cargo install num2words
```

You can also change the language via the CLI argument `--lang [locale]` and
provide a specific output type or a currency with the argument
`--to [cardinal|ordinal|ordinal_num|year|ISO 4217]`.

For more information about the usage of `num2words` please refer to the docs
or via the following command:

```sh
$ num2words --help
```

## Information

### Supported languages

Here is a list of all of the supported languages:

| Flag | Code              | Locale    | Language    | 42            |
| ---- | ----------------- | --------- | ----------- | ------------- |
| 🇺🇸🇬🇧 | `Lang::English`   | `en`      | English     | forty-two     |
| 🇫🇷🇨🇦 | `Lang::French`    | `fr`      | French      | quarante-deux |
| 🇧🇪🇨🇩 | `Lang::French_BE` | `fr_BE`   | French (BE) | quarante-deux |
| 🇨🇭   | `Lang::French_CH` | `fr_CH`   | French (CH) | quarante-deux |
| 🇪🇸   | `Lang::Spanish`   | `es`      | Spanish     | cuarenta y dos|
| 🇺🇦   | `Lang::Ukrainian` | `uk`      | Ukrainian   | сорок два     |

This list can be expanded! Contributions are welcomed.

### Supported output types

Here is a list of all of the supported outputs types (with the associated
command-line interface code):

| Library method   | CLI argument  | Example output                         |
| ---------------- | ------------- | -------------------------------------- |
| `.cardinal()`    | `cardinal`    | forty-two (42)                         |
| `.ordinal()`     | `ordinal`     | forty-second (42)                      |
| `.ordinal_num()` | `ordinal_num` | 42nd (42)                              |
| `.year()`        | `year`        | nineteen oh-one (1901)                 |
| `.currency(cur)` | ISO 4217 code | forty-two dollars and one cent (42.01) |

### Supported currencies

Three-letter enum variants corresponds to the currency's ISO 4217 code, but
there are exceptions to accomodate generic terminologies: `DINAR`, `DOLLAR`,
`PESO` and `RIYAL`.

A summary of all of the supported currencies are available in the [documentation
of `Currency`](https://docs.rs/num2words/latest/num2words/enum.Currency.html).

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
