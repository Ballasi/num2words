use std::str::FromStr;

/// Defines currencies
///
/// `DOLLAR` is the more generic currency. Any other currencies using dollars
/// should use the localization information (e.g. `USD` becomes `US dollar[s]`
/// and `CAD` becomes `canadian dollar[s]` in English).
pub enum Currency {
    AUD,
    CAD,
    DOLLAR,
    EUR,
    GBP,
    USD,
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
