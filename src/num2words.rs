use crate::{lang, Currency, Lang, Language, Number, Output};

/// Error type returned by the builder
#[derive(Debug, PartialEq)]
pub enum Num2Err {
    /// General error, language cannot convert number
    CannotConvert,
    /// Request of a negative ordinal
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Err, Num2Words, Output};
    /// assert_eq!(
    ///     Num2Words::new(-42).output(Output::Ordinal).to_words(),
    ///     Err(Num2Err::NegativeOrdinal)
    /// );
    /// ```
    NegativeOrdinal,
    /// Request of a float ordinal
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Err, Num2Words, Output};
    /// assert_eq!(
    ///     Num2Words::new(42.01).output(Output::Ordinal).to_words(),
    ///     Err(Num2Err::FloatingOrdinal)
    /// );
    /// ```
    FloatingOrdinal,
    /// Request of a float year
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Err, Num2Words, Output};
    /// assert_eq!(
    ///     Num2Words::new(42.01).output(Output::Year).to_words(),
    ///     Err(Num2Err::FloatingYear)
    /// );
    /// ```
    FloatingYear,
}

/// Builder for `num2words`
pub struct Num2Words {
    num: Number,
    lang: Lang,
    output: Output,
    currency: Currency,
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
    /// ```
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

    /// Sets the type of output
    ///
    /// For all of the available outputs, see [`Output`].
    ///
    /// Example:
    /// ```
    /// use num2words::{Num2Words, Output};
    /// assert_eq!(
    ///     Num2Words::new(42).output(Output::Ordinal).to_words(),
    ///     Ok(String::from("forty-second"))
    /// );
    /// ```
    pub fn output(mut self, output: Output) -> Self {
        self.output = output;
        self
    }

    /// Sets the output to the currency it has been given
    ///
    /// For all of the available currencies, see [`Currency`].
    ///
    /// **Warning**: this overrides the output to `Output::Currency`.
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

    /// Builds the output
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
