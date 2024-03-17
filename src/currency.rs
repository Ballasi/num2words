use std::str::FromStr;

/// Defines currencies
///
/// Every three-letter variant is a valid ISO 4217 currency code. The only
/// exceptions are `DINAR`, `DOLLAR`, `PESO` and `RIYAL`, which are generic
/// terminology for the respective currencies.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum Currency {
    /// Dirham
    AED,
    /// Argentine peso
    ARS,
    /// Australian dollar
    AUD,
    /// Brazilian real
    BRL,
    /// Canadian dollar
    CAD,
    /// Swiss franc
    CHF,
    /// Chilean peso
    CLP,
    /// Chinese yuan
    CNY,
    /// Colombian peso
    COP,
    /// Costa Rican colón
    CRC,
    /// Dinar
    DINAR,
    /// Dollar
    DOLLAR,
    /// Algerian dinar
    DZD,
    /// Euro
    EUR,
    /// British pound
    GBP,
    /// Hong Kong dollar
    HKD,
    /// Indonesian rupiah
    IDR,
    /// Israeli new shekel
    ILS,
    /// Indian rupee
    INR,
    /// Japanese yen
    JPY,
    /// South Korean won
    KRW,
    /// Kuwaiti dinar
    KWD,
    /// Kazakhstani tenge
    KZT,
    /// Mexican peso
    MXN,
    /// Malaysian ringgit
    MYR,
    /// Norwegian krone
    NOK,
    /// New Zealand dollar
    NZD,
    /// Peruvian sol
    PEN,
    /// Peso
    PESO,
    /// Philippine peso
    PHP,
    /// Polish zloty
    PLN,
    /// Qatari riyal
    QAR,
    /// Riyal
    RIYAL,
    /// Russian ruble
    RUB,
    /// Saudi riyal
    SAR,
    /// Singapore dollar
    SGD,
    /// Thai baht
    THB,
    /// Turkish lira
    TRY,
    /// Taiwan dollar
    TWD,
    /// Ukrainian hryvnia
    UAH,
    /// US dollar
    USD,
    /// Uruguayan peso
    UYU,
    /// Vietnamese dong
    VND,
    /// South African rand
    ZAR,
}

impl Currency {
    /// Returns a default string representation for the currency
    ///
    /// Since many languages share the same work for a specific currency (like
    /// euro), it is easier and wiser for modularity to have a default value.
    pub fn default_string(&self, plural_form: bool) -> String {
        match self {
            Currency::AED => "dirham{}",
            Currency::ARS => "argentine peso{}",
            Currency::AUD => "australian dollar{}",
            Currency::BRL => {
                if plural_form {
                    "reais"
                } else {
                    "real"
                }
            }
            Currency::CAD => "canadian dollar{}",
            Currency::CHF => "franc{}",
            Currency::CLP => "chilean peso{}",
            Currency::CNY => "yuan{}",
            Currency::COP => "colombian peso{}",
            Currency::CRC => {
                if plural_form {
                    "colones"
                } else {
                    "colón"
                }
            }
            Currency::DINAR => "dinar{}",
            Currency::DOLLAR => "dollar{}",
            Currency::DZD => "algerian dinar{}",
            Currency::EUR => "euro{}",
            Currency::GBP => "pound{}",
            Currency::HKD => "hong kong dollar{}",
            Currency::IDR => "indonesian rupiah{}",
            Currency::ILS => "new shekel{}",
            Currency::INR => "rupee{}",
            Currency::JPY => "yen{}",
            Currency::KRW => "won{}",
            Currency::KWD => "kuwaiti dinar{}",
            Currency::KZT => "tenge{}",
            Currency::MXN => "mexican peso{}",
            Currency::MYR => "ringgit{}",
            Currency::NOK => "norwegian krone{}",
            Currency::NZD => "new zealand dollar{}",
            Currency::PEN => {
                if plural_form {
                    "soles"
                } else {
                    "sol"
                }
            }
            Currency::PESO => "peso{}",
            Currency::PHP => "philippine peso{}",
            Currency::PLN => "zloty{}",
            Currency::QAR => "qatari riyal{}",
            Currency::RIYAL => "riyal{}",
            Currency::RUB => "ruble{}",
            Currency::SAR => "saudi riyal{}",
            Currency::SGD => "singapore dollar{}",
            Currency::THB => "baht{}",
            Currency::TRY => "lira{}",
            Currency::TWD => "taiwan dollar{}",
            Currency::UAH => "hryvnia{}",
            Currency::USD => "US dollar{}",
            Currency::UYU => "uruguayan peso{}",
            Currency::VND => "dong{}",
            Currency::ZAR => "rand{}",
        }
        .replace("{}", if plural_form { "s" } else { "" })
    }

    /// Returns a default string representation for the cents of the currency
    pub fn default_subunit_string(&self, cent: &str, plural_form: bool) -> String {
        String::from(
            match self {
                Currency::AED | Currency::KWD => "fils",
                Currency::ARS | Currency::BRL | Currency::CLP | Currency::COP | Currency::MXN => {
                    "centavo{}"
                }
                Currency::CRC => "céntimo{}",
                Currency::IDR | Currency::MYR => "sen{}",
                Currency::KRW => "jeon{}",
                Currency::SAR => "halalat{}",
                Currency::THB => "satang{}",
                Currency::UAH => "kopiyok{}",
                Currency::UYU => "centesimo{}",
                Currency::VND => "xu{}",
                _ => cent,
            }
            .replace("{}", if plural_form { "s" } else { "" }),
        )
    }
}

impl FromStr for Currency {
    type Err = ();

    fn from_str(currency: &str) -> Result<Self, Self::Err> {
        match currency {
            "AED" => Ok(Currency::AED),
            "ARS" => Ok(Currency::ARS),
            "AUD" => Ok(Currency::AUD),
            "BRL" => Ok(Currency::BRL),
            "CAD" => Ok(Currency::CAD),
            "CHF" => Ok(Currency::CHF),
            "CLP" => Ok(Currency::CLP),
            "CNY" => Ok(Currency::CNY),
            "COP" => Ok(Currency::COP),
            "CRC" => Ok(Currency::CRC),
            "DINAR" => Ok(Currency::DINAR),
            "DOLLAR" => Ok(Currency::DOLLAR),
            "DZD" => Ok(Currency::DZD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "HKD" => Ok(Currency::HKD),
            "IDR" => Ok(Currency::IDR),
            "ILS" => Ok(Currency::ILS),
            "INR" => Ok(Currency::INR),
            "JPY" => Ok(Currency::JPY),
            "KRW" => Ok(Currency::KRW),
            "KWD" => Ok(Currency::KWD),
            "KZT" => Ok(Currency::KZT),
            "MXN" => Ok(Currency::MXN),
            "MYR" => Ok(Currency::MYR),
            "NOK" => Ok(Currency::NOK),
            "NZD" => Ok(Currency::NZD),
            "PEN" => Ok(Currency::PEN),
            "PESO" => Ok(Currency::PESO),
            "PHP" => Ok(Currency::PHP),
            "PLN" => Ok(Currency::PLN),
            "QAR" => Ok(Currency::QAR),
            "RIYAL" => Ok(Currency::RIYAL),
            "RUB" => Ok(Currency::RUB),
            "SAR" => Ok(Currency::SAR),
            "SGD" => Ok(Currency::SGD),
            "THB" => Ok(Currency::THB),
            "TRY" => Ok(Currency::TRY),
            "TWD" => Ok(Currency::TWD),
            "UAH" => Ok(Currency::UAH),
            "USD" => Ok(Currency::USD),
            "UYU" => Ok(Currency::UYU),
            "VND" => Ok(Currency::VND),
            "ZAR" => Ok(Currency::ZAR),
            _ => Err(()),
        }
    }
}
