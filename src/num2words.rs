use crate::{lang, Currency, Language, Number};
use std::str::FromStr;

/// Error type returned by [`num2words!`]
///
/// In case of
#[derive(Debug, PartialEq)]
pub enum Num2Err {
    CannotConvert,
    InvalidLang,
    InvalidToTag,
    NegativeOrdinal,
    FloatingOrdinal,
}

/// Macro to convert numbers to words
///
/// # Usage
///
/// The macro can be called with two optional parameters: `lang` and `to`.
///
/// Example:
/// ```
/// use num2words::num2words;
/// assert_eq!(
///     num2words!(42), Ok(String::from("forty-two"))
/// );
/// assert_eq!(
///     num2words!(42, lang = "en", to = "ordinal"),
///     Ok(String::from("forty-second"))
/// );
/// ```
///
/// ### `lang`
///
/// Can be:
/// * `en`
///
/// This list can be expanded! Do not hesitate to
/// [contribute](https://github.com/Ballasi/num2words)!
///
/// ### `to`
///
/// Can be:
/// * `cardinal`
/// * `ordinal`
/// * `ordinal_num`
/// * `year`
/// * any [currency](#currency)
///
/// ### `currency`
///
/// Can be:
/// * `AUD`
/// * `CAD`
/// * `DOLLAR` (non-localized dollar)
/// * `EUR`
/// * `GBP`
/// * `USD`
#[macro_export]
macro_rules! num2words {
    (
        $num: expr
        $(, lang = $lang: expr)?
        $(, to = $to: expr)?
    ) => {{
        let num = $num;
        let lang = $($lang;)? {"en"};
        let to = $($to;)? {"cardinal"};

        $crate::num2words::num2words(num, lang, to)
    }}
}

/// Function called by macro [`num2words!`]
///
/// The usage of this function is not recommended, but it can be done.
///
/// [`num2words!`] is calling this function, and it provides an easier
/// usage (defaults `lang` and `to` field).
pub fn num2words<T>(num: T, lang: &str, to: &str) -> Result<String, Num2Err>
where
    T: Into<Number>,
{
    let num = num.into();
    match lang::to_language(lang) {
        Some(lang) => {
            if let Ok(currency) = Currency::from_str(to) {
                lang.to_currency(num, currency)
            } else {
                match to {
                    "cardinal" => lang.to_cardinal(num),
                    "ordinal" => {
                        if let Number::Int(n) = num {
                            if n < 0 {
                                Err(Num2Err::NegativeOrdinal)
                            } else {
                                lang.to_ordinal(num)
                            }
                        } else {
                            Err(Num2Err::FloatingOrdinal)
                        }
                    }
                    "ordinal_num" => {
                        if let Number::Int(n) = num {
                            if n < 0 {
                                Err(Num2Err::NegativeOrdinal)
                            } else {
                                lang.to_ordinal_num(num)
                            }
                        } else {
                            Err(Num2Err::FloatingOrdinal)
                        }
                    }
                    "year" => lang.to_year(num),
                    _ => Err(Num2Err::InvalidToTag),
                }
            }
        }
        None => Err(Num2Err::InvalidLang),
    }
}
