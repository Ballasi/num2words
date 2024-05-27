use crate::{num2words::Num2Err, Currency, Language};
use num_bigfloat::BigFloat;

pub struct French {
    feminine: bool,
    reformed: bool,
    region: RegionFrench,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum RegionFrench {
    #[default]
    FR,
    BE,
    CH,
}

const UNITS: [&'static str; 9] = [
    "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf",
];

const TENS: [&'static str; 9] = [
    "dix",
    "vingt",
    "trente",
    "quarante",
    "cinquante",
    "soixante",
    "soixante-dix",
    "quatre-vingt",
    "quatre-vingt-dix",
];

const TEENS: [&'static str; 10] = [
    "dix", "onze", "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit",
    "dix-neuf",
];

const MEGAS: [&'static str; 33] = [
    "mille",
    "million",
    "milliard",
    "billion",
    "billiard",
    "trillion",
    "trilliard",
    "quadrillion",
    "quadrilliard",
    "quintillion",
    "quintilliard",
    "sextillion",
    "sextilliard",
    "septillion",
    "septilliard",
    "octillion",
    "octilliard",
    "nonillion",
    "nonilliard",
    "décillion",
    "décilliard",
    "unodécillion",
    "unodécilliard",
    "duodécillion",
    "duodécilliard",
    "trédécillion",
    "trédécilliard",
    "quattuordécillion",
    "quattuordécilliard",
    "quindécillion",
    "quindécilliard",
    "sexdécillion",
    "sexdécilliard",
];

impl French {
    pub fn new(feminine: bool, reformed: bool, region: RegionFrench) -> Self {
        Self {
            feminine,
            reformed,
            region,
        }
    }

    fn get_tens(&self, tens: usize) -> &str {
        match (tens, self.region) {
            (7, RegionFrench::BE) | (7, RegionFrench::CH) => "septante",
            (8, RegionFrench::CH) => "huitante",
            (9, RegionFrench::BE) | (9, RegionFrench::CH) => "nonante",
            _ => TENS[tens - 1],
        }
    }

    fn currencies(&self, currency: Currency, plural_form: bool) -> String {
        match currency {
            Currency::ARS => String::from("peso{} argentin"),
            Currency::AUD => String::from("dollar{} australien"),
            Currency::BRL => String::from(if plural_form { "réaux" } else { "réal" }),
            Currency::CAD => String::from("dollar{} canadien"),
            Currency::CLP => String::from("peso{} chilien"),
            Currency::COP => String::from("peso{} colombien"),
            Currency::DZD => String::from("dinar{} algérien"),
            Currency::GBP => String::from("livre{}"),
            Currency::HKD => String::from("dollar{} de Hong Kong"),
            Currency::IDR => String::from("roupie{} indonésienne"),
            Currency::ILS => String::from("shekel{}"),
            Currency::INR => String::from("roupie{}"),
            Currency::KWD => String::from("dinar{} koweïtien"),
            Currency::MXN => String::from("peso{} mexicain"),
            Currency::NOK => String::from("couronne{} norvégienne"),
            Currency::NZD => String::from("dollar{} néo-zélandais"),
            Currency::PHP => String::from("peso{} phillippin"),
            Currency::PLN => String::from("złoty{}"),
            Currency::QAR => String::from("riyal{} qatarien"),
            Currency::RUB => String::from("rouble{}"),
            Currency::SAR => String::from("riyal{} saoudien"),
            Currency::SGD => String::from("dollar{} de Singapour"),
            Currency::THB => String::from("baht{}"),
            Currency::TRY => String::from("lire{}"),
            Currency::TWD => String::from("dollar{} de Taïwan"),
            Currency::UAH => String::from("hryvnia{}"),
            Currency::USD => String::from("dollar{} américain"),
            Currency::UYU => String::from("peso{} uruguayen"),
            _ => currency.default_string(plural_form),
        }
        .replace("{}", if plural_form { "s" } else { "" })
    }

    fn cents(&self, _currency: Currency, plural_form: bool) -> String {
        match _currency {
            Currency::UAH => String::from("kopeck{}"),
            _ => _currency.default_subunit_string("centime{}", plural_form),
        }
        .replace("{}", if plural_form { "s" } else { "" })
    }

    fn split_thousands(&self, mut num: BigFloat) -> Vec<u64> {
        let mut thousands = Vec::new();
        let bf_1000 = BigFloat::from(1000);

        while !num.is_zero() {
            thousands.push((num % bf_1000).to_u64().unwrap());
            num = num.div(&bf_1000).int();
        }

        thousands
    }

    fn int_to_cardinal(&self, mut num: BigFloat) -> Result<String, Num2Err> {
        // special case zero
        if num.is_zero() {
            return Ok(String::from("zéro"));
        }

        // handling negative values
        let mut words = vec![];
        if num.is_negative() {
            words.push(String::from("moins"));
            num = -num;
        }

        // iterate over thousands
        for (i, triplet) in self.split_thousands(num).iter().enumerate().rev() {
            let hundreds = (triplet / 100 % 10) as usize;
            let tens = (triplet / 10 % 10) as usize;
            let units = (triplet % 10) as usize;

            if hundreds > 0 {
                if hundreds != 1 {
                    words.push(String::from(UNITS[hundreds - 1]));
                }
                words.push(String::from(
                    if i != 1 && tens == 0 && units == 0 && hundreds > 1 {
                        "cents"
                    } else {
                        "cent"
                    },
                ));
            }

            if tens != 0 || units != 0 {
                let et_string = if units == 1 {
                    if self.reformed {
                        "-et-"
                    } else {
                        " et "
                    }
                } else {
                    "-"
                };
                match units {
                    0 => {
                        if tens == 8 && i != 1 && self.region != RegionFrench::CH {
                            words.push(String::from("quatre-vingts"))
                        } else {
                            words.push(String::from(self.get_tens(tens)))
                        }
                    }
                    _ => match (tens, self.region) {
                        (0, _) => {
                            if i != 1 || units > 1 || hundreds > 0 {
                                // if i != 0, i.e., "un", "un million" but "mille"
                                // if units > 1, e.g. 2000 => "deux mille"
                                // if hundreds > 0, e.g. 201000 => "deux cent un mille"
                                words.push(String::from(
                                    if i == 0 && units == 1 && self.feminine {
                                        "une"
                                    } else {
                                        UNITS[units - 1]
                                    },
                                ));
                            }
                        }
                        (1, _) => words.push(String::from(TEENS[units])),
                        (7, RegionFrench::FR) => words.push(format!(
                            "{}{}{}",
                            self.get_tens(tens - 1),
                            et_string,
                            TEENS[units]
                        )),
                        (8, RegionFrench::FR) | (8, RegionFrench::BE) => words.push(format!(
                            "{}-{}",
                            self.get_tens(tens),
                            if i == 0 && units == 1 && self.feminine {
                                "une"
                            } else {
                                UNITS[units - 1]
                            }
                        )),
                        (9, RegionFrench::FR) => {
                            words.push(format!("{}-{}", self.get_tens(tens - 1), TEENS[units]))
                        }
                        _ => words.push(format!(
                            "{}{}{}",
                            self.get_tens(tens),
                            et_string,
                            if i == 0 && units == 1 && self.feminine {
                                "une"
                            } else {
                                UNITS[units - 1]
                            }
                        )),
                    },
                }
            }

            if i != 0 && triplet != &0 {
                if i > MEGAS.len() {
                    return Err(Num2Err::CannotConvert);
                }
                let plural_form = if hundreds == 0 && tens == 0 && units == 1 || i == 1 {
                    ""
                } else {
                    "s"
                };
                words.push(format!("{}{}", MEGAS[i - 1], plural_form));
            }
        }

        Ok(words.join(if self.reformed { "-" } else { " " }))
    }

    fn float_to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        let integral_part = num.int();
        let mut words: Vec<String> = vec![];

        if !integral_part.is_zero() {
            let integral_word = self.int_to_cardinal(integral_part)?;
            words.push(integral_word);
        }

        let mut ordinal_part = num.frac();
        if !ordinal_part.is_zero() {
            words.push(String::from("point"));
        }
        while !ordinal_part.is_zero() {
            let digit = (ordinal_part * BigFloat::from(10)).int();
            ordinal_part = (ordinal_part * BigFloat::from(10)).frac();
            words.push(match digit.to_u64().unwrap() {
                0 => String::from("zéro"),
                i => String::from(UNITS[i as usize - 1]),
            });
        }
        Ok(words.join(" "))
    }
}

impl Language for French {
    fn to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num.is_inf_pos() {
            Ok(String::from("infinité"))
        } else if num.is_inf_neg() {
            Ok(String::from("moins infinité"))
        } else if num.frac().is_zero() {
            self.int_to_cardinal(num)
        } else {
            self.float_to_cardinal(num)
        }
    }

    fn to_ordinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num == BigFloat::from(1) {
            return Ok(String::from("premier"));
        }
        let cardinal_word = self.to_cardinal(num)?;

        let mut words: Vec<String> = vec![];
        let mut split = cardinal_word.split_whitespace().peekable();

        while let Some(w) = split.next() {
            if split.peek().is_some() {
                // not last word, no modification needed
                words.push(String::from(w));
            } else {
                // last word, needs to be processed
                words.push(format!(
                    "{}ième",
                    if w.ends_with('e') {
                        &w[..w.len() - 1]
                    } else {
                        &w
                    }
                ));
            }
        }

        Ok(words.join(" "))
    }

    fn to_ordinal_num(&self, num: BigFloat) -> Result<String, Num2Err> {
        Ok(format!(
            "{}{}",
            num.to_u128().unwrap(),
            if num == BigFloat::from(1) {
                "er"
            } else {
                "ème"
            }
        ))
    }

    fn to_year(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num.is_negative() {
            Ok(format!("{} avant JC", self.to_cardinal(-num)?))
        } else {
            self.to_cardinal(num)
        }
    }

    fn to_currency(&self, num: BigFloat, currency: Currency) -> Result<String, Num2Err> {
        if num.is_inf() {
            Ok(format!(
                "{}une infinité de {}",
                if num.is_negative() { "moins " } else { "" },
                self.currencies(currency, true)
            ))
        } else if num.frac().is_zero() {
            let words = self.int_to_cardinal(num)?;
            Ok(format!(
                "{} {}",
                words,
                self.currencies(currency, num != BigFloat::from(1))
            ))
        } else {
            let integral_part = num.int();
            let cents_nb = (num * BigFloat::from(100)).int() % BigFloat::from(100);
            let cents_words = self.int_to_cardinal(cents_nb)?;
            let cents_suffix = self.cents(currency, cents_nb != BigFloat::from(1));
            let integral_word = self.to_currency(integral_part, currency)?;

            if cents_nb.is_zero() {
                Ok(integral_word)
            } else if integral_part.is_zero() {
                Ok(format!("{} {}", cents_words, cents_suffix))
            } else {
                Ok(format!(
                    "{} et {} {}",
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
            Num2Words::new(0).lang(Lang::French).cardinal().to_words(),
            Ok(String::from("zéro"))
        );
        assert_eq!(
            Num2Words::new(71).lang(Lang::French).cardinal().to_words(),
            Ok(String::from("soixante et onze"))
        );
        assert_eq!(
            Num2Words::new(80).lang(Lang::French).cardinal().to_words(),
            Ok(String::from("quatre-vingts"))
        );
        assert_eq!(
            Num2Words::new(200).lang(Lang::French).cardinal().to_words(),
            Ok(String::from("deux cents"))
        );
        assert_eq!(
            Num2Words::new(201).lang(Lang::French).cardinal().to_words(),
            Ok(String::from("deux cent un"))
        );
        assert_eq!(
            Num2Words::new(80000)
                .lang(Lang::French)
                .cardinal()
                .to_words(),
            Ok(String::from("quatre-vingt mille"))
        );
        assert_eq!(
            Num2Words::new(80000000)
                .lang(Lang::French)
                .cardinal()
                .to_words(),
            Ok(String::from("quatre-vingts millions"))
        );
        assert_eq!(
            Num2Words::new(91).lang(Lang::French).cardinal().to_words(),
            Ok(String::from("quatre-vingt-onze"))
        );
        assert_eq!(
            Num2Words::new(-10).lang(Lang::French).cardinal().to_words(),
            Ok(String::from("moins dix"))
        );
        assert_eq!(
            Num2Words::new(38123147081932i64)
                .lang(Lang::French)
                .cardinal()
                .to_words(),
            Ok(String::from(
                "trente-huit billions cent vingt-trois milliards cent \
                 quarante-sept millions quatre-vingt-un mille neuf cent \
                 trente-deux"
            ))
        );
        assert_eq!(
            Num2Words::new(100000000000i64)
                .lang(Lang::French)
                .cardinal()
                .to_words(),
            Ok(String::from("cent milliards"))
        );
    }

    #[test]
    fn test_feminine() {
        assert_eq!(
            Num2Words::new(1)
                .lang(Lang::French)
                .cardinal()
                .prefer("f")
                .to_words(),
            Ok(String::from("une"))
        );
        assert_eq!(
            Num2Words::new(61)
                .lang(Lang::French)
                .cardinal()
                .prefer("f")
                .to_words(),
            Ok(String::from("soixante et une"))
        );
        assert_eq!(
            Num2Words::new(61000)
                .lang(Lang::French)
                .cardinal()
                .prefer("f")
                .to_words(),
            Ok(String::from("soixante et un mille"))
        );
    }

    #[test]
    fn test_reformed() {
        assert_eq!(
            Num2Words::new(121000)
                .lang(Lang::French)
                .cardinal()
                .prefer("reformed")
                .to_words(),
            Ok(String::from("cent-vingt-et-un-mille"))
        );
    }

    #[test]
    fn test_french_be() {
        assert_eq!(
            Num2Words::new(71).lang(Lang::French_BE).to_words(),
            Ok(String::from("septante et un"))
        );
        assert_eq!(
            Num2Words::new(81).lang(Lang::French_BE).to_words(),
            Ok(String::from("quatre-vingt-un"))
        );
        assert_eq!(
            Num2Words::new(92).lang(Lang::French_BE).to_words(),
            Ok(String::from("nonante-deux"))
        );
    }

    #[test]
    fn test_french_ch() {
        assert_eq!(
            Num2Words::new(71).lang(Lang::French_CH).to_words(),
            Ok(String::from("septante et un"))
        );
        assert_eq!(
            Num2Words::new(80).lang(Lang::French_CH).to_words(),
            Ok(String::from("huitante"))
        );
        assert_eq!(
            Num2Words::new(92).lang(Lang::French_CH).to_words(),
            Ok(String::from("nonante-deux"))
        );
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(
            Num2Words::new(10).lang(Lang::French).ordinal().to_words(),
            Ok(String::from("dixième"))
        );
        assert_eq!(
            Num2Words::new(21).lang(Lang::French).ordinal().to_words(),
            Ok(String::from("vingt et unième"))
        );
        assert_eq!(
            Num2Words::new(102).lang(Lang::French).ordinal().to_words(),
            Ok(String::from("cent deuxième"))
        );
        assert_eq!(
            Num2Words::new(73).lang(Lang::French).ordinal().to_words(),
            Ok(String::from("soixante-treizième"))
        );
    }

    #[test]
    fn test_ordinal_num() {
        assert_eq!(
            Num2Words::new(1)
                .lang(Lang::French)
                .ordinal_num()
                .to_words(),
            Ok(String::from("1er"))
        );
        assert_eq!(
            Num2Words::new(2)
                .lang(Lang::French)
                .ordinal_num()
                .to_words(),
            Ok(String::from("2ème"))
        );
        assert_eq!(
            Num2Words::new(10)
                .lang(Lang::French)
                .ordinal_num()
                .to_words(),
            Ok(String::from("10ème"))
        );
        assert_eq!(
            Num2Words::new(21)
                .lang(Lang::French)
                .ordinal_num()
                .to_words(),
            Ok(String::from("21ème"))
        );
        assert_eq!(
            Num2Words::new(102)
                .lang(Lang::French)
                .ordinal_num()
                .to_words(),
            Ok(String::from("102ème"))
        );
        assert_eq!(
            Num2Words::new(73)
                .lang(Lang::French)
                .ordinal_num()
                .to_words(),
            Ok(String::from("73ème"))
        );
    }

    #[test]
    fn test_cardinal_float() {
        assert_eq!(
            Num2Words::new(12.5)
                .lang(Lang::French)
                .cardinal()
                .to_words(),
            Ok(String::from("douze point cinq"))
        );
        assert_eq!(
            Num2Words::new(12.51)
                .lang(Lang::French)
                .cardinal()
                .to_words(),
            Ok(String::from("douze point cinq un"))
        );
    }

    #[test]
    fn test_currency() {
        assert_eq!(
            Num2Words::new(1.01)
                .lang(Lang::French)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("un dollar et un centime"))
        );
        assert_eq!(
            Num2Words::new(4000)
                .lang(Lang::French)
                .currency(Currency::USD)
                .to_words(),
            Ok(String::from("quatre mille dollars américain"))
        );
        assert_eq!(
            Num2Words::new(1.)
                .lang(Lang::French)
                .currency(Currency::EUR)
                .to_words(),
            Ok(String::from("un euro"))
        );
        assert_eq!(
            Num2Words::new(0.20)
                .lang(Lang::French)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("vingt centimes"))
        );
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::French)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("zéro dollars"))
        );
    }

    #[test]
    fn test_year() {
        assert_eq!(
            Num2Words::new(1990).lang(Lang::French).year().to_words(),
            Ok(String::from("mille neuf cent quatre-vingt-dix"))
        );
        assert_eq!(
            Num2Words::new(0).lang(Lang::French).year().to_words(),
            Ok(String::from("zéro"))
        );
        assert_eq!(
            Num2Words::new(-44).lang(Lang::French).year().to_words(),
            Ok(String::from("quarante-quatre avant JC"))
        );
    }

    #[test]
    fn test_big_num() {
        use crate::lang::fr::MEGAS;
        use num_bigfloat::BigFloat;

        let mut num = BigFloat::from(1);
        let mut un = "";
        for m in MEGAS {
            num *= BigFloat::from(1000);
            assert_eq!(
                Num2Words::new(num).lang(Lang::French).cardinal().to_words(),
                Ok(String::from(format!("{}{}", un, m)))
            );
            un = "un ";
        }
    }

    #[test]
    fn test_infinity() {
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::French)
                .cardinal()
                .to_words(),
            Ok(String::from("infinité"))
        );
        assert_eq!(
            Num2Words::new(f64::NEG_INFINITY)
                .lang(Lang::French)
                .cardinal()
                .to_words(),
            Ok(String::from("moins infinité"))
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::French)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("une infinité de dollars"))
        );
    }
}
