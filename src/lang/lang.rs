use crate::lang;
use crate::num2words::Num2Err;
use crate::Currency;
use num_bigfloat::BigFloat;
use std::str::FromStr;

/// Defines what is a language
pub trait Language {
    fn to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err>;
    fn to_ordinal(&self, num: BigFloat) -> Result<String, Num2Err>;
    fn to_ordinal_num(&self, num: BigFloat) -> Result<String, Num2Err>;
    fn to_year(&self, num: BigFloat) -> Result<String, Num2Err>;
    fn to_currency(&self, num: BigFloat, currency: Currency) -> Result<String, Num2Err>;
}

/// Languages available in `num2words`
#[allow(non_camel_case_types)]
pub enum Lang {
    /// ```
    /// use num2words::{Num2Words, Lang};
    /// assert_eq!(
    ///     Num2Words::new(42).lang(Lang::English).to_words(),
    ///     Ok(String::from("forty-two"))
    /// );
    /// ```
    English,
    /// French from France and Canada
    /// ```
    /// use num2words::{Num2Words, Lang};
    /// assert_eq!(
    ///     Num2Words::new(42).lang(Lang::French).to_words(),
    ///     Ok(String::from("quarante-deux"))
    /// );
    /// ```
    French,
    /// French from Belgium and the Democratic Republic of the Congo
    /// ```
    /// use num2words::{Num2Words, Lang};
    /// assert_eq!(
    ///     Num2Words::new(70).lang(Lang::French_BE).to_words(),
    ///     Ok(String::from("septante"))
    /// );
    /// ```
    French_BE,
    /// French from Swiss Confederation and Aosta Valley (Italy)
    /// ```
    /// use num2words::{Num2Words, Lang};
    /// assert_eq!(
    ///     Num2Words::new(80).lang(Lang::French_CH).to_words(),
    ///     Ok(String::from("huitante"))
    /// );
    /// ```
    French_CH,
    /// ```
    /// use num2words::{Num2Words, Lang};
    /// assert_eq!(
    ///     Num2Words::new(42).lang(Lang::Spanish).to_words(),
    ///     Ok(String::from("cuarenta y dos"))
    /// );
    /// ```
    Spanish,
    /// ```
    /// use num2words::{Num2Words, Lang};
    /// assert_eq!(
    ///     Num2Words::new(42).lang(Lang::Ukrainian).to_words(),
    ///     Ok(String::from("сорок два"))
    /// );
    /// ```
    Ukrainian,
}

impl FromStr for Lang {
    type Err = ();

    /// Parses a string to return a value of this type
    ///
    /// | Locale    | Lang              | 42            |
    /// | --------- | ----------------- | ------------- |
    /// | `en`      | `Lang::English`   | forty-two     |
    /// | `es`      | `Lang::Spanish`   | cuarenta y dos|
    /// | `fr`      | `Lang::French`    | quarante-deux |
    /// | `fr_BE`   | `Lang::French_BE` | quarante-deux |
    /// | `fr_CH`   | `Lang::French_CH` | quarante-deux |
    /// | `uk`      | `Lang::Ukrainian` | сорок два     |
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "en" => Ok(Self::English),
            "es" => Ok(Self::Spanish),
            "fr" => Ok(Self::French),
            "fr_BE" => Ok(Self::French_BE),
            "fr_CH" => Ok(Self::French_CH),
            "uk" => Ok(Self::Ukrainian),
            _ => Err(()),
        }
    }
}

pub fn to_language(lang: Lang, preferences: Vec<String>) -> Box<dyn Language> {
    match lang {
        Lang::English => {
            let last = preferences
                .iter()
                .rev()
                .find(|v| ["oh", "nil"].contains(&v.as_str()));

            if let Some(v) = last {
                return Box::new(lang::English::new(v == "oh", v == "nil"));
            }

            Box::new(lang::English::new(false, false))
        }
        Lang::French => {
            let feminine = preferences
                .iter()
                .find(|v| ["feminine", "feminin", "féminin", "f"].contains(&v.as_str()))
                .is_some();
            let reformed = preferences
                .iter()
                .find(|v: &&String| ["reformed", "1990", "rectifié", "rectification"].contains(&v.as_str()))
                .is_some();

            Box::new(lang::French::new(feminine, reformed, lang::fr::RegionFrench::FR))
        }
        Lang::French_BE => {
            let feminine = preferences
                .iter()
                .find(|v| ["feminine", "feminin", "féminin", "f"].contains(&v.as_str()))
                .is_some();
            let reformed = preferences
                .iter()
                .find(|v: &&String| ["reformed", "1990", "rectifié", "rectification"].contains(&v.as_str()))
                .is_some();

            Box::new(lang::French::new(feminine, reformed, lang::fr::RegionFrench::BE))
        }
        Lang::French_CH => {
            let feminine = preferences
                .iter()
                .find(|v| ["feminine", "feminin", "féminin", "f"].contains(&v.as_str()))
                .is_some();
            let reformed = preferences
                .iter()
                .find(|v: &&String| ["reformed", "1990", "rectifié", "rectification"].contains(&v.as_str()))
                .is_some();

            Box::new(lang::French::new(feminine, reformed, lang::fr::RegionFrench::CH))
            }
        Lang::Spanish => {
            use super::es::{DecimalChar, NegativeFlavour};
            let neg_flavour = preferences
                .iter()
                .find_map(|v| NegativeFlavour::from_str(v).ok())
                .unwrap_or_default();
            let prefer_veinte = preferences
                .iter()
                .any(|v| ["veinte"].binary_search(&v.as_str()).is_ok());
            let decimal_char = preferences
                .iter()
                .find_map(|v| DecimalChar::from_str(v).ok())
                .unwrap_or_default();
            let feminine = preferences.iter().any(|v| {
                ["f", "femenino", "feminine"]
                    .binary_search(&v.as_str())
                    .is_ok()
            });
            let plural = preferences
                .iter()
                .any(|v| ["plural"].binary_search(&v.as_str()).is_ok());
            let lang = lang::Spanish::new(decimal_char, feminine)
                .with_plural(plural)
                .with_veinte(prefer_veinte)
                .with_neg_flavour(neg_flavour);
            Box::new(lang)
        }
        Lang::Ukrainian => {
            let declension: lang::uk::Declension = preferences
                .iter()
                .rev()
                .find_map(|d| d.parse().ok())
                .unwrap_or_default();
            let gender: lang::uk::Gender = preferences
                .iter()
                .rev()
                .find_map(|d| d.parse().ok())
                .unwrap_or_default();
            let number: lang::uk::GrammaticalNumber = preferences
                .iter()
                .rev()
                .find_map(|d| d.parse().ok())
                .unwrap_or_default();
            Box::new(lang::Ukrainian::new(gender, number, declension))
        }
    }
}
