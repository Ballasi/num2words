use crate::{num2words::Num2Err, Currency, Language, Number};

pub struct English {
    prefer_oh: bool,
    prefer_nil: bool,
}

const UNITS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TENS: [&'static str; 9] = [
    "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const TEENS: [&'static str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

// As defined by the AHD4, CED, RHD2, W3 and UM authorities
// For more information, see
// https://en.wikipedia.org/wiki/Names_of_large_numbers
const MEGAS: [&'static str; 21] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
    "undecillion",
    "duodecillion",
    "tredecillion",
    "quattuordecillion",
    "quindecillion",
    "sexdecillion",
    "septendecillion",
    "octodecillion",
    "novemdecillion",
    "vigintillion",
];

impl English {
    pub fn new(prefer_oh: bool, prefer_nil: bool) -> Self {
        Self {
            prefer_oh,
            prefer_nil,
        }
    }

    fn currencies(&self, currency: Currency, plural_form: bool) -> String {
        currency
            .default_string()
            .replace("{}", if plural_form { "s" } else { "" })
    }

    fn cents(&self, currency: Currency, plural_form: bool) -> String {
        currency
            .default_cent_string()
            .replace("{}", if plural_form { "s" } else { "" })
    }

    fn split_thousands(&self, mut num: i64) -> Vec<i64> {
        let mut thousands = Vec::new();

        while num > 0 {
            thousands.push(num % 1000);
            num /= 1000;
        }

        thousands
    }

    fn int_to_cardinal(&self, mut num: i64) -> Result<String, Num2Err> {
        // special case zero
        if num == 0 {
            return Ok(String::from(if self.prefer_oh {
                "oh"
            } else if self.prefer_nil {
                "nil"
            } else {
                "zero"
            }));
        }

        // handling negative values
        let mut words = vec![];
        if num < 0 {
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
                words.push(String::from(UNITS[hundreds - 1]));
                words.push(String::from("hundred"));
            }

            if tens != 0 || units != 0 {
                if i == 0 && !first_elem {
                    words.push(String::from("and"));
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
                            _ => format!("{}-{}", ten, UNITS[units - 1]),
                        });
                    }
                }
            }

            if i != 0 && triplet != &0 {
                words.push(String::from(MEGAS[i - 1]));
            }
        }

        Ok(words.join(" "))
    }

    fn float_to_cardinal(&self, num: f64) -> Result<String, Num2Err> {
        let integral_part = num.floor() as i64;
        let mut words: Vec<String> = vec![];

        if integral_part != 0 {
            let integral_word = self.int_to_cardinal(integral_part)?;
            words.push(integral_word);
        }

        let as_string = num.to_string();
        let mut split = as_string.split('.');
        split.next();
        match split.next() {
            Some(s) => {
                words.push(String::from("point"));
                for c in s.chars() {
                    match String::from(c).parse::<usize>() {
                        Ok(0) => {
                            words.push(String::from(if self.prefer_oh { "oh" } else { "zero" }))
                        }
                        Ok(i) => words.push(String::from(UNITS[i - 1])),
                        _ => {}
                    }
                }
            }
            None => {}
        }
        Ok(words.join(" "))
    }
}

impl Language for English {
    fn to_cardinal(self, num: Number) -> Result<String, Num2Err> {
        match num {
            Number::Int(i) => self.int_to_cardinal(i),
            Number::Float(i) => self.float_to_cardinal(i),
        }
    }

    fn to_ordinal(self, num: Number) -> Result<String, Num2Err> {
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
                    "one" => String::from("first"),
                    "two" => String::from("second"),
                    "three" => String::from("third"),
                    "four" => String::from("fourth"),
                    "five" => String::from("fifth"),
                    "six" => String::from("sixth"),
                    "seven" => String::from("seventh"),
                    "eight" => String::from("eighth"),
                    "nine" => String::from("ninth"),
                    "ten" => String::from("tenth"),
                    "eleven" => String::from("eleventh"),
                    "twelve" => String::from("twelfth"),
                    _ => {
                        if suffix.ends_with('y') {
                            format!("{}ieth", &suffix[..suffix.len() - 1])
                        } else {
                            format!("{}th", suffix)
                        }
                    }
                };

                words.push(format!("{}{}", prefix, suffix))
            }
        }

        Ok(words.join(" "))
    }

    fn to_ordinal_num(self, num: Number) -> Result<String, Num2Err> {
        Ok(format!(
            "{}{}",
            num,
            match num.as_i64() % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
        ))
    }

    fn to_year(self, num: Number) -> Result<String, Num2Err> {
        match num {
            Number::Int(mut year) => {
                let mut suffix = "";
                if year < 0 {
                    year = -year;
                    suffix = " BC";
                }

                let (high, low) = (year / 100, year % 100);
                let year_word = if high == 0 || (high % 10 == 0 && low < 10) || high >= 100 {
                    // if year is 00XX, X00X, or beyond 9999, go cardinal
                    self.int_to_cardinal(year)?
                } else {
                    let high_word = self.int_to_cardinal(high)?;
                    let low_word = if low == 0 {
                        String::from("hundred")
                    } else if low < 10 {
                        format!("oh-{}", self.int_to_cardinal(low)?)
                    } else {
                        self.int_to_cardinal(low)?
                    };

                    format!("{} {}", high_word, low_word)
                };

                Ok(format!("{}{}", year_word, suffix))
            }
            Number::Float(_) => Err(Num2Err::FloatingYear),
        }
    }

    fn to_currency(self, num: Number, currency: Currency) -> Result<String, Num2Err> {
        match num {
            Number::Int(num) => {
                let words = self.int_to_cardinal(num as i64)?;
                Ok(format!("{} {}", words, self.currencies(currency, num != 1)))
            }
            Number::Float(num) => {
                let integral_part = num.floor() as i64;
                let cents_nb = (num * 100.).round() as i64 % 100;
                let cents_words = self.int_to_cardinal(cents_nb)?;
                let cents_suffix = self.cents(currency, cents_nb != 1);
                let integral_word = self.to_currency(Number::Int(integral_part), currency)?;

                if cents_nb == 0 {
                    Ok(integral_word)
                } else if integral_part == 0 {
                    Ok(format!("{} {}", cents_words, cents_suffix))
                } else {
                    Ok(format!(
                        "{} and {} {}",
                        integral_word, cents_words, cents_suffix
                    ))
                }
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
            Num2Words::new(0).lang(Lang::English).cardinal().to_words(),
            Ok(String::from("zero"))
        );
        assert_eq!(
            Num2Words::new(-10)
                .lang(Lang::English)
                .cardinal()
                .to_words(),
            Ok(String::from("minus ten"))
        );
        assert_eq!(
            Num2Words::new(38123147081932i64)
                .lang(Lang::English)
                .cardinal()
                .to_words(),
            Ok(String::from(
                "thirty-eight trillion one hundred twenty-three \
                 billion one hundred forty-seven million eighty-one thousand \
                 nine hundred and thirty-two"
            ))
        );
        assert_eq!(
            Num2Words::new(100000000000i64)
                .lang(Lang::English)
                .cardinal()
                .to_words(),
            Ok(String::from("one hundred billion"))
        );
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(
            Num2Words::new(10).lang(Lang::English).ordinal().to_words(),
            Ok(String::from("tenth"))
        );
        assert_eq!(
            Num2Words::new(21).lang(Lang::English).ordinal().to_words(),
            Ok(String::from("twenty-first"))
        );
        assert_eq!(
            Num2Words::new(102).lang(Lang::English).ordinal().to_words(),
            Ok(String::from("one hundred second"))
        );
        assert_eq!(
            Num2Words::new(73).lang(Lang::English).ordinal().to_words(),
            Ok(String::from("seventy-third"))
        );
        assert_eq!(
            Num2Words::new(-1).lang(Lang::English).ordinal().to_words(),
            Err(num2words::Num2Err::NegativeOrdinal)
        );
        assert_eq!(
            Num2Words::new(1.2).lang(Lang::English).ordinal().to_words(),
            Err(num2words::Num2Err::FloatingOrdinal)
        );
    }

    #[test]
    fn test_ordinal_num() {
        assert_eq!(
            Num2Words::new(10)
                .lang(Lang::English)
                .ordinal_num()
                .to_words(),
            Ok(String::from("10th"))
        );
        assert_eq!(
            Num2Words::new(21)
                .lang(Lang::English)
                .ordinal_num()
                .to_words(),
            Ok(String::from("21st"))
        );
        assert_eq!(
            Num2Words::new(102)
                .lang(Lang::English)
                .ordinal_num()
                .to_words(),
            Ok(String::from("102nd"))
        );
        assert_eq!(
            Num2Words::new(73)
                .lang(Lang::English)
                .ordinal_num()
                .to_words(),
            Ok(String::from("73rd"))
        );
        assert_eq!(
            Num2Words::new(-42)
                .lang(Lang::English)
                .ordinal_num()
                .to_words(),
            Err(num2words::Num2Err::NegativeOrdinal)
        );
        assert_eq!(
            Num2Words::new(7.3)
                .lang(Lang::English)
                .ordinal_num()
                .to_words(),
            Err(num2words::Num2Err::FloatingOrdinal)
        );
    }

    #[test]
    fn test_cardinal_float() {
        assert_eq!(
            Num2Words::new(12.5)
                .lang(Lang::English)
                .cardinal()
                .to_words(),
            Ok(String::from("twelve point five"))
        );
        assert_eq!(
            Num2Words::new(12.51)
                .lang(Lang::English)
                .cardinal()
                .to_words(),
            Ok(String::from("twelve point five one"))
        );
        assert_eq!(
            Num2Words::new(12.53)
                .lang(Lang::English)
                .cardinal()
                .to_words(),
            Ok(String::from("twelve point five three"))
        );
        assert_eq!(
            Num2Words::new(12.59)
                .lang(Lang::English)
                .cardinal()
                .to_words(),
            Ok(String::from("twelve point five nine"))
        );
    }

    #[test]
    fn test_currency() {
        assert_eq!(
            Num2Words::new(1.01)
                .lang(Lang::English)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("one dollar and one cent"))
        );
        assert_eq!(
            Num2Words::new(4000)
                .lang(Lang::English)
                .currency(Currency::USD)
                .to_words(),
            Ok(String::from("four thousand US dollars"))
        );
        assert_eq!(
            Num2Words::new(1.)
                .lang(Lang::English)
                .currency(Currency::EUR)
                .to_words(),
            Ok(String::from("one euro"))
        );
        assert_eq!(
            Num2Words::new(0.20)
                .lang(Lang::English)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("twenty cents"))
        );
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::English)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("zero dollars"))
        );
    }

    #[test]
    fn test_year() {
        assert_eq!(
            Num2Words::new(1990).lang(Lang::English).year().to_words(),
            Ok(String::from("nineteen ninety"))
        );
        assert_eq!(
            Num2Words::new(5555).lang(Lang::English).year().to_words(),
            Ok(String::from("fifty-five fifty-five"))
        );
        assert_eq!(
            Num2Words::new(2022).lang(Lang::English).year().to_words(),
            Ok(String::from("twenty twenty-two"))
        );
        assert_eq!(
            Num2Words::new(2001).lang(Lang::English).year().to_words(),
            Ok(String::from("two thousand and one"))
        );
        assert_eq!(
            Num2Words::new(1901).lang(Lang::English).year().to_words(),
            Ok(String::from("nineteen oh-one"))
        );
        assert_eq!(
            Num2Words::new(5500).lang(Lang::English).year().to_words(),
            Ok(String::from("fifty-five hundred"))
        );
        assert_eq!(
            Num2Words::new(500).lang(Lang::English).year().to_words(),
            Ok(String::from("five hundred"))
        );
        assert_eq!(
            Num2Words::new(50).lang(Lang::English).year().to_words(),
            Ok(String::from("fifty"))
        );
        assert_eq!(
            Num2Words::new(0).lang(Lang::English).year().to_words(),
            Ok(String::from("zero"))
        );
        assert_eq!(
            Num2Words::new(-44).lang(Lang::English).year().to_words(),
            Ok(String::from("forty-four BC"))
        );
        assert_eq!(
            Num2Words::new(1.1).lang(Lang::English).year().to_words(),
            Err(num2words::Num2Err::FloatingYear)
        );
    }

    #[test]
    fn test_prefer() {
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::English)
                .prefer("oh")
                .to_words(),
            Ok(String::from("oh"))
        );
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::English)
                .prefer("nil")
                .to_words(),
            Ok(String::from("nil"))
        );
        assert_eq!(
            Num2Words::new(0.005)
                .lang(Lang::English)
                .prefer("oh")
                .to_words(),
            Ok(String::from("point oh oh five"))
        );
        assert_eq!(
            Num2Words::new(2.05)
                .lang(Lang::English)
                .prefer("nil")
                .to_words(),
            Ok(String::from("two point zero five"))
        );
    }
}
