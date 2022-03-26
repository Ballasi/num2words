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
    fn split_thousands(self, mut num: i64) -> Vec<i64> {
        let mut thousands = Vec::new();

        while num > 0 {
            thousands.push(num % 1000);
            num /= 1000;
        }

        thousands
    }

    fn int_to_cardinal(self, mut num: i64) -> Result<String, Num2Err> {
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
                        let mut ten: String = String::from(TENS[tens - 1]);
                        if units > 0 {
                            ten = format!("{}-{}", ten, UNITS[units - 1]);
                        }
                        words.push(ten);
                    }
                }
            }

            if i != 0 {
                words.push(String::from(MEGAS[i - 1]));
            }
        }

        Ok(words.join(" "))
    }

    fn float_to_cardinal(self, num: f64) -> Result<String, Num2Err> {
        let integral_part = num.floor();

        match self.int_to_cardinal(integral_part as i64) {
            Ok(integral_word) => {
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
            Err(err) => Err(err),
        }
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
        Ok(String::new())
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
        Ok(String::new())
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
