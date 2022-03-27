#![crate_type = "lib"]
#![crate_name = "num2words"]

/*!
 * Convert number like `42` to `forty-two`
 *
 * Example usage:
 * ```
 * use num2words::Num2Words;
 * assert_eq!(Num2Words::new(42).to_words(), Ok(String::from("forty-two")));
 * ```
 *
 * This lib will also be a downloadable binary in the near future.
 *
 * For more detailed usage about the different parameters that you can give
 * to the macro, please take a look at [`num2words!`].
 *
 * This library is widely inspired by [Savoir-faire Linux's Python
 * lib](https://github.com/savoirfairelinux/num2words/).
 *
 * **Warning**: this lib is not usable at its current state, we would recommend
 * you come back later.
 */
mod num2words;
mod currency;
mod lang;
mod number;
mod output;

pub use currency::Currency;
pub use lang::Lang;
pub use crate::num2words::{Num2Words, Num2Err};
pub use output::Output;
use lang::Language;
use number::Number;
