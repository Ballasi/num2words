use crate::lang;
use crate::num2words::Num2Err;
use crate::{Currency, Number};
use std::str::FromStr;

/// Defines what is a language
pub trait Language {
    fn to_cardinal(self, num: Number) -> Result<String, Num2Err>;
    fn to_ordinal(self, num: Number) -> Result<String, Num2Err>;
    fn to_ordinal_num(self, num: Number) -> Result<String, Num2Err>;
    fn to_year(self, num: Number) -> Result<String, Num2Err>;
    fn to_currency(self, num: Number, currency: Currency) -> Result<String, Num2Err>;
}

pub enum Lang {
    English,
}

impl FromStr for Lang {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "en" => Ok(Self::English),
            _ => Err(()),
        }
    }
}

pub fn to_language(lang: Lang) -> impl Language {
    match lang {
        Lang::English => lang::English {},
    }
}
