use std::str::FromStr;

/// Defines currencies
///
/// `DOLLAR` is the more generic currency. Any other currencies using dollars
/// should use the localization information (e.g. `USD` becomes `US dollar[s]`
/// and `CAD` becomes `canadian dollar[s]` in English).
///
/// Other than `DOLLAR`, every currency is defined by their ISO 4217 code.
#[derive(Clone, Copy)]
pub enum Currency {
    AUD,
    CAD,
    DOLLAR,
    EUR,
    GBP,
    USD,
}

impl Currency {
    /// Returns a default string representation for the currency
    ///
    /// Since many languages share the same work for a specific currency (like
    /// euro), it is easier and wiser for modularity to have a default value.
    ///
    /// Contains a `{}` for the plural form
    pub fn default_string(&self) -> &str {
        match self {
            Currency::AUD => "australian dollar{}",
            Currency::CAD => "canadian dollar{}",
            Currency::DOLLAR => "dollar{}",
            Currency::EUR => "euro{}",
            Currency::GBP => "pound{}",
            Currency::USD => "US dollar{}",
        }
    }

    /// Returns a default string representation for the cents of the currency
    ///
    /// Contains a `{}` for the plural form
    pub fn default_cent_string(&self) -> &str {
        "cent{}"
    }
}

impl FromStr for Currency {
    type Err = ();

    fn from_str(currency: &str) -> Result<Self, Self::Err> {
        match currency {
            "AUD" => Ok(Currency::AUD),
            "CAD" => Ok(Currency::CAD),
            "DOLLAR" => Ok(Currency::DOLLAR),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "USD" => Ok(Currency::USD),
            _ => Err(()),
        }
    }
}
