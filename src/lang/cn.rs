use crate::{num2words::Num2Err, Currency, Language};
use num_bigfloat::BigFloat;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

pub enum DigitsError {
    DigitExceedLimit(u8, u8),
    NoConversionCharToNumRule(char),
    NoConversionNumToCharRule(u8),
}
impl Debug for DigitsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DigitsError::DigitExceedLimit(d, max) => {
                write!(f, "digits exceeded limit ({d} out of {max})")
            }
            DigitsError::NoConversionCharToNumRule(c) => {
                write!(f, "no conversion rule for char '{c}' to num")
            }
            DigitsError::NoConversionNumToCharRule(n) => {
                write!(f, "no conversion rule for num [{n}] to char")
            }
        }
    }
}
pub const fn arabic_num_to_char(digit: u8) -> Option<char> {
    if digit <= 9 {
        Some((b'0' + digit) as char)
    } else if digit <= 35 {
        Some((b'a' + digit - 10) as char)
    } else if digit <= 61 {
        Some((b'A' + digit - 36) as char)
    } else {
        None
    }
}
pub const fn char_to_arabic_num(digit: char) -> Option<u8> {
    if digit >= '0' && digit <= '9' {
        Some((digit as u8) - b'0')
    } else if digit >= 'a' && digit <= 'z' {
        Some((digit as u8) - b'a' + 10)
    } else if digit >= 'A' && digit <= 'Z' {
        Some((digit as u8) - b'A' + 36)
    } else {
        None
    }
}
pub struct Digits {
    digits: Vec<u8>,
    max_digit: u8,
}
impl Digits {
    pub fn new(max_digit: u8) -> Digits {
        Digits {
            digits: Vec::new(),
            max_digit,
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.digits.len()
    }
    #[inline]
    pub fn append(&mut self, digit: u8) -> Result<(), DigitsError> {
        if digit >= self.max_digit {
            Err(DigitsError::DigitExceedLimit(digit, self.max_digit))
        } else {
            Ok(self.digits.push(digit))
        }
    }
    pub fn from_string(
        digits_string: &str,
        max_digit: u8,
        convert_rule: fn(char) -> Option<u8>,
    ) -> Result<Digits, DigitsError> {
        let mut digit = Digits::new(max_digit);
        let mut digits_chars = digits_string.chars().collect::<Vec<char>>();
        digits_chars.reverse();
        for digit_char in digits_chars {
            digit.append(match convert_rule(digit_char) {
                Some(n) => n,
                None => return Err(DigitsError::NoConversionCharToNumRule(digit_char)),
            })?;
        }
        Ok(digit)
    }
    pub fn to_string(&self, convert_rule: fn(u8) -> Option<char>) -> Result<String, DigitsError> {
        let mut result = String::new();
        let digits_nums = self.digits.iter().rev();
        for digit in digits_nums {
            result.push(match convert_rule(*digit) {
                Some(c) => c,
                None => return Err(DigitsError::NoConversionNumToCharRule(*digit)),
            });
        }
        Ok(result)
    }
    pub fn to_string_complex(&self, convert_rule: fn(u8) -> Option<String>) -> Option<String> {
        let mut result = String::new();
        let digits_nums = self.digits.iter().rev();
        for digit in digits_nums {
            match convert_rule(*digit) {
                Some(c) => result.push_str(c.as_str()),
                None => return None,
            }
        }
        Some(result)
    }

    pub fn cast_to_string(&self) -> String {
        fn convert_rule(value: u8) -> Option<String> {
            match arabic_num_to_char(value) {
                Some(v) => Some(v.to_string()),
                None => Some(format!("[{}]", value.to_string())),
            }
        }
        match self.to_string_complex(convert_rule) {
            Some(v) => v,
            None => {
                panic!("Impossible")
            }
        }
    }
    pub fn from_u64(mut value: u64, max_digit: u8) -> Digits {
        let mut digits = Digits::new(max_digit);
        while value > 0 {
            digits.append((value % max_digit as u64) as u8);
            value /= max_digit as u64;
        }
        digits
    }
    #[inline]
    pub fn get_u8_array(&self) -> &Vec<u8> {
        &self.digits
    }
    pub fn from_big_float_int(mut num: BigFloat, base: u8) -> Result<Digits, DigitsError> {
        if num.is_zero() {
            return Ok(Digits::from_u64(0, base));
        }
        let mut digits = Digits::new(base);
        let base_bf = BigFloat::from(base);
        while num > BigFloat::from(0) {
            let rem = (num % base_bf).floor();
            digits.append(rem.to_u64().unwrap() as u8)?;
            num = (num / &base_bf).floor();
        }
        Ok(digits)
    }

    pub fn from_big_float_frac(
        mut num: BigFloat,
        base: u8,
        max_len: usize,
    ) -> Result<Digits, DigitsError> {
        if num.is_zero() {
            return Ok(Digits::from_u64(0, base));
        }
        let base_bf = BigFloat::from(base);
        let mut digits = Digits::new(base);
        for _ in 0..max_len {
            num -= num.floor();
            num *= &base_bf;
            let digit = num.floor();
            digits.append(digit.to_u64().unwrap() as u8)?;
            num -= digit;
            if num < BigFloat::from(1e-15) {
                break;
            }
        }
        Ok(digits)
    }
}
impl Index<usize> for Digits {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.digits[index]
    }
}
impl IndexMut<usize> for Digits {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.digits[index]
    }
}
#[cfg(test)]
mod tests1 {
    use super::*;
    #[test]
    fn test_134() {
        let num = "134";
        let digits = Digits::from_string(num, 10, char_to_arabic_num).unwrap();
        assert_eq!(digits.to_string(arabic_num_to_char).unwrap(), num)
    }
    #[test]
    fn test_2978() {
        let num = 2978;
        let digits = Digits::from_u64(num, 10);
        assert_eq!(
            digits.to_string(arabic_num_to_char).unwrap(),
            num.to_string()
        )
    }
    #[test]
    fn test_114514() {
        let num = 114514;
        let digits = Digits::from_u64(num, 10);
        assert_eq!(digits.to_string(arabic_num_to_char).unwrap(), "114514");
    }
}

pub struct Chinese {
    //prefer 零 over 〇
    prefer_ling: bool,
    //prefer 一十 over 十
    prefer_one_ten: bool,
    //control whether to use traditional
    traditional: bool,
}
///units of 10^4 in simplified chinese
const MEGA_UNITS: [&str; 12] = [
    "", "万", "亿", "兆", "京", "垓", "秭", "穰", "沟", "涧", "正", "载",
];
///units of 10^4 in traditional chinese
const MEGA_UNITS_TRAD: [&str; 12] = [
    "", "萬", "億", "兆", "京", "垓", "秭", "穰", "溝", "澗", "正", "載",
];
///digits in both chinese lang
const DIGITS: [char; 9] = ['一', '二', '三', '四', '五', '六', '七', '八', '九'];

impl Chinese {
    pub fn default() -> Chinese {
        Chinese {
            prefer_ling: true,
            prefer_one_ten: false,
            traditional: false,
        }
    }
    pub fn new(prefer_ling: bool, prefer_one_ten: bool, traditional: bool) -> Chinese {
        Chinese {
            prefer_ling,
            prefer_one_ten,
            traditional,
        }
    }
    pub fn megaunit(&self, place: usize) -> Result<&'static str, &str> {
        if !self.traditional {
            if place < MEGA_UNITS.len() {
                return Ok(MEGA_UNITS[place]);
            }
        } else {
            if place < MEGA_UNITS_TRAD.len() {
                return Ok(MEGA_UNITS_TRAD[place]);
            }
        }
        Err("Too big too find a unit")
    }
    pub fn digit_to_char(&self, digit: u8) -> char {
        if digit > 0 && digit <= 9 {
            DIGITS[digit as usize - 1]
        } else {
            self.zero()
        }
    }
    pub fn zero(&self) -> char {
        if self.prefer_ling {
            '零'
        } else {
            '〇'
        }
    }
    fn int_to_text(&self, num: Digits) -> Result<String, &str> {
        if num.len() == 0 {
            return Ok(self.zero().to_string());
        }
        let last_digit = num.len() - 1;
        let mut text = String::new();
        let mut zeros: usize = 0;
        for (place, digit) in num.get_u8_array().iter().enumerate() {
            //counting zeros
            if *digit == 0 {
                //if it is the first place zero but
                //there is no zero in the smaller section we still need to add zero
                if place % 4 == 0 && place != 0 {
                    if zeros == 0 {
                        text.push(self.zero());
                    }
                }
                zeros += 1;
            } else {
                zeros = 0;
            }
            //if is first digit in the section or in the number, we should do some work
            if place % 4 == 3 || place == last_digit {
                let section_start = place - place % 4;
                //check if the unit is not used, like 1_0000_0000 do not display 万
                if zeros >= 4 {
                    //if the whole section is 0, skip it
                    continue;
                } else {
                    //enter normal process
                    text.push_str(self.megaunit(place / 4)?);
                    let mut section_zeros: usize = 0;
                    for (section_place, section_digit) in
                        num.get_u8_array()[section_start..=place].iter().enumerate()
                    {
                        if *section_digit == 0 {
                            //check is there any hanging zeros, like 1001
                            //if it is the rightmost place of 0, we should consider adding 零
                            if section_zeros == 0 {
                                //do not add 零 when it is the rightmost digit in the section
                                //but add it elsewhere
                                if section_place != 0 {
                                    text.push(self.zero());
                                }
                            }
                            //record one more zero
                            section_zeros += 1;
                        } else {
                            //reset section zeros
                            section_zeros = 0;
                            //enter normal number process
                            //add the approximate unit in section
                            match section_place {
                                0 => (),
                                1 => {
                                    text.push('十');
                                    if (*section_digit == 1)
                                        && (!self.prefer_one_ten)
                                        && (section_start + section_place == last_digit)
                                    {
                                        continue;
                                    }
                                }
                                2 => {
                                    text.push('百');
                                }
                                3 => {
                                    text.push('千');
                                }
                                _ => {
                                    debug_assert!(
                                        false,
                                        "should not have section place of: {:?}",
                                        section_place
                                    );
                                }
                            }
                            text.push(self.digit_to_char(*section_digit));
                        }
                    }
                }
            }
        }
        Ok(text.chars().rev().collect::<String>())
    }
    fn float_to_text(&self, num: BigFloat) -> Result<String, Num2Err> {
        let integer_part = num.int();
        let mut text = match self.int_to_text(match Digits::from_big_float_int(integer_part, 10) {
            Ok(v) => v,
            //impossible
            Err(_) => return Err(Num2Err::CannotConvert),
        }) {
            Ok(v) => v,
            //impossible
            Err(_) => return Err(Num2Err::CannotConvert),
        };
        if num.frac().is_zero() {
            return Ok(text);
        }
        //add decimal point
        if !self.traditional {
            text.push('点');
        } else {
            text.push('點');
        }
        let decimal_part = match Digits::from_big_float_frac(num.frac(), 10, 1000) {
            Ok(v) => v,
            //impossible
            Err(_) => return Err(Num2Err::CannotConvert),
        };
        //adding the decimal part
        for digit in decimal_part.get_u8_array() {
            text.push(self.digit_to_char(*digit));
        }
        Ok(text)
    }
}

impl Language for Chinese {
    fn to_cardinal(&self, mut num: BigFloat) -> Result<String, Num2Err> {
        if num.is_nan() {
            return Err(Num2Err::CannotConvert);
        }
        let mut text = String::new();
        if num.is_negative() {
            text.push('负');
            num = -num;
        }
        if num.is_inf() {
            text.push_str("无穷");
            return Ok(text);
        }
        text.push_str(self.float_to_text(num)?.as_str());
        Ok(text)
    }

    fn to_ordinal(&self, mut num: BigFloat) -> Result<String, Num2Err> {
        let mut result = "第".to_string();
        result.push_str(self.to_cardinal(num)?.as_str());
        Ok(result)
    }

    fn to_ordinal_num(&self, num: BigFloat) -> Result<String, Num2Err> {
        let digits = match Digits::from_big_float_int(num, 10) {
            Ok(v) => v,
            Err(_) => return Err(Num2Err::CannotConvert),
        };
        let mut result = "第".to_string();
        result.push_str(digits.cast_to_string().as_str());
        Ok(result)
    }

    fn to_year(&self, num: BigFloat) -> Result<String, Num2Err> {
        todo!()
    }

    fn to_currency(&self, num: BigFloat, currency: Currency) -> Result<String, Num2Err> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Lang, Num2Words};

    #[test]
    fn test_zero_and_single_digits() {
        let zh = Chinese::default();
        // single digit
        for i in 0..=9 {
            let n = num_bigfloat::BigFloat::from(i);
            let s = zh.to_cardinal(n).unwrap();
            let expected = match i {
                0 => "零",
                1 => "一",
                2 => "二",
                3 => "三",
                4 => "四",
                5 => "五",
                6 => "六",
                7 => "七",
                8 => "八",
                9 => "九",
                _ => unreachable!(),
            };
            assert_eq!(s, expected);
        }
    }

    #[test]
    fn test_traditional_and_simplified_difference() {
        let zh_simp = Chinese::new(true, false, false);
        let zh_trad = Chinese::new(true, false, true);
        let n = num_bigfloat::BigFloat::from(12345678u64);
        let simp = zh_simp.to_cardinal(n.clone()).unwrap();
        let trad = zh_trad.to_cardinal(n).unwrap();
        assert!(simp.contains("万"));
        assert!(trad.contains("萬"));
    }

    #[test]
    fn test_one_ten_preference() {
        let zh_short = Chinese::new(true, false, false);
        let zh_full = Chinese::new(true, true, false);
        let ten = num_bigfloat::BigFloat::from(10u8);
        let short = zh_short.to_cardinal(ten.clone()).unwrap();
        let full = zh_full.to_cardinal(ten).unwrap();
        assert_eq!(short, "十");
        assert_eq!(full, "一十");
    }

    #[test]
    fn test_section_with_all_zero_middle() {
        // 1000001 => 一百万零一
        let zh = Chinese::default();
        let n = num_bigfloat::BigFloat::from(1_000_001u64);
        let s = zh.to_cardinal(n).unwrap();
        assert_eq!(s, "一百万零一");
    }

    #[test]
    fn test_large_numbers_multiple_units() {
        let zh = Chinese::default();
        let n = num_bigfloat::BigFloat::from(1_0000_0000_0000_0000u64);
        let s = zh.to_cardinal(n).unwrap();
        // 应出现“京”
        assert!(s.contains("京"));
    }

    #[test]
    fn test_trailing_decimal_zeros() {
        let zh = Chinese::default();
        let n = num_bigfloat::BigFloat::from(3.1400);
        let s = zh.to_cardinal(n).unwrap();
        // 应该裁掉末尾零，不多出“零”
        assert_eq!(s, "三点一四");
    }

    #[test]
    fn test_long_fraction_precision_limit() {
        let zh = Chinese::default();
        // 1/3 = 0.3333…
        let n = num_bigfloat::BigFloat::from(1) / num_bigfloat::BigFloat::from(3);
        let s = zh.to_cardinal(n).unwrap();
        assert!(s.starts_with("零点三"));
    }

    #[test]
    fn test_negative_with_fraction() {
        let zh = Chinese::default();
        let n = num_bigfloat::BigFloat::from(-45.6);
        let s = zh.to_cardinal(n).unwrap();
        assert!(s.starts_with("负四十五点六"));
    }

    #[test]
    fn test_bigfloat_int_and_frac_consistency() {
        // 验证整数和小数拼接逻辑一致
        let zh = Chinese::default();
        let n1 = num_bigfloat::BigFloat::from(42);
        let n2 = num_bigfloat::BigFloat::from(42.0);
        assert_eq!(zh.to_cardinal(n1).unwrap(), zh.to_cardinal(n2).unwrap());
    }

    #[test]
    fn test_huge_number_overflow_handling() {
        let zh = Chinese::default();
        // 超出载之后
        let n = num_bigfloat::BigFloat::from(10).pow(&BigFloat::from(52)); // 超过“载”
        let result = zh.to_cardinal(n);
        assert!(result.is_err() || result.unwrap().contains("载"));
    }

    #[test]
    fn test_round_trip_digits_string() {
        let num_str = "1A3z";
        let digits = Digits::from_string(num_str, 62, char_to_arabic_num).unwrap();
        assert_eq!(digits.to_string(arabic_num_to_char).unwrap(), num_str);
    }

    #[test]
    fn test_big_float_frac_precision_bound() {
        let bf = num_bigfloat::BigFloat::from(0.999_999);
        let digits = Digits::from_big_float_frac(bf, 10, 5).unwrap();
        assert!(digits.len() <= 5);
    }
    #[test]
    fn test_cardinal_common_cases() {
        let zh = Chinese::default();

        // 基础整数
        assert_eq!(zh.to_cardinal(BigFloat::from(0)).unwrap(), "零");
        assert_eq!(zh.to_cardinal(BigFloat::from(9)).unwrap(), "九");
        assert_eq!(zh.to_cardinal(BigFloat::from(10)).unwrap(), "十");
        assert_eq!(zh.to_cardinal(BigFloat::from(11)).unwrap(), "十一");
        assert_eq!(zh.to_cardinal(BigFloat::from(20)).unwrap(), "二十");
        assert_eq!(zh.to_cardinal(BigFloat::from(21)).unwrap(), "二十一");

        // 中间零
        assert_eq!(zh.to_cardinal(BigFloat::from(105)).unwrap(), "一百零五");
        assert_eq!(zh.to_cardinal(BigFloat::from(1005)).unwrap(), "一千零五");
        assert_eq!(zh.to_cardinal(BigFloat::from(10005)).unwrap(), "一万零五");
        assert_eq!(
            zh.to_cardinal(BigFloat::from(1000500)).unwrap(),
            "一百万零五百"
        );

        // 大数字
        assert_eq!(zh.to_cardinal(BigFloat::from(1_0000_0000)).unwrap(), "一亿");
        assert_eq!(
            zh.to_cardinal(BigFloat::from(1_0000_0000_0000u64)).unwrap(),
            "一兆"
        );
    }
    #[test]
    fn test_ordinal_numbers() {
        let zh = Chinese::default();

        assert_eq!(zh.to_ordinal(BigFloat::from(1)).unwrap(), "第一");
        assert_eq!(zh.to_ordinal(BigFloat::from(2)).unwrap(), "第二");
        assert_eq!(zh.to_ordinal(BigFloat::from(10)).unwrap(), "第十");
        assert_eq!(zh.to_ordinal(BigFloat::from(105)).unwrap(), "第一百零五");
        assert_eq!(
            zh.to_ordinal(BigFloat::from(1234)).unwrap(),
            "第一千二百三十四"
        );
    }
    #[test]
    fn test_ordinal_numeric_format() {
        let zh = Chinese::default();

        assert_eq!(zh.to_ordinal_num(BigFloat::from(1)).unwrap(), "第1");
        assert_eq!(zh.to_ordinal_num(BigFloat::from(9)).unwrap(), "第9");
        assert_eq!(zh.to_ordinal_num(BigFloat::from(42)).unwrap(), "第42");
        assert_eq!(zh.to_ordinal_num(BigFloat::from(1005)).unwrap(), "第1005");
    }
    #[test]
    fn test_to_year_format() {
        let zh = Chinese::default();

        assert_eq!(zh.to_year(BigFloat::from(2024)).unwrap(), "第2024年");
        assert_eq!(zh.to_year(BigFloat::from(2000)).unwrap(), "第2000年");
        assert_eq!(zh.to_year(BigFloat::from(1)).unwrap(), "第1年");
    }
    #[test]
    fn test_edge_cases_cardinal() {
        let zh = Chinese::default();

        // 负数
        assert_eq!(zh.to_cardinal(BigFloat::from(-7)).unwrap(), "负七");

        // 小数
        assert_eq!(zh.to_cardinal(BigFloat::from(3.14)).unwrap(), "三点一四");
        assert_eq!(zh.to_cardinal(BigFloat::from(0.205)).unwrap(), "零点二零五");

        // 无限与 NaN
        let inf = BigFloat::from(f64::INFINITY);
        let nan = BigFloat::from(f64::NAN);
        assert_eq!(zh.to_cardinal(inf).unwrap(), "无穷");
        assert!(zh.to_cardinal(nan).is_err());
    }
    #[test]
    fn test_preference_modes() {
        let zh1 = Chinese::new(true, false, false); // prefer 零
        let zh2 = Chinese::new(false, false, false); // prefer 〇
        assert_eq!(zh1.zero(), '零');
        assert_eq!(zh2.zero(), '〇');

        let zh_short = Chinese::new(true, false, false);
        let zh_full = Chinese::new(true, true, false);
        let n = BigFloat::from(10);
        assert_eq!(zh_short.to_cardinal(n.clone()).unwrap(), "十");
        assert_eq!(zh_full.to_cardinal(n).unwrap(), "一十");
    }
}
