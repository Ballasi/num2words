use crate::{num2words::Num2Err, Currency, Language, Number};

pub struct English {}

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

const MEGAS: [&'static str; 15] = [
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
];

impl English {
    fn currencies(&self, currency: Currency) -> String {
        match currency {
            Currency::AUD => String::from("australian dollar"),
            Currency::CAD => String::from("canadian dollar"),
            Currency::DOLLAR => String::from("dollar"),
            Currency::EUR => String::from("euro"),
            Currency::GBP => String::from("pound"),
            Currency::USD => String::from("US dollar"),
        }
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
            return Ok(String::from("zero"));
        }

        // handling negative values
        let mut words = vec![];
        if num < 0 {
            words.push(String::from("minus"));
            num = -num;
        }

        // iterate over thousands
        for (i, triplet) in self.split_thousands(num).iter().enumerate().rev() {
            let hundreds = (triplet / 100 % 10) as usize;
            let tens = (triplet / 10 % 10) as usize;
            let units = (triplet % 10) as usize;

            if hundreds > 0 {
                words.push(String::from(UNITS[hundreds - 1]));
                words.push(String::from("hundred"));
            }

            if tens != 0 || units != 0 {
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
        let integral_part = num.floor();
        let integral_word = self.int_to_cardinal(integral_part as i64)?;

        let mut words: Vec<String> = vec![];
        words.push(integral_word);
        let as_string = num.to_string();
        let mut split = as_string.split('.');
        split.next();
        match split.next() {
            Some(s) => {
                words.push(String::from("point"));
                for c in s.chars() {
                    match String::from(c).parse::<usize>() {
                        Ok(0) => words.push(String::from("zero")),
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
        Ok(String::new())
    }

    fn to_currency(self, num: Number, currency: Currency) -> Result<String, Num2Err> {
        match num {
            Number::Int(num) => {
                let words = self.int_to_cardinal(num as i64)?;
                let plural_form = String::from(if num == 1 { "" } else { "s" });
                Ok(format!(
                    "{} {}{}",
                    words,
                    self.currencies(currency),
                    plural_form
                ))
            }
            Number::Float(num) => {
                let integral_part = num.floor() as i64;
                let cents = (num * 100.).round() as i64 % 100;
                let cents_word = self.int_to_cardinal(cents)?;
                let integral_word = self.to_currency(Number::Int(integral_part), currency)?;
                let plural_form = String::from(if cents == 1 { "" } else { "s" });

                if cents == 0 {
                    Ok(integral_word)
                } else if integral_part == 0 {
                    Ok(format!("{} {}{}", cents_word, "cent", plural_form))
                } else {
                    Ok(format!(
                        "{} and {} {}{}",
                        integral_word, cents_word, "cent", plural_form
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::num2words;

    #[test]
    fn test_cardinal() {
        assert_eq!(
            num2words!(0, lang = "en", to = "cardinal"),
            Ok(String::from("zero"))
        );
        assert_eq!(
            num2words!(-10, lang = "en", to = "cardinal"),
            Ok(String::from("minus ten"))
        );
        assert_eq!(
            num2words!(38123147081932, lang = "en", to = "cardinal"),
            Ok(String::from(
                "thirty-eight trillion one hundred twenty-three \
                 billion one hundred forty-seven million eighty-one thousand \
                 nine hundred thirty-two"
            ))
        );
        assert_eq!(
            num2words!(100000000000, lang = "en", to = "cardinal"),
            Ok(String::from("one hundred billion"))
        );
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(
            num2words!(10, lang = "en", to = "ordinal"),
            Ok(String::from("tenth"))
        );
        assert_eq!(
            num2words!(21, lang = "en", to = "ordinal"),
            Ok(String::from("twenty-first"))
        );
        assert_eq!(
            num2words!(102, lang = "en", to = "ordinal"),
            Ok(String::from("one hundred second"))
        );
        assert_eq!(
            num2words!(73, lang = "en", to = "ordinal"),
            Ok(String::from("seventy-third"))
        );
        assert_eq!(
            num2words!(-1, lang = "en", to = "ordinal"),
            Err(num2words::Num2Err::NegativeOrdinal)
        );
        assert_eq!(
            num2words!(1.2, lang = "en", to = "ordinal"),
            Err(num2words::Num2Err::FloatingOrdinal)
        );
    }

    #[test]
    fn test_ordinal_num() {
        assert_eq!(
            num2words!(10, lang = "en", to = "ordinal_num"),
            Ok(String::from("10th"))
        );
        assert_eq!(
            num2words!(21, lang = "en", to = "ordinal_num"),
            Ok(String::from("21st"))
        );
        assert_eq!(
            num2words!(102, lang = "en", to = "ordinal_num"),
            Ok(String::from("102nd"))
        );
        assert_eq!(
            num2words!(73, lang = "en", to = "ordinal_num"),
            Ok(String::from("73rd"))
        );
        assert_eq!(
            num2words!(-42, lang = "en", to = "ordinal_num"),
            Err(num2words::Num2Err::NegativeOrdinal)
        );
        assert_eq!(
            num2words!(7.3, lang = "en", to = "ordinal_num"),
            Err(num2words::Num2Err::FloatingOrdinal)
        );
    }

    #[test]
    fn test_cardinal_float() {
        assert_eq!(
            num2words!(12.5, lang = "en", to = "cardinal"),
            Ok(String::from("twelve point five"))
        );
        assert_eq!(
            num2words!(12.51, lang = "en", to = "cardinal"),
            Ok(String::from("twelve point five one"))
        );
        assert_eq!(
            num2words!(12.53, lang = "en", to = "cardinal"),
            Ok(String::from("twelve point five three"))
        );
        assert_eq!(
            num2words!(12.59, lang = "en", to = "cardinal"),
            Ok(String::from("twelve point five nine"))
        );
    }

    #[test]
    fn test_currency() {
        assert_eq!(
            num2words!(1.01, lang = "en", to = "DOLLAR"),
            Ok(String::from("one dollar and one cent"))
        );
        assert_eq!(
            num2words!(4000, lang = "en", to = "USD"),
            Ok(String::from("four thousand US dollars"))
        );
        assert_eq!(
            num2words!(1., lang = "en", to = "EUR"),
            Ok(String::from("one euro"))
        );
        assert_eq!(
            num2words!(0.20, lang = "en", to = "DOLLAR"),
            Ok(String::from("twenty cents"))
        );
        assert_eq!(
            num2words!(0, lang = "en", to = "DOLLAR"),
            Ok(String::from("zero dollars"))
        );
    }

    #[test]
    fn test_year() {
        assert_eq!(
            num2words!(1990, lang = "en", to = "year"),
            Ok(String::from("nineteen ninety"))
        );
        assert_eq!(
            num2words!(5555, lang = "en", to = "year"),
            Ok(String::from("fifty-five fifty-five"))
        );
        assert_eq!(
            num2words!(2022, lang = "en", to = "year"),
            Ok(String::from("twenty twenty-two"))
        );
        assert_eq!(
            num2words!(2001, lang = "en", to = "year"),
            Ok(String::from("two thousand and one"))
        );
        assert_eq!(
            num2words!(1901, lang = "en", to = "year"),
            Ok(String::from("nineteen oh-one"))
        );
        assert_eq!(
            num2words!(5500, lang = "en", to = "year"),
            Ok(String::from("fifty-five hundred"))
        );
        assert_eq!(
            num2words!(500, lang = "en", to = "year"),
            Ok(String::from("five hundred"))
        );
        assert_eq!(
            num2words!(50, lang = "en", to = "year"),
            Ok(String::from("fifty"))
        );
        assert_eq!(
            num2words!(0, lang = "en", to = "year"),
            Ok(String::from("zero"))
        );
        assert_eq!(
            num2words!(-44, lang = "en", to = "year"),
            Ok(String::from("forty-four BC"))
        );
    }
}
