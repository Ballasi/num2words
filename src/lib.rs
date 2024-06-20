#![crate_type = "lib"]
#![crate_name = "num2words"]

/*!
 * Convert numbers like `42` to `forty-two`
 *
 * ## Usage
 *
 * This crate can be either used as a library or a binary.
 *
 * ### Library
 *
 * Example usage:
 *
 * ```rust
 * use num2words::Num2Words;
 * assert_eq!(Num2Words::new(42).to_words(), Ok(String::from("forty-two")));
 * ```
 *
 * The builder `Num2Words` can take any of these methods: `lang`, `cardinal`,
 * `ordinal`, `ordinal_num`, `year`, and `currency`.
 *
 * ```rust
 * use num2words::*;
 * assert_eq!(
 *     Num2Words::new(42).lang(Lang::French).to_words(),
 *     Ok(String::from("quarante-deux"))
 * );
 * assert_eq!(
 *     Num2Words::new(42).ordinal().to_words(),
 *     Ok(String::from("forty-second"))
 * );
 * assert_eq!(
 *     Num2Words::new(42.01).currency(Currency::DOLLAR).to_words(),
 *     Ok(String::from("forty-two dollars and one cent"))
 * );
 * ```
 *
 * These arguments can be chained.
 *
 * For more information about the available languages, outputs types and
 * currencies, see [Information](#information).
 *
 * ### Binary
 *
 * This crate provides a command-line interface to run requests on `num2words`.
 *
 * Example:
 * ```sh
 * $ num2words 42
 * forty-two
 * $ num2words 10 --to UAH --lang uk
 * десять гривень
 * ```
 *
 * You can download the app via the following command:
 * ```sh
 * $ cargo install num2words
 * ```
 *
 * You can also change the language via the CLI argument `--lang [locale]` and
 * provide a specific output type or a currency with the argument
 * `--to [cardinal|ordinal|ordinal_num|year|ISO 4217]`.
 *
 * For more information about the usage of `num2words` please refer to the docs
 * or via the following command:
 * ```sh
 * $ num2words --help
 * ```
 *
 * ## Information
 *
 * ### Supported languages
 *
 * Here is a list of all of the supported languages:
 *
 * | Flag | Code              | Locale    | Language    | 42            |
 * | ---- | ----------------- | --------- | ----------- | ------------- |
 * | 🇺🇸🇬🇧 | `Lang::English`   | `en`      | English     | forty-two     |
 * | 🇫🇷🇨🇦 | `Lang::French`    | `fr`      | French      | quarante-deux |
 * | 🇧🇪🇨🇩 | `Lang::French_BE` | `fr_BE`   | French (BE) | quarante-deux |
 * | 🇨🇭   | `Lang::French_CH` | `fr_CH`   | French (CH) | quarante-deux |
 * | 🇪🇸   | `Lang::Spanish`   | `es`      | Spanish     | cuarenta y dos|
 * | 🇺🇦   | `Lang::Ukrainian` | `uk`      | Ukrainian   | сорок два     |
 *
 * This list can be expanded! Contributions are welcomed.
 *
 * ### Supported output types
 *
 * Here is a list of all of the supported outputs types (with the associated
 * command-line interface code):
 *
 * | Library method   | CLI argument  | Example output                         |
 * | ---------------- | ------------- | -------------------------------------- |
 * | `.cardinal()`    | `cardinal`    | forty-two (42)                         |
 * | `.ordinal()`     | `ordinal`     | forty-second (42)                      |
 * | `.ordinal_num()` | `ordinal_num` | 42nd (42)                              |
 * | `.year()`        | `year`        | nineteen oh-one (1901)                 |
 * | `.currency(cur)` | ISO 4217 code | forty-two dollars and one cent (42.01) |
 *
 * ### Supported currencies
 *
 * Three-letter enum variants corresponds to the currency's ISO 4217 code, but
 * there are exceptions to accomodate generic terminologies: `DINAR`, `DOLLAR`,
 * `PESO` and `RIYAL`.
 *
 * A summary of all of the supported currencies are available in the
 * documentation of [`Currency`].
 *
 * ### About
 *
 * This library is widely inspired by [Savoir-faire Linux's Python
 * lib](https://github.com/savoirfairelinux/num2words/).
 */
mod num2words;

mod currency;
mod lang;
mod output;

pub use crate::num2words::{Num2Err, Num2Words};
pub use currency::Currency;
pub use lang::Lang;
use lang::Language;
use output::Output;
