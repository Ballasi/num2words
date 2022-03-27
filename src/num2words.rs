use crate::{lang, Currency, Lang, Language, Number, Output};

/// Error type returned by the builder
#[derive(Debug, PartialEq)]
pub enum Num2Err {
    CannotConvert,
    NegativeOrdinal,
    FloatingOrdinal,
    FloatingYear,
}

pub struct Num2Words {
    num: Number,
    lang: Lang,
    output: Output,
    currency: Currency,
}

impl Num2Words {
    pub fn new<T>(num: T) -> Self
    where
        T: Into<Number>,
    {
        Self {
            num: num.into(),
            lang: Lang::English,
            output: Output::Cardinal,
            currency: Currency::DOLLAR,
        }
    }

    pub fn lang(mut self, lang: Lang) -> Self {
        self.lang = lang;
        self
    }

    pub fn output(mut self, output: Output) -> Self {
        self.output = output;
        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.output = Output::Currency;
        self.currency = currency;
        self
    }

    /// Macro to convert numbers to words
    ///
    /// # Usage
    ///
    /// The macro can be called with two optional parameters: `lang` and `to`.
    ///
    /// Example:
    /// ```
    /// use num2words::*;
    /// assert_eq!(
    ///     Num2Words::new(42).to_words(), Ok(String::from("forty-two"))
    /// );
    /// assert_eq!(
    ///     Num2Words::new(42).lang(Lang::English).output(Output::Ordinal).to_words(),
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
    pub fn to_words(self) -> Result<String, Num2Err> {
        let lang = lang::to_language(self.lang);
        match self.output {
            Output::Cardinal => lang.to_cardinal(self.num),
            Output::Currency => lang.to_currency(self.num, self.currency),
            Output::Ordinal => {
                if let Number::Int(n) = self.num {
                    if n < 0 {
                        Err(Num2Err::NegativeOrdinal)
                    } else {
                        lang.to_ordinal(self.num)
                    }
                } else {
                    Err(Num2Err::FloatingOrdinal)
                }
            }
            Output::OrdinalNum => {
                if let Number::Int(n) = self.num {
                    if n < 0 {
                        Err(Num2Err::NegativeOrdinal)
                    } else {
                        lang.to_ordinal_num(self.num)
                    }
                } else {
                    Err(Num2Err::FloatingOrdinal)
                }
            }
            Output::Year => lang.to_year(self.num),
        }
    }
}
