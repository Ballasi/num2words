use crate::{lang, Currency, Lang, Output};
use num_bigfloat::BigFloat;

/// Error type returned by the builder
#[derive(Debug, PartialEq)]
pub enum Num2Err {
    /// General error, language cannot convert number
    ///
    /// It is likely that the language does not support the number because
    /// it is too large.
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Err, Num2Words};
    /// assert_eq!(
    ///     Num2Words::new(1e100).to_words(),
    ///     Err(Num2Err::CannotConvert)
    /// );
    /// ```
    CannotConvert,
    /// Request of a negative ordinal
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Err, Num2Words};
    /// assert_eq!(
    ///     Num2Words::new(-42).ordinal().to_words(),
    ///     Err(Num2Err::NegativeOrdinal)
    /// );
    /// ```
    NegativeOrdinal,
    /// Request of a float ordinal
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Err, Num2Words};
    /// assert_eq!(
    ///     Num2Words::new(42.01).ordinal().to_words(),
    ///     Err(Num2Err::FloatingOrdinal)
    /// );
    /// ```
    FloatingOrdinal,
    /// Request of a float year
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Err, Num2Words};
    /// assert_eq!(
    ///     Num2Words::new(42.01).year().to_words(),
    ///     Err(Num2Err::FloatingYear)
    /// );
    /// ```
    FloatingYear,
    /// Request of an infinite ordinal
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Err, Num2Words};
    /// assert_eq!(
    ///     Num2Words::parse("inf").unwrap().ordinal().to_words(),
    ///     Err(Num2Err::InfiniteOrdinal)
    /// );
    /// ```
    InfiniteOrdinal,
    /// Request of an infinite year
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Err, Num2Words};
    /// assert_eq!(
    ///     Num2Words::parse("inf").unwrap().year().to_words(),
    ///     Err(Num2Err::InfiniteYear)
    /// );
    /// ```
    InfiniteYear,
}

impl std::fmt::Display for Num2Err {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Num2Err::CannotConvert => "cannot convert number",
                Num2Err::NegativeOrdinal => "cannot treat negative number as ordinal",
                Num2Err::FloatingOrdinal => "cannot treat float as ordinal",
                Num2Err::FloatingYear => "cannot treat float as year",
                Num2Err::InfiniteOrdinal => "cannot treat infinity as ordinal",
                Num2Err::InfiniteYear => "cannot treat infinity as year",
            }
        )
    }
}

/// Builder for `num2words`
pub struct Num2Words {
    num: BigFloat,
    lang: Lang,
    output: Output,
    currency: Currency,
    preferences: Vec<String>,
}

impl Num2Words {
    /// Creates a new builder
    ///
    /// Example:
    /// ```
    /// use num2words::Num2Words;
    /// assert_eq!(
    ///     Num2Words::new(42).to_words(),
    ///     Ok(String::from("forty-two"))
    /// );
    /// assert_eq!(
    ///     Num2Words::new(1e3).to_words(),
    ///     Ok(String::from("one thousand"))
    /// );
    /// ```
    pub fn new<T>(num: T) -> Self
    where
        T: Into<BigFloat>,
    {
        Self {
            num: num.into(),
            lang: Lang::English,
            output: Output::Cardinal,
            currency: Currency::DOLLAR,
            preferences: vec![],
        }
    }

    /// Creates a new builder from a string
    ///
    /// Example:
    /// ```
    /// use num2words::Num2Words;
    /// assert_eq!(
    ///     Num2Words::parse("42").unwrap().to_words(),
    ///     Ok(String::from("forty-two"))
    /// );
    /// assert_eq!(
    ///     Num2Words::parse("1e3").unwrap().to_words(),
    ///     Ok(String::from("one thousand"))
    /// );
    /// ```
    pub fn parse(num: &str) -> Option<Self> {
        let num = BigFloat::parse(num)?;
        if num.is_nan() {
            return None;
        }
        Some(Self {
            num,
            lang: Lang::English,
            output: Output::Cardinal,
            currency: Currency::DOLLAR,
            preferences: vec![],
        })
    }

    /// Sets the language of the output
    ///
    /// For all of the available languages, see [`Lang`].
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Words, Lang};
    /// assert_eq!(
    ///     Num2Words::new(42).lang(Lang::English).to_words(),
    ///     Ok(String::from("forty-two"))
    /// );
    /// ```
    pub fn lang(mut self, lang: Lang) -> Self {
        self.lang = lang;
        self
    }

    /// Sets the type of output to cardinal (`forty-two`)
    ///
    /// Example:
    /// ```
    /// use num2words::Num2Words;
    /// assert_eq!(
    ///     Num2Words::new(42).cardinal().to_words(),
    ///     Ok(String::from("forty-two"))
    /// );
    /// ```
    pub fn cardinal(mut self) -> Self {
        self.output = Output::Cardinal;
        self
    }

    /// Sets the type of output to ordinal (`forty-second`)
    ///
    /// Example:
    /// ```
    /// use num2words::Num2Words;
    /// assert_eq!(
    ///     Num2Words::new(42).ordinal().to_words(),
    ///     Ok(String::from("forty-second"))
    /// );
    /// ```
    pub fn ordinal(mut self) -> Self {
        self.output = Output::Ordinal;
        self
    }

    /// Sets the type of output to numbered ordinal (`42nd`)
    ///
    /// Example:
    /// ```
    /// use num2words::Num2Words;
    /// assert_eq!(
    ///     Num2Words::new(42).ordinal_num().to_words(),
    ///     Ok(String::from("42nd"))
    /// );
    /// ```
    pub fn ordinal_num(mut self) -> Self {
        self.output = Output::OrdinalNum;
        self
    }

    /// Sets the type of output to year (`nineteen oh-one`)
    ///
    /// Example:
    /// ```
    /// use num2words::Num2Words;
    /// assert_eq!(
    ///     Num2Words::new(1901).year().to_words(),
    ///     Ok(String::from("nineteen oh-one"))
    /// );
    /// ```
    pub fn year(mut self) -> Self {
        self.output = Output::Year;
        self
    }

    /// Sets the output to the currency it has been given
    ///
    /// For all of the available currencies, see [`Currency`].
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Words, Currency};
    /// assert_eq!(
    ///     Num2Words::new(42.01).currency(Currency::DOLLAR).to_words(),
    ///     Ok(String::from("forty-two dollars and one cent"))
    /// );
    /// ```
    pub fn currency(mut self, currency: Currency) -> Self {
        self.output = Output::Currency;
        self.currency = currency;
        self
    }

    /// Adds a preference parameter
    ///
    /// # English language accepts:
    /// * oh and/or nil as replacements for "zero"
    /// 
    /// # French language accepts:
    /// * feminine/f/féminin/feminin
    /// 
    /// * reformed/1990/rectifié/rectification
    ///
    /// # Spanish language accepts:
    /// * negativo/menos/bajo cero/prepended/appended
    /// * veinte
    /// * coma/punto
    /// * f/femenino/feminine
    /// * plural
    ///
    /// # Ukrainian language supports grammatical categories (bold - default):
    /// * Number: **singular/sing/однина/од**, plural/pl/множина/мн
    ///
    /// * Gender: **masculine/m/чоловічий/чол/ч**, feminine/f/жіночий/жін/ж, neuter/n/середній/сер/с
    ///
    /// * Declension: **nominative/nom/називний/н**, genitive/gen/родовий/р, dative/dat/давальний/д,\
    /// accusative/acc/знахідний/з, instrumental/inc/орудний/о, locative/loc/місцевий/м
    ///
    /// Examples:
    /// ```
    /// use num2words::{Num2Words, Lang};
    /// assert_eq!(
    ///     Num2Words::new(0.05).prefer("oh").to_words(),
    ///     Ok(String::from("point oh five"))
    /// );
    /// assert_eq!(
    ///     Num2Words::new(161).lang(Lang::French).prefer("f").prefer("reformed").to_words(),
    ///     Ok(String::from("cent-soixante-et-une"))
    /// );
    /// assert_eq!(
    ///     Num2Words::new(122.04).lang(Lang::Spanish).prefer("coma").prefer("veinte").to_words(),
    ///     Ok(String::from("ciento veinte y dos coma cero cuatro"))
    /// );
    /// assert_eq!(
    ///     Num2Words::new(51).lang(Lang::Ukrainian).prefer("орудний").to_words(),
    ///     Ok(String::from("пʼятдесятьма одним"))
    /// );
    /// ```
    pub fn prefer<T>(mut self, prefer: T) -> Self
    where
        T: Into<String>,
    {
        self.preferences.push(prefer.into());
        self
    }

    /// Builds the output
    pub fn to_words(self) -> Result<String, Num2Err> {
        let lang = lang::to_language(self.lang, self.preferences);
        match self.output {
            Output::Cardinal => lang.to_cardinal(self.num),
            Output::Currency => lang.to_currency(self.num, self.currency),
            Output::Ordinal => {
                if self.num.is_inf() {
                    return Err(Num2Err::InfiniteOrdinal);
                }
                if !self.num.frac().is_zero() {
                    return Err(Num2Err::FloatingOrdinal);
                }
                if self.num.is_negative() {
                    return Err(Num2Err::NegativeOrdinal);
                }
                lang.to_ordinal(self.num)
            }
            Output::OrdinalNum => {
                if self.num.is_inf() {
                    return Err(Num2Err::InfiniteOrdinal);
                }
                if !self.num.frac().is_zero() {
                    return Err(Num2Err::FloatingOrdinal);
                }
                if self.num.is_negative() {
                    return Err(Num2Err::NegativeOrdinal);
                }
                lang.to_ordinal_num(self.num)
            }
            Output::Year => {
                if self.num.is_inf() {
                    return Err(Num2Err::InfiniteYear);
                }
                if !self.num.frac().is_zero() {
                    return Err(Num2Err::FloatingYear);
                }
                lang.to_year(self.num)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_string_not_valid() {
        assert!(Num2Words::parse("not a number").is_none());
        assert!(Num2Words::parse("NAN").is_none());
    }
}
