use crate::lang;
use crate::num2words::Num2Err;
use crate::{Currency, Number};

/// Defines what is a language
pub trait Language {
    fn to_cardinal(self, num: Number) -> Result<String, Num2Err>;
    fn to_ordinal(self, num: Number) -> Result<String, Num2Err>;
    fn to_ordinal_num(self, num: Number) -> Result<String, Num2Err>;
    fn to_year(self, num: Number) -> Result<String, Num2Err>;
    fn to_currency(self, num: Number, currency: Currency) -> Result<String, Num2Err>;
}

pub fn to_language(lang: &str) -> Option<impl Language> {
    match lang {
        "en" => Some(lang::English {}),
        _ => None,
    }
}
