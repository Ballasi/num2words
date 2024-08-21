use crate::{num2words::Num2Err, Currency, Language};
use num_bigfloat::BigFloat;

pub struct Dutch {}

const UNITS: [&str; 9] = [
    "één", "twee", "drie", "vier", "vijf", "zes", "zeven", "acht", "negen",
];

const TENS: [&str; 9] = [
    "tien", "twintig", "dertig", "veertig", "vijftig", "zestig", "zeventig", "tachtig", "negentig",
];

const TEENS: [&str; 10] = [
    "tien",
    "elf",
    "twaalf",
    "dertien",
    "veertien",
    "vijftien",
    "zestien",
    "zeventien",
    "achtien",
    "negentien",
];

// As defined by the AHD4, CED, RHD2, W3 and UM authorities
// For more information, see
// https://en.wikipedia.org/wiki/Names_of_large_numbers
const MEGAS: [&str; 21] = [
    "duizend",
    "miljoen",
    "miljard",
    "biljoen",
    "biljard",
    "triljoen",
    "triljard",
    "quadriljoen",
    "quadriljard",
    "quintiljoen",
    "quintiljard",
    "sextiljoen",
    "sextiljard",
    "septiljoen",
    "septiljard",
    "octiljoen",
    "octiljard",
    "noniljoen",
    "noniljard",
    "deciljoen",
    "deciljard",
];

fn space_words(words: &mut Vec<String>) {
    for x in (0..words.len()).rev() {
        let word: String = words.get(x).unwrap().clone();
        if ["minus"].contains(&word.as_str()) {
            words.insert(x + 1, " ".to_string());
        } else if ["komma"].contains(&word.as_str()) {
            words.insert(x, " ".to_string());
        } else if MEGAS.contains(&word.as_str()) {
            if x != words.len() - 1 {
                words.insert(x + 1, " ".to_string());
            }
            if !word.eq("duizend") {
                words.insert(x, " ".to_string());
            }
        }
    }
}

impl Dutch {
    pub fn new() -> Self {
        Self {}
    }

    fn currencies(&self, currency: Currency) -> String {
        currency.default_string(false)
    }

    fn cents(&self, currency: Currency) -> String {
        currency.default_subunit_string("cent{}", false)
    }

    fn split_thousands(&self, mut num: BigFloat) -> Vec<u64> {
        let mut thousands = Vec::new();
        let bf_1000 = BigFloat::from(1000);

        while !num.int().is_zero() {
            thousands.push((num % bf_1000).to_u64().unwrap());
            num /= bf_1000;
        }

        thousands
    }

    fn int_to_cardinal(&self, mut num: BigFloat) -> Result<String, Num2Err> {
        // special case zero
        if num.is_zero() {
            return Ok(String::from(
                "nul"
            ));
        }

        // handling negative values
        let mut words = vec![];
        if num.is_negative() {
            words.push(String::from("minus"));
            num = -num;
        }

        // iterate over thousands
        let mut first_elem = true;
        for (i, triplet) in self.split_thousands(num).iter().enumerate().rev() {
            let hundreds = (triplet / 100 % 10) as usize;
            let tens = (triplet / 10 % 10) as usize;
            let units = (triplet % 10) as usize;

            if hundreds > 0 {
                if hundreds > 1 {
                    words.push(String::from(UNITS[hundreds - 1]));
                }
                words.push(String::from("honderd"));
            }

            if tens != 0 || units != 0 {
                if i == 0 && !first_elem {
                    if let Some(last) = words.last() {
                        if !MEGAS.contains(&last.as_str()) {
                            if last.ends_with('e') {
                                words.push(String::from("ën"));
                            } else {
                                words.push(String::from("en"));
                            }
                        }
                    }
                } else {
                    first_elem = false;
                }

                match tens {
                    0 => {
                        // case 102 => [one hundred] two
                        words.push(String::from(UNITS[units - 1]));
                    }
                    1 => {
                        // case 112 => [one hundred] twelve
                        words.push(String::from(TEENS[units]));
                    }
                    _ => {
                        // case 142 => [one hundred] forty-two
                        let ten: String = String::from(TENS[tens - 1]);
                        words.push(match units {
                            0 => ten,
                            _ => {
                                if UNITS[units - 1].ends_with('e') {
                                    format!("{}ën{}", UNITS[units - 1], ten)
                                } else {
                                    format!("{}en{}", UNITS[units - 1], ten)
                                }
                            }
                        });
                    }
                }
            }

            if i != 0 && triplet != &0 {
                if i > MEGAS.len() {
                    return Err(Num2Err::CannotConvert);
                }
                words.push(String::from(MEGAS[i - 1]));
            }
        }

        space_words(&mut words);

        Ok(words.join(""))
    }


    fn float_to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        let integral_part = num.int();
        let mut words: Vec<String> = vec![];


        let integral_word = self.int_to_cardinal(integral_part)?;
        words.push(integral_word);


        let mut ordinal_part = num.frac();
        if !ordinal_part.is_zero() {
            words.push(String::from("komma"));
        }
        while !ordinal_part.is_zero() {
            let digit = (ordinal_part * BigFloat::from(10)).int();
            ordinal_part = (ordinal_part * BigFloat::from(10)).frac();
            words.push(" ".to_string());
            words.push(match digit.to_u64().unwrap() {
                0 => String::from("nul"),
                i => String::from(UNITS[i as usize - 1]),
            });
        }

        space_words(&mut words);

        Ok(words.join(""))
    }
}

impl Language for Dutch {
    fn to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num.is_inf_pos() {
            Ok(String::from("oneindig"))
        } else if num.is_inf_neg() {
            Ok(String::from("negatief oneindig"))
        } else if num.frac().is_zero() {
            self.int_to_cardinal(num)
        } else {
            self.float_to_cardinal(num)
        }
    }

    fn to_ordinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        let cardinal_word = self.to_cardinal(num)?;

        let mut words: Vec<String> = vec![];
        let mut split = cardinal_word.split_whitespace().peekable();

        while let Some(w) = split.next() {
            if split.peek().is_some() {
                // not last word, no modification needed
                words.push(String::from(w));
            } else {
                // last word, needs to be processed
                let mut prefix = String::from("");
                let mut suffix = String::from(w);

                if w.contains('-') {
                    // e.g. forty-two => forty-second
                    let mut w_split = w.split('-');

                    if let Some(pre) = w_split.next() {
                        prefix = format!("{}-", pre);
                    }

                    if let Some(suf) = w_split.next() {
                        suffix = String::from(suf);
                    }
                }

                suffix = match suffix.as_str() {
                    "one" => String::from("eerste"),
                    "two" => String::from("tweede"),
                    "three" => String::from("derde"),
                    "four" => String::from("vierde"),
                    "five" => String::from("vijfde"),
                    "six" => String::from("zesde"),
                    "seven" => String::from("zevende"),
                    "eight" => String::from("achtste"),
                    "nine" => String::from("negende"),
                    "ten" => String::from("tiende"),
                    "eleven" => String::from("elfde"),
                    "twelve" => String::from("twaalfde"),
                    _ => {
                        if suffix.ends_with("tig") {
                            format!("{}ste", suffix)
                        } else {
                            format!("{}de", suffix)
                        }
                    }
                };

                words.push(format!("{}{}", prefix, suffix))
            }
        }

        Ok(words.join(" "))
    }

    fn to_ordinal_num(&self, num: BigFloat) -> Result<String, Num2Err> {
        let tail = (num % BigFloat::from(100)).to_u64().unwrap();
        let last = tail % 10;
        Ok(format!(
            "{}{}",
            num.to_u128().unwrap(),
            match (tail / 10 != 1, last) {
                (true, 1) => "e",
                (true, 2) => "e",
                (true, 3) => "e",
                _ => "e",
            }
        ))
    }

    fn to_year(&self, num: BigFloat) -> Result<String, Num2Err> {
        if !num.frac().is_zero() {
            return Err(Num2Err::FloatingYear);
        }

        let mut num = num;

        let mut suffix = "";
        if num.is_negative() {
            num = num.inv_sign();
            suffix = " voor christus";
        }

        let bf_100 = BigFloat::from(100);

        let (high, low) = (
            (num / bf_100).to_i64().unwrap(),
            (num % bf_100).to_i64().unwrap(),
        );
        let year_word = if high == 0 || (high % 10 == 0 && low < 10) || high >= 100 {
            // if year is 00XX, X00X, or beyond 9999, go cardinal
            self.int_to_cardinal(num)?
        } else {
            let high_word = self.int_to_cardinal(BigFloat::from(high))?;
            let low_word = if low == 0 {
                String::from("honderd")
            } else {
                self.int_to_cardinal(BigFloat::from(low))?
            };

            format!("{}{}", high_word, low_word)
        };

        Ok(format!("{}{}", year_word, suffix))
    }

    fn to_currency(&self, num: BigFloat, currency: Currency) -> Result<String, Num2Err> {
        if num.is_inf() {
            Ok(format!(
                "{}oneindig {}",
                if num.is_negative() { "minus " } else { "" },
                self.currencies(currency)
            ))
        } else if num.frac().is_zero() {
            let words = self.int_to_cardinal(num)?;
            Ok(format!(
                "{} {}",
                words,
                self.currencies(currency)
            ))
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
                    "{} en {} {}",
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
            Num2Words::new(0).lang(Lang::Dutch).cardinal().to_words(),
            Ok(String::from("nul"))
        );
        assert_eq!(
            Num2Words::new(-10)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from("minus tien"))
        );
        assert_eq!(
            Num2Words::new(38123147081932i64)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from(
                "achtendertig biljoen honderddrieëntwintig miljard honderdzevenenveertig miljoen éénentachtigduizend negenhonderdentweeëndertig"
            ))
        );
        assert_eq!(
            Num2Words::new(100000000000i64)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from("honderd miljard"))
        );
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(
            Num2Words::new(10).lang(Lang::Dutch).ordinal().to_words(),
            Ok(String::from("tiende"))
        );
        assert_eq!(
            Num2Words::new(21).lang(Lang::Dutch).ordinal().to_words(),
            Ok(String::from("éénentwintigste"))
        );
        assert_eq!(
            Num2Words::new(102).lang(Lang::Dutch).ordinal().to_words(),
            Ok(String::from("honderdtweede"))
        );
        assert_eq!(
            Num2Words::new(73).lang(Lang::Dutch).ordinal().to_words(),
            Ok(String::from("drieënzeventigste"))
        );
        assert_eq!(
            Num2Words::new(-1).lang(Lang::Dutch).ordinal().to_words(),
            Err(num2words::Num2Err::NegativeOrdinal)
        );
        assert_eq!(
            Num2Words::new(1.2).lang(Lang::Dutch).ordinal().to_words(),
            Err(num2words::Num2Err::FloatingOrdinal)
        );
    }

    #[test]
    fn test_ordinal_num() {
        assert_eq!(
            Num2Words::new(10)
                .lang(Lang::Dutch)
                .ordinal_num()
                .to_words(),
            Ok(String::from("10e"))
        );
        assert_eq!(
            Num2Words::new(13)
                .lang(Lang::Dutch)
                .ordinal_num()
                .to_words(),
            Ok(String::from("13e"))
        );
        assert_eq!(
            Num2Words::new(21)
                .lang(Lang::Dutch)
                .ordinal_num()
                .to_words(),
            Ok(String::from("21e"))
        );
        assert_eq!(
            Num2Words::new(102)
                .lang(Lang::Dutch)
                .ordinal_num()
                .to_words(),
            Ok(String::from("102e"))
        );
        assert_eq!(
            Num2Words::new(73)
                .lang(Lang::Dutch)
                .ordinal_num()
                .to_words(),
            Ok(String::from("73e"))
        );
        assert_eq!(
            Num2Words::new(-42)
                .lang(Lang::Dutch)
                .ordinal_num()
                .to_words(),
            Err(num2words::Num2Err::NegativeOrdinal)
        );
        assert_eq!(
            Num2Words::new(7.3)
                .lang(Lang::Dutch)
                .ordinal_num()
                .to_words(),
            Err(num2words::Num2Err::FloatingOrdinal)
        );
    }

    #[test]
    fn test_cardinal_float() {
        assert_eq!(
            Num2Words::new(12.5)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from("twaalf komma vijf"))
        );
        assert_eq!(
            Num2Words::new(12.51)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from("twaalf komma vijf één"))
        );
        assert_eq!(
            Num2Words::new(12.53)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from("twaalf komma vijf drie"))
        );
        assert_eq!(
            Num2Words::new(12.59)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from("twaalf komma vijf negen"))
        );
    }

    #[test]
    fn test_currency() {
        assert_eq!(
            Num2Words::new(1.01)
                .lang(Lang::Dutch)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("één dollar en één cent"))
        );
        assert_eq!(
            Num2Words::new(4000)
                .lang(Lang::Dutch)
                .currency(Currency::USD)
                .to_words(),
            Ok(String::from("vierduizend US dollar"))
        );
        assert_eq!(
            Num2Words::new(1.)
                .lang(Lang::Dutch)
                .currency(Currency::EUR)
                .to_words(),
            Ok(String::from("één euro"))
        );
        assert_eq!(
            Num2Words::new(0.20)
                .lang(Lang::Dutch)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("twintig cent"))
        );
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Dutch)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("nul dollar"))
        );
    }

    #[test]
    fn test_year() {
        assert_eq!(
            Num2Words::new(1990).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("negentiennegentig"))
        );
        assert_eq!(
            Num2Words::new(5555).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("vijfenvijftigvijfenvijftig"))
        );
        assert_eq!(
            Num2Words::new(2022).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("twintigtweeëntwintig"))
        );
        assert_eq!(
            Num2Words::new(2001).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("tweeduizend één"))
        );
        assert_eq!(
            Num2Words::new(1901).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("negentienéén"))
        );
        assert_eq!(
            Num2Words::new(1910).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("negentientien"))
        );
        assert_eq!(
            Num2Words::new(5500).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("vijfenvijftighonderd"))
        );
        assert_eq!(
            Num2Words::new(500).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("vijfhonderd"))
        );
        assert_eq!(
            Num2Words::new(50).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("vijftig"))
        );
        assert_eq!(
            Num2Words::new(0).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("nul"))
        );
        assert_eq!(
            Num2Words::new(-44).lang(Lang::Dutch).year().to_words(),
            Ok(String::from("vierenveertig voor christus"))
        );
        assert_eq!(
            Num2Words::new(1.1).lang(Lang::Dutch).year().to_words(),
            Err(num2words::Num2Err::FloatingYear)
        );
    }

    #[test]
    fn test_prefer() {
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Dutch)
                .prefer("oh")
                .to_words(),
            Ok(String::from("nul"))
        );
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Dutch)
                .prefer("nil")
                .to_words(),
            Ok(String::from("nul"))
        );
        assert_eq!(
            Num2Words::new(0.005)
                .lang(Lang::Dutch)
                .prefer("oh")
                .to_words(),
            Ok(String::from("nul komma nul nul vijf"))
        );
        assert_eq!(
            Num2Words::new(2.05)
                .lang(Lang::Dutch)
                .prefer("nil")
                .to_words(),
            Ok(String::from("twee komma nul vijf"))
        );
    }

    #[test]
    fn test_big_num() {
        use crate::lang::nl::MEGAS;
        use num_bigfloat::BigFloat;

        let mut num = BigFloat::from(1);
        for m in MEGAS {
            num *= BigFloat::from(1000);
            if m.eq("duizend") {
                assert_eq!(
                    Num2Words::new(num)
                        .lang(Lang::Dutch)
                        .cardinal()
                        .to_words(),
                    Ok(format!("één{}", m))
                );
            } else {
                assert_eq!(
                    Num2Words::new(num)
                        .lang(Lang::Dutch)
                        .cardinal()
                        .to_words(),
                    Ok(format!("één {}", m))
                );
            }
        }

        assert_eq!(
            Num2Words::parse("2.8e64")
                .unwrap()
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from("achtentwintig deciljard"))
        );

        assert_eq!(
            Num2Words::new(1e100)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Err(num2words::Num2Err::CannotConvert)
        );
    }

    #[test]
    fn test_infinity() {
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from("oneindig"))
        );
        assert_eq!(
            Num2Words::new(f64::NEG_INFINITY)
                .lang(Lang::Dutch)
                .cardinal()
                .to_words(),
            Ok(String::from("negatief oneindig"))
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Dutch)
                .ordinal()
                .to_words(),
            Err(num2words::Num2Err::InfiniteOrdinal)
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Dutch)
                .ordinal_num()
                .to_words(),
            Err(num2words::Num2Err::InfiniteOrdinal)
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Dutch)
                .year()
                .to_words(),
            Err(num2words::Num2Err::InfiniteYear)
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Dutch)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("oneindig dollar"))
        );
    }
}
