use std::str::FromStr;

/// Defines currencies
///
/// Every three-letter variant is a valid ISO 4217 currency code. The only
/// exceptions are `DINAR`, `DOLLAR`, `PESO` and `RIYAL`, which are generic
/// terminology for the respective currencies.
///
/// Here is a summary of all of the available currencies:
///
/// | Enum variant       | CLI argument  | Currency name in English |
/// | ------------------ | ------------- | ------------------------ |
/// | `Currency::AED`    | `AED`         | Dirham                   |
/// | `Currency::ARS`    | `ARS`         | Argentine peso           |
/// | `Currency::AUD`    | `AUD`         | Australian dollar        |
/// | `Currency::BRL`    | `BRL`         | Brazilian real           |
/// | `Currency::CAD`    | `CAD`         | Canadian dollar          |
/// | `Currency::CHF`    | `CHF`         | Swiss franc              |
/// | `Currency::CLP`    | `CLP`         | Chilean peso             |
/// | `Currency::CNY`    | `CNY`         | Chinese yuan             |
/// | `Currency::COP`    | `COP`         | Colombian peso           |
/// | `Currency::CRC`    | `CRC`         | Costa Rican colón        |
/// | `Currency::DINAR`  | `DINAR`       | Dinar                    |
/// | `Currency::DOLLAR` | `DOLLAR`      | Dollar                   |
/// | `Currency::DZD`    | `DZD`         | Algerian dinar           |
/// | `Currency::EUR`    | `EUR`         | Euro                     |
/// | `Currency::GBP`    | `GBP`         | British pound            |
/// | `Currency::HKD`    | `HKD`         | Hong Kong dollar         |
/// | `Currency::IDR`    | `IDR`         | Indonesian rupiah        |
/// | `Currency::ILS`    | `ILS`         | Israeli new shekel       |
/// | `Currency::INR`    | `INR`         | Indian rupee             |
/// | `Currency::JPY`    | `JPY`         | Japanese yen             |
/// | `Currency::KRW`    | `KRW`         | South Korean won         |
/// | `Currency::KWD`    | `KWD`         | Kuwaiti dinar            |
/// | `Currency::KZT`    | `KZT`         | Kazakhstani tenge        |
/// | `Currency::MXN`    | `MXN`         | Mexican peso             |
/// | `Currency::MYR`    | `MYR`         | Malaysian ringgit        |
/// | `Currency::NOK`    | `NOK`         | Norwegian krone          |
/// | `Currency::NZD`    | `NZD`         | New Zealand dollar       |
/// | `Currency::PEN`    | `PEN`         | Peruvian sol             |
/// | `Currency::PESO`   | `PESO`        | Peso                     |
/// | `Currency::PHP`    | `PHP`         | Philippine peso          |
/// | `Currency::PLN`    | `PLN`         | Polish zloty             |
/// | `Currency::QAR`    | `QAR`         | Qatari riyal             |
/// | `Currency::RIYAL`  | `RIYAL`       | Riyal                    |
/// | `Currency::RUB`    | `RUB`         | Russian ruble            |
/// | `Currency::SAR`    | `SAR`         | Saudi riyal              |
/// | `Currency::SGD`    | `SGD`         | Singapore dollar         |
/// | `Currency::THB`    | `THB`         | Thai baht                |
/// | `Currency::TRY`    | `TRY`         | Turkish lira             |
/// | `Currency::TWD`    | `TWD`         | Taiwan dollar            |
/// | `Currency::UAH`    | `UAH`         | Ukrainian hryvnia        |
/// | `Currency::USD`    | `USD`         | US dollar                |
/// | `Currency::UYU`    | `UYU`         | Uruguayan peso           |
/// | `Currency::VND`    | `VND`         | Vietnamese dong          |
/// | `Currency::ZAR`    | `ZAR`         | South African rand       |
#[derive(Clone, Copy)]
pub enum Currency {
    AED,
    ARS,
    AUD,
    BRL,
    CAD,
    CHF,
    CLP,
    CNY,
    COP,
    CRC,
    DINAR,
    DOLLAR,
    DZD,
    EUR,
    GBP,
    HKD,
    IDR,
    ILS,
    INR,
    JPY,
    KRW,
    KWD,
    KZT,
    MXN,
    MYR,
    NOK,
    NZD,
    PEN,
    PESO,
    PHP,
    PLN,
    QAR,
    RIYAL,
    RUB,
    SAR,
    SGD,
    THB,
    TRY,
    TWD,
    UAH,
    USD,
    UYU,
    VND,
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
