use crate::lang;
use crate::{Currency, Number};

/// Defines what is a language
pub trait Language {
    fn to_cardinal(self, num: Number) -> Option<String>;
    fn to_ordinal(self, num: Number) -> Option<String>;
    fn to_ordinal_num(self, num: Number) -> Option<String>;
    fn to_year(self, num: Number) -> Option<String>;
    fn to_currency(self, num: Number, currency: Currency) -> Option<String>;
}

pub fn to_language(lang: &str) -> impl Language {
    match lang {
        "en" => lang::English {},
        _ => lang::English {},
    }
}
