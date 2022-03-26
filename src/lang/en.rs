use crate::{Currency, Language, Number};

pub struct English {}

impl Language for English {
    fn to_cardinal(self, num: Number) -> Option<String> {
        Some(String::new())
    }
    fn to_ordinal(self, num: Number) -> Option<String> {
        Some(String::new())
    }
    fn to_ordinal_num(self, num: Number) -> Option<String> {
        Some(String::new())
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
