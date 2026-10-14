use crate::{num2words::Num2Err, Currency, Language};
use num_bigfloat::BigFloat;

pub struct Turkish {
    joined: bool,
}

const UNITS: [&str; 9] = [
    "bir", "iki", "üç", "dört", "beş", "altı", "yedi", "sekiz", "dokuz",
];

const TENS: [&str; 9] = [
    "on", "yirmi", "otuz", "kırk", "elli", "altmış", "yetmiş", "seksen", "doksan",
];

// Short scale, as listed in the TDK dictionary up to kentilyon. Names beyond
// desilyon (10^33) are not consistently spelled across sources.
const MEGAS: [&str; 11] = [
    "bin",
    "milyon",
    "milyar",
    "trilyon",
    "katrilyon",
    "kentilyon",
    "seksilyon",
    "septilyon",
    "oktilyon",
    "nonilyon",
    "desilyon",
];

impl Turkish {
    pub fn new(joined: bool) -> Self {
        Self { joined }
    }

    fn currencies(&self, currency: Currency) -> &'static str {
        match currency {
            Currency::AED => "dirhem",
            Currency::ARS => "Arjantin pesosu",
            Currency::AUD => "Avustralya doları",
            Currency::BRL => "Brezilya reali",
            Currency::CAD => "Kanada doları",
            Currency::CHF => "İsviçre frangı",
            Currency::CLP => "Şili pesosu",
            Currency::CNY => "yuan",
            Currency::COP => "Kolombiya pesosu",
            Currency::CRC => "Kosta Rika kolonu",
            Currency::DINAR => "dinar",
            Currency::DOLLAR => "dolar",
            Currency::DZD => "Cezayir dinarı",
            Currency::EUR => "avro",
            Currency::GBP => "İngiliz sterlini",
            Currency::HKD => "Hong Kong doları",
            Currency::IDR => "Endonezya rupisi",
            Currency::ILS => "yeni şekel",
            Currency::INR => "Hint rupisi",
            Currency::JPY => "yen",
            Currency::KRW => "won",
            Currency::KWD => "Kuveyt dinarı",
            Currency::KZT => "tenge",
            Currency::MXN => "Meksika pesosu",
            Currency::MYR => "ringgit",
            Currency::NOK => "Norveç kronu",
            Currency::NZD => "Yeni Zelanda doları",
            Currency::PEN => "sol",
            Currency::PESO => "peso",
            Currency::PHP => "Filipin pesosu",
            Currency::PLN => "zloti",
            Currency::QAR => "Katar riyali",
            Currency::RIYAL => "riyal",
            Currency::RUB => "ruble",
            Currency::SAR => "Suudi riyali",
            Currency::SGD => "Singapur doları",
            Currency::THB => "baht",
            Currency::TRY => "Türk lirası",
            Currency::TWD => "Tayvan doları",
            Currency::UAH => "grivna",
            Currency::USD => "ABD doları",
            Currency::UYU => "Uruguay pesosu",
            Currency::VND => "dong",
            Currency::ZAR => "rand",
        }
    }

    fn cents(&self, currency: Currency) -> &'static str {
        match currency {
            Currency::AED | Currency::KWD => "fils",
            Currency::ARS | Currency::BRL | Currency::CLP | Currency::COP | Currency::MXN => {
                "sentavo"
            }
            Currency::CHF | Currency::DZD => "santim",
            Currency::CRC => "sentimo",
            Currency::GBP => "peni",
            Currency::IDR | Currency::MYR => "sen",
            Currency::KRW => "jeon",
            Currency::RUB | Currency::UAH => "kopek",
            Currency::SAR => "halala",
            Currency::THB => "satang",
            Currency::TRY => "kuruş",
            Currency::UYU => "sentesimo",
            Currency::VND => "xu",
            _ => "sent",
        }
    }

    fn split_thousands(&self, mut num: BigFloat) -> Vec<u64> {
        let mut thousands = Vec::new();
        let bf_1000 = BigFloat::from(1000);

        while !num.is_zero() {
            thousands.push((num % bf_1000).to_u64().unwrap());
            num /= bf_1000;
        }

        thousands
    }

    fn int_to_cardinal(&self, mut num: BigFloat) -> Result<String, Num2Err> {
        // special case zero
        if num.is_zero() {
            return Ok(String::from("sıfır"));
        }

        // handling negative values
        let negative = num.is_negative();
        if negative {
            num = -num;
        }

        // iterate over thousands
        let mut words = vec![];
        for (i, triplet) in self.split_thousands(num).iter().enumerate().rev() {
            if *triplet == 0 {
                continue;
            }

            let hundreds = (triplet / 100 % 10) as usize;
            let tens = (triplet / 10 % 10) as usize;
            let units = (triplet % 10) as usize;

            if hundreds > 0 {
                // 100 => yüz, not bir yüz
                if hundreds > 1 {
                    words.push(UNITS[hundreds - 1]);
                }
                words.push("yüz");
            }

            if tens > 0 {
                words.push(TENS[tens - 1]);
            }

            // 1000 => bin, not bir bin; but 1000000 => bir milyon
            if units > 0 && !(i == 1 && *triplet == 1) {
                words.push(UNITS[units - 1]);
            }

            if i != 0 {
                if i > MEGAS.len() {
                    return Err(Num2Err::CannotConvert);
                }
                words.push(MEGAS[i - 1]);
            }
        }

        // "eksi" is always written apart, even when the number is joined
        let words = words.join(if self.joined { "" } else { " " });
        Ok(if negative {
            format!("eksi {}", words)
        } else {
            words
        })
    }

    fn float_to_cardinal(&self, mut num: BigFloat) -> Result<String, Num2Err> {
        let mut words: Vec<String> = vec![];

        // the sign is handled here so that -0.5 does not lose it
        if num.is_negative() {
            words.push(String::from("eksi"));
            num = -num;
        }

        words.push(self.int_to_cardinal(num.int())?);
        words.push(String::from("virgül"));

        // the decimal part is read as a number, e.g. 12.51 => on iki virgül
        // elli bir, while its leading zeros are read one by one, e.g. 2.05 =>
        // iki virgül sıfır beş
        let mut decimal_part = num.frac();
        let mut decimals = BigFloat::from(0);
        let mut leading_zero = true;
        while !decimal_part.is_zero() {
            let digit = (decimal_part * BigFloat::from(10)).int();
            decimal_part = (decimal_part * BigFloat::from(10)).frac();

            if leading_zero && digit.is_zero() {
                words.push(String::from("sıfır"));
            } else {
                leading_zero = false;
                decimals = decimals * BigFloat::from(10) + digit;
            }
        }
        words.push(self.int_to_cardinal(decimals)?);

        Ok(words.join(" "))
    }

    fn ordinal_suffix(&self, word: &str) -> String {
        // dört => dördüncü
        let stem = match word.strip_suffix("dört") {
            Some(prefix) => format!("{}dörd", prefix),
            None => String::from(word),
        };

        // vowel harmony on the last vowel of the word
        let vowel = match word.chars().rev().find(|c| "aeıioöuü".contains(*c)) {
            Some('a') | Some('ı') => "ı",
            Some('e') | Some('i') => "i",
            Some('o') | Some('u') => "u",
            _ => "ü",
        };

        if stem.ends_with(|c| "aeıioöuü".contains(c)) {
            // e.g. iki => ikinci, altı => altıncı
            format!("{}nc{}", stem, vowel)
        } else {
            // e.g. bir => birinci, üç => üçüncü
            format!("{}{}nc{}", stem, vowel, vowel)
        }
    }
}

impl Language for Turkish {
    fn to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num.is_inf_pos() {
            Ok(String::from("sonsuz"))
        } else if num.is_inf_neg() {
            Ok(String::from("eksi sonsuz"))
        } else if num.frac().is_zero() {
            self.int_to_cardinal(num)
        } else {
            self.float_to_cardinal(num)
        }
    }

    fn to_ordinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        // the suffix only applies to the last word, e.g. yirmi birinci
        let cardinal_word = self.int_to_cardinal(num)?;
        Ok(self.ordinal_suffix(&cardinal_word))
    }

    fn to_ordinal_num(&self, num: BigFloat) -> Result<String, Num2Err> {
        // as recommended by TDK, a dot follows the number, e.g. 42.
        Ok(format!("{}.", num.to_u128().ok_or(Num2Err::CannotConvert)?))
    }

    fn to_year(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num.is_negative() {
            Ok(format!("milattan önce {}", self.int_to_cardinal(-num)?))
        } else {
            self.int_to_cardinal(num)
        }
    }

    fn to_currency(&self, num: BigFloat, currency: Currency) -> Result<String, Num2Err> {
        // currency names are never pluralized after a number in Turkish
        if num.is_inf() {
            Ok(format!(
                "{}sonsuz {}",
                if num.is_negative() { "eksi " } else { "" },
                self.currencies(currency)
            ))
        } else if num.is_negative() {
            // amounts rounding to zero, e.g. -0.001, are not negative
            if (-num * BigFloat::from(100)).int().is_zero() {
                return self.to_currency(BigFloat::from(0), currency);
            }
            Ok(format!("eksi {}", self.to_currency(-num, currency)?))
        } else if num.frac().is_zero() {
            let words = self.int_to_cardinal(num)?;
            Ok(format!("{} {}", words, self.currencies(currency)))
        } else {
            let integral_part = num.int();
            let cents_nb = (num * BigFloat::from(100)).int() % BigFloat::from(100);
            let cents_words = self.int_to_cardinal(cents_nb)?;
            let cents_suffix = self.cents(currency);
            let integral_word = self.to_currency(integral_part, currency)?;

            if cents_nb.is_zero() {
                Ok(integral_word)
            } else if integral_part.is_zero() {
                Ok(format!("{} {}", cents_words, cents_suffix))
            } else {
                Ok(format!(
                    "{} {} {}",
                    integral_word, cents_words, cents_suffix
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_cardinal() {
        assert_eq!(
            Num2Words::new(0).lang(Lang::Turkish).cardinal().to_words(),
            Ok(String::from("sıfır"))
        );
        assert_eq!(
            Num2Words::new(-10)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("eksi on"))
        );
        assert_eq!(
            Num2Words::new(11).lang(Lang::Turkish).cardinal().to_words(),
            Ok(String::from("on bir"))
        );
        assert_eq!(
            Num2Words::new(100)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("yüz"))
        );
        assert_eq!(
            Num2Words::new(101)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("yüz bir"))
        );
        assert_eq!(
            Num2Words::new(254)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("iki yüz elli dört"))
        );
        assert_eq!(
            Num2Words::new(1000)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("bin"))
        );
        assert_eq!(
            Num2Words::new(1001)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("bin bir"))
        );
        assert_eq!(
            Num2Words::new(21000)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("yirmi bir bin"))
        );
        assert_eq!(
            Num2Words::new(101000)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("yüz bir bin"))
        );
        assert_eq!(
            Num2Words::new(1000000)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("bir milyon"))
        );
        assert_eq!(
            Num2Words::new(1001000)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("bir milyon bin"))
        );
        assert_eq!(
            Num2Words::new(38123147081932i64)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from(
                "otuz sekiz trilyon yüz yirmi üç milyar yüz kırk yedi milyon \
                 seksen bir bin dokuz yüz otuz iki"
            ))
        );
        assert_eq!(
            Num2Words::new(100000000000i64)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("yüz milyar"))
        );
    }

    #[test]
    fn test_joined() {
        assert_eq!(
            Num2Words::new(1984)
                .lang(Lang::Turkish)
                .prefer("bitişik")
                .to_words(),
            Ok(String::from("bindokuzyüzseksendört"))
        );
        assert_eq!(
            Num2Words::new(-21)
                .lang(Lang::Turkish)
                .prefer("joined")
                .to_words(),
            Ok(String::from("eksi yirmibir"))
        );
        assert_eq!(
            Num2Words::new(24)
                .lang(Lang::Turkish)
                .prefer("bitisik")
                .ordinal()
                .to_words(),
            Ok(String::from("yirmidördüncü"))
        );
        assert_eq!(
            Num2Words::new(650.35)
                .lang(Lang::Turkish)
                .prefer("bitişik")
                .currency(Currency::TRY)
                .to_words(),
            Ok(String::from("altıyüzelli Türk lirası otuzbeş kuruş"))
        );
    }

    #[test]
    fn test_ordinal() {
        let ordinals = [
            (0, "sıfırıncı"),
            (1, "birinci"),
            (2, "ikinci"),
            (3, "üçüncü"),
            (4, "dördüncü"),
            (5, "beşinci"),
            (6, "altıncı"),
            (7, "yedinci"),
            (8, "sekizinci"),
            (9, "dokuzuncu"),
            (10, "onuncu"),
            (20, "yirminci"),
            (30, "otuzuncu"),
            (40, "kırkıncı"),
            (50, "ellinci"),
            (60, "altmışıncı"),
            (70, "yetmişinci"),
            (80, "sekseninci"),
            (90, "doksanıncı"),
            (100, "yüzüncü"),
            (1000, "bininci"),
            (1000000, "bir milyonuncu"),
            (1000000000, "bir milyarıncı"),
            (21, "yirmi birinci"),
            (44, "kırk dördüncü"),
            (102, "yüz ikinci"),
            (1923, "bin dokuz yüz yirmi üçüncü"),
        ];
        for (num, word) in ordinals {
            assert_eq!(
                Num2Words::new(num).lang(Lang::Turkish).ordinal().to_words(),
                Ok(String::from(word))
            );
        }
        assert_eq!(
            Num2Words::new(-1).lang(Lang::Turkish).ordinal().to_words(),
            Err(num2words::Num2Err::NegativeOrdinal)
        );
        assert_eq!(
            Num2Words::new(1.2).lang(Lang::Turkish).ordinal().to_words(),
            Err(num2words::Num2Err::FloatingOrdinal)
        );
    }

    #[test]
    fn test_ordinal_num() {
        assert_eq!(
            Num2Words::new(1)
                .lang(Lang::Turkish)
                .ordinal_num()
                .to_words(),
            Ok(String::from("1."))
        );
        assert_eq!(
            Num2Words::new(42)
                .lang(Lang::Turkish)
                .ordinal_num()
                .to_words(),
            Ok(String::from("42."))
        );
        assert_eq!(
            Num2Words::new(-42)
                .lang(Lang::Turkish)
                .ordinal_num()
                .to_words(),
            Err(num2words::Num2Err::NegativeOrdinal)
        );
    }

    #[test]
    fn test_cardinal_float() {
        assert_eq!(
            Num2Words::new(12.5)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("on iki virgül beş"))
        );
        assert_eq!(
            Num2Words::new(12.51)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("on iki virgül elli bir"))
        );
        assert_eq!(
            Num2Words::new(2.05)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("iki virgül sıfır beş"))
        );
        assert_eq!(
            Num2Words::new(0.005)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("sıfır virgül sıfır sıfır beş"))
        );
        assert_eq!(
            Num2Words::new(-0.5)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("eksi sıfır virgül beş"))
        );
        assert_eq!(
            Num2Words::new(-3.25)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("eksi üç virgül yirmi beş"))
        );
    }

    #[test]
    fn test_currency() {
        assert_eq!(
            Num2Words::new(1.01)
                .lang(Lang::Turkish)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("bir dolar bir sent"))
        );
        assert_eq!(
            Num2Words::new(4000)
                .lang(Lang::Turkish)
                .currency(Currency::USD)
                .to_words(),
            Ok(String::from("dört bin ABD doları"))
        );
        assert_eq!(
            Num2Words::new(42.50)
                .lang(Lang::Turkish)
                .currency(Currency::TRY)
                .to_words(),
            Ok(String::from("kırk iki Türk lirası elli kuruş"))
        );
        assert_eq!(
            Num2Words::new(1.)
                .lang(Lang::Turkish)
                .currency(Currency::EUR)
                .to_words(),
            Ok(String::from("bir avro"))
        );
        assert_eq!(
            Num2Words::new(0.20)
                .lang(Lang::Turkish)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("yirmi sent"))
        );
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Turkish)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("sıfır dolar"))
        );
        assert_eq!(
            Num2Words::new(-1.5)
                .lang(Lang::Turkish)
                .currency(Currency::TRY)
                .to_words(),
            Ok(String::from("eksi bir Türk lirası elli kuruş"))
        );
        assert_eq!(
            Num2Words::new(-0.001)
                .lang(Lang::Turkish)
                .currency(Currency::TRY)
                .to_words(),
            Ok(String::from("sıfır Türk lirası"))
        );
    }

    #[test]
    fn test_year() {
        assert_eq!(
            Num2Words::new(1923).lang(Lang::Turkish).year().to_words(),
            Ok(String::from("bin dokuz yüz yirmi üç"))
        );
        assert_eq!(
            Num2Words::new(2024).lang(Lang::Turkish).year().to_words(),
            Ok(String::from("iki bin yirmi dört"))
        );
        assert_eq!(
            Num2Words::new(0).lang(Lang::Turkish).year().to_words(),
            Ok(String::from("sıfır"))
        );
        assert_eq!(
            Num2Words::new(-44).lang(Lang::Turkish).year().to_words(),
            Ok(String::from("milattan önce kırk dört"))
        );
        assert_eq!(
            Num2Words::new(1.1).lang(Lang::Turkish).year().to_words(),
            Err(num2words::Num2Err::FloatingYear)
        );
    }

    #[test]
    fn test_big_num() {
        use crate::lang::tr::MEGAS;
        use num_bigfloat::BigFloat;

        let mut num = BigFloat::from(1);
        let mut bir = "";
        for m in MEGAS {
            num *= BigFloat::from(1000);
            assert_eq!(
                Num2Words::new(num)
                    .lang(Lang::Turkish)
                    .cardinal()
                    .to_words(),
                Ok(format!("{}{}", bir, m))
            );
            bir = "bir ";
        }

        assert_eq!(
            Num2Words::new(1e100)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Err(num2words::Num2Err::CannotConvert)
        );
    }

    #[test]
    fn test_infinity() {
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("sonsuz"))
        );
        assert_eq!(
            Num2Words::new(f64::NEG_INFINITY)
                .lang(Lang::Turkish)
                .cardinal()
                .to_words(),
            Ok(String::from("eksi sonsuz"))
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Turkish)
                .ordinal()
                .to_words(),
            Err(num2words::Num2Err::InfiniteOrdinal)
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Turkish)
                .year()
                .to_words(),
            Err(num2words::Num2Err::InfiniteYear)
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Turkish)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("sonsuz dolar"))
        );
    }
}
