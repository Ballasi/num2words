use crate::{Currency, Language, Number};

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

    fn int_to_cardinal(self, mut num: i64) -> Option<String> {
        // special case zero
        if num == 0 {
            return Some(String::from("zero"));
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
                words.push(UNITS[hundreds - 1].into());
                words.push("hundred".into());
            }

            if tens != 0 || units != 0 {
                match tens {
                    0 => {
                        // case 102 => [one hundred] two
                        words.push(UNITS[units - 1].into());
                    }
                    1 => {
                        // case 112 => [one hundred] twelve
                        words.push(TEENS[units].into());
                    }
                    _ => {
                        // case 142 => [one hundred] forty-two
                        let mut ten: String = TENS[tens - 1].into();
                        if units > 0 {
                            ten = format!("{}-{}", ten, UNITS[units - 1]);
                        }
                        words.push(ten);
                    }
                }
            }

            if i != 0 {
                words.push(MEGAS[i - 1].into());
            }
        }

        Some(words.join(" "))
    }

    fn float_to_cardinal(self, num: f64) -> Option<String> {
        todo!()
    }
}

impl Language for English {
    fn to_cardinal(self, num: Number) -> Option<String> {
        match num {
            Number::Int(i) => self.int_to_cardinal(i),
            Number::Float(i) => self.float_to_cardinal(i),
        }
    }
    fn to_ordinal(self, num: Number) -> Option<String> {
        Some(String::new())
    }
    fn to_ordinal_num(self, num: Number) -> Option<String> {
        Some(format!(
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
    fn to_year(self, num: Number) -> Option<String> {
        Some(String::new())
    }
    fn to_currency(self, num: Number, currency: Currency) -> Option<String> {
        Some(String::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::num2words;

    #[test]
    fn test_cardinal() {
        assert_eq!(
            num2words!(0, lang = "en", to = "cardinal"),
            Some(String::from("zero"))
        );
        assert_eq!(
            num2words!(-10, lang = "en", to = "cardinal"),
            Some(String::from("minus ten"))
        );
        assert_eq!(
            num2words!(38123147081932, lang = "en", to = "cardinal"),
            Some(String::from(
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
            Some(String::from("10th"))
        );
        assert_eq!(
            num2words!(21, lang = "en", to = "ordinal_num"),
            Some(String::from("21st"))
        );
        assert_eq!(
            num2words!(102, lang = "en", to = "ordinal_num"),
            Some(String::from("102nd"))
        );
        assert_eq!(
            num2words!(73, lang = "en", to = "ordinal_num"),
            Some(String::from("73rd"))
        );
    }

    #[test]
    fn test_cardinal_float() {
        assert_eq!(
            num2words!(12.5, lang = "en", to = "cardinal"),
            Some(String::from("twelve point five"))
        );
        assert_eq!(
            num2words!(12.51, lang = "en", to = "cardinal"),
            Some(String::from("twelve point five one"))
        );
        assert_eq!(
            num2words!(12.53, lang = "en", to = "cardinal"),
            Some(String::from("twelve point five three"))
        );
        assert_eq!(
            num2words!(12.59, lang = "en", to = "cardinal"),
            Some(String::from("twelve point five nine"))
        );
    }

    #[test]
    fn test_currency() {
        assert_eq!(
            num2words!(1.01, lang = "en", to = "DOLLAR"),
            Some(String::from("one dollar and one cent"))
        );
        assert_eq!(
            num2words!(4000, lang = "en", to = "USD"),
            Some(String::from("four thousand US dollars"))
        );
    }

    #[test]
    fn test_year() {
        assert_eq!(
            num2words!(1990, lang = "en", to = "year"),
            Some(String::from("nineteen ninety"))
        );
        assert_eq!(
            num2words!(5555, lang = "en", to = "year"),
            Some(String::from("fifty-five fifty-five"))
        );
        assert_eq!(
            num2words!(2022, lang = "en", to = "year"),
            Some(String::from("twenty twenty-two"))
        );
        assert_eq!(
            num2words!(2001, lang = "en", to = "year"),
            Some(String::from("two thousand and one"))
        );
        assert_eq!(
            num2words!(1901, lang = "en", to = "year"),
            Some(String::from("nineteen oh-one"))
        );
        assert_eq!(
            num2words!(5500, lang = "en", to = "year"),
            Some(String::from("fifty-five hundred"))
        );
        assert_eq!(
            num2words!(500, lang = "en", to = "year"),
            Some(String::from("five hundred"))
        );
        assert_eq!(
            num2words!(50, lang = "en", to = "year"),
            Some(String::from("fifty"))
        );
        assert_eq!(
            num2words!(0, lang = "en", to = "year"),
            Some(String::from("zero"))
        );
        assert_eq!(
            num2words!(-44, lang = "en", to = "year"),
            Some(String::from("forty-four BC"))
        );
    }
}
