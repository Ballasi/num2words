use crate::{num2words::Num2Err, Currency, Language};
use num_bigfloat::BigFloat;
use std::str::FromStr;

// Джерело: Український Правопис 2019
// § 38. Складні числівники
// § 105. Відмінювання кількісних числівників
// § 106. Відмінювання порядкових числівників
// § 107. Відмінювання дробових числівників

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Declenation {
    #[default]
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Instrumental,
    Locative,
    //Vocative,
}

impl Declenation {
    fn index(&self) -> usize {
        use Declenation::*;
        match self {
            Nominative => 0,
            Genitive => 1,
            Dative => 2,
            Accusative => 3,
            Instrumental => 4,
            Locative => 5,
            //Vocative => 6,
        }
    }
}

impl FromStr for Declenation {
    type Err = ();

    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Declenation::*;

        Ok(match s.to_lowercase().as_str() {
            "н" | "називний"  | "nom" | "nominative"   => Nominative,
            "р" | "родовий"   | "gen" | "genitive"     => Genitive,
            "д" | "давальний" | "dat" | "dative"       => Dative,
            "з" | "знахідний" | "acc" | "accusative"   => Accusative,
            "о" | "орудний"   | "ins" | "instrumental" => Instrumental,
            "м" | "місцевий"  | "loc" | "locative"     => Locative,
            //"к" | "кличний"   | "voc" | "vocative"     => Vocative
            _ => return Err(()),
        })
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Gender {
    #[default]
    Masculine,
    Feminine,
    Neuter,
}

impl FromStr for Gender {
    type Err = ();
    
    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Gender::*;
        Ok(match s.to_lowercase().as_str() {
            "ч"  | "чол" | "чоловічий" | "m"  | "masculine" => Masculine,
            "ж"  | "жін" | "жіночий"   | "f"  | "feminine"  => Feminine,
            "с"  | "сер" | "середній"  | "n"  | "neuter"    => Neuter,
            _ => return Err(()),
        })
    }
}

impl Gender {
    fn index(&self) -> usize {
        use Gender::*;
        match self {
            Masculine => 0,
            Feminine => 1,
            Neuter => 2,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum GrammaticalNumber {
    #[default]
    Singular,
    Plural,
}

impl FromStr for GrammaticalNumber {
    type Err = ();
    
    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GrammaticalNumber::*;
        Ok(match s.to_lowercase().as_str() {
            "од" | "однина"  | "sing" | "singular" => Singular,
            "мн" | "множина" | "pl"   | "plural"   => Plural,
            _ => return Err(()),
        })
    }
}

impl GrammaticalNumber {
    fn index(&self) -> usize {
        use GrammaticalNumber::*;
        match self {
            Singular => 0,
            Plural => 1,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
struct GrammaticalProperties {
    gender: Gender,
    number: GrammaticalNumber,
    declenation: Declenation,
}

impl GrammaticalProperties {
    /*fn agreement_with_num(&self, num: u64) -> GrammaticalProperties {
        let tail = num % 100;
        let units = tail%10;
        let tens = tail/10;
        self.agreement_with_units(tens as usize, units as usize)
    }*/

    fn agreement_with_units(&self, tens: usize, units: usize) -> GrammaticalProperties {
        if units == 0 || units > 4 || tens == 1 {
            GrammaticalProperties {
                number: GrammaticalNumber::Plural,
                ..*self
            }
        } else if units == 1 {
            GrammaticalProperties {
                number: GrammaticalNumber::Singular,
                ..*self
            }
        } else {
            //2..4

            GrammaticalProperties {
                number: GrammaticalNumber::Plural,
                declenation: if self.declenation == Declenation::Nominative {
                    Declenation::Genitive
                } else {
                    self.declenation
                },
                ..*self
            }
        }
    }
}

pub struct Ukrainian {
    properties: GrammaticalProperties,
}

const MINUS: &str = "мінус";

const INFINITY: [&str; 6] = [
    "нескінченність",
    "нескінченності",
    "нескінченності",
    "нескінченність",
    "нескінченністю",
    "нескінченності",
    //"нескінченносте",
];

const ZERO: [&str; 6] = [
    "нуль",
    "нуля",
    "нулю",
    "нуль",
    "нулем",
    "нулі", //"нулю"
];

#[rustfmt::skip]
const GENDERED: [[[&str; 6]; 3];2] = [[
    [ "один", "одного", "одному", "один", "одним", "одному" ],
    [ "одна", "одної",  "одній",  "одну", "одною", "одній"  ],
    [ "одне", "одного", "одному", "одне", "одним", "одному" ],
],
[
    [ "два", "двох", "двом", "два", "двома", "двох" ],
    [ "дві", "двох", "двом", "дві", "двома", "двох" ],
    [ "два", "двох", "двом", "два", "двома", "двох" ],
]];

#[rustfmt::skip]
const UNITS: [[&str; 6]; 7] = [
    [ "три",     "трьох",    "трьом",    "три",     "трьома",    "трьох"    ],
    [ "чотири",  "чотирьох", "чотирьом", "чотири",  "чотирма",   "чотирьох" ],
    [ "пʼять",   "пʼяти",    "пʼяти",    "пʼять",   "пʼятьма",   "пʼяти"    ],
    [ "шість",   "шести",    "шісти",    "шість",   "шістьма",   "шести"    ],
    [ "сім",     "семи",     "семи",     "сім",     "сімома",    "семи"     ],
    [ "вісім",   "восьми",   "восьми",   "вісім",   "вісьма",    "восьми"   ],
    [ "девʼять", "девʼяти",  "девʼяти",  "девʼять", "девʼятьма", "девʼяти"  ],
];

#[rustfmt::skip]
const TEENS: [[&str; 6]; 10] = [
    [ "десять",        "десяти",        "десяти",        "десять",        "десятьом",        "десяти"        ],
    [ "одинадцять",    "одинадцяти",    "одинадцяти",    "одинадцять",    "одинадцятьма",    "одинадцяти"    ],
    [ "дванадцять",    "дванадцяти",    "дванадцяти",    "дванадцять",    "дванадцятьма",    "дванадцяти"    ],
    [ "тринадцять",    "тринадцяти",    "тринадцяти",    "тринадцять",    "тринадцятьма",    "тринадцяти"    ],
    [ "чотирнадцять",  "чотирнадцяти",  "чотирнадцяти",  "чотирнадцять",  "чотирнадцятьма",  "чотирнадцяти"  ],
    [ "пʼятнадцять",   "пʼятнадцяти",   "пʼятнадцяти",   "пʼятнадцять",   "пʼятнадцятьма",   "пʼятнадцяти"   ],
    [ "шістнадцять",   "шістнадцяти",   "шістнадцяти",   "шістнадцять",   "шістнадцятьма",   "шістнадцяти"   ],
    [ "сімнадцять",    "сімнадцяти",    "сімнадцяти",    "сімнадцять",    "сімнадцятьма",    "сімнадцяти"    ],
    [ "вісімнадцять",  "вісімнадцяти",  "вісімнадцяти",  "вісімнадцять",  "вісімнадцятьма",  "вісімнадцяти"  ],
    [ "девʼятнадцять", "девʼятнадцяти", "девʼятнадцяти", "девʼятнадцять", "девʼятнадцятьма", "девʼятнадцяти" ],
];

#[rustfmt::skip]
const TENS: [[&str; 6]; 8] = [
    [ "двадцять",   "двадцяти",    "двадцяти",    "двадцять",   "двадцятьма",     "двадцяти"    ],
    [ "тридцять",   "тридцяти",    "тридцяти",    "тридцять",   "тридцятьма",    "тридцяти"    ],
    [ "сорок",      "сорока",      "сорока",      "сорок",      "сорока",        "сорока"      ],
    [ "пʼятдесят",  "пʼятдесяти",  "пʼятдесяти",  "пʼятдесят",  "пʼятдесятьма",  "пʼятдесяти"  ],
    [ "шістдесят",  "шістдесяти",  "шістдесяти",  "шістдесят",  "шістдесятьма",  "шістдесяти"  ],
    [ "сімдесят",   "сімдесяти",   "сімдесяти",   "сімдесять",  "сімдесятьма",   "сімдесяти"   ],
    [ "вісімдесят", "вісімдесяти", "вісімдесяти", "вісімдесят", "вісімдесятьма", "вісімдесяти" ],
    [ "девʼяносто", "девʼяноста",  "девʼяноста",  "девʼяносто", "девʼяноста",    "девʼяноста"  ],
];

#[rustfmt::skip]
const HUNDREDS: [[&str; 6]; 9] = [
    [ "сто",       "ста",         "ста",          "сто",       "ста",            "ста"          ],
    [ "двісті",    "двохсот",     "двомстам",     "двісті",    "двомастами",     "двохстах"     ],
    [ "триста",    "трьохсот",    "трьомстам",    "триста",    "трьомастами",    "трьохстах"    ],
    [ "чотириста", "чотирьохсот", "чотирьомстам", "чотириста", "чотирмастами",   "чотирьохстах" ],
    [ "пʼятсот",   "пʼятисот",    "пʼятистам",    "пʼятсот",   "пʼятьмастами",   "пʼятистах"    ],
    [ "шістсот",   "шестисот",    "шестистам",    "шістсот",   "шістьмастами",   "шестистах"    ],
    [ "сімсот",    "семисот",     "семистам",     "сімсот",    "сімомастами",    "семистах"     ],
    [ "вісімсот",  "восьмисот",   "восьмистам",   "вісімсот",  "восьмистами",    "восьмистах"   ],
    [ "девʼятсот", "девʼятисот",  "девʼятистам",  "девʼятсот", "девʼятьмастами", "девʼятистах"  ],
];

#[rustfmt::skip]
const THOUSAND: [[&str; 6]; 2] = [
    ["тисяча", "тисячі", "тисячі",  "тисячу", "тисячею",  "тисячі"  ],
    ["тисячі", "тисяч",  "тисячам", "тисячі", "тисячами", "тисячах" ],
];

// Number names by "rule n-1" from https://uk.wikipedia.org/wiki/Іменні_назви_степенів_тисячі
const MEGA_BASES: [&str; 20] = [
    "мільйон",
    "мільярд",
    "трильйон",
    "квадрильйон",
    "квінтильйон",
    "секстильйон",
    "септильйон",
    "октильйон",
    "нонильйон",
    "децильйон",
    "ундецильйон",
    "додецильйон",
    "тредецильйон",
    "кваттуордецильйон",
    "квіндецильйон",
    "седецильйон",
    "септдецильйон",
    "дуодевігінтильйон",
    "ундевігінтильйон",
    "вігінтильйон",
];

#[rustfmt::skip]
const MEGA_DESINENSES: [[&str; 6]; 2] = [
    [ "",  "а",  "у",  "",  "ом",  "і" ],
    [ "и", "ів", "ам", "и", "ами", "и" ],
];

impl Ukrainian {
    #![allow(dead_code)]

    pub fn new(gender: Gender, number: GrammaticalNumber, declenation: Declenation) -> Self {
        Self {
            properties: GrammaticalProperties {
                gender,
                number,
                declenation,
            },
        }
    }

    fn currencies(&self, _currency: Currency, _plural_form: bool) -> String {
        todo!()
        //currency.default_string(plural_form)
    }

    fn cents(&self, _currency: Currency, _plural_form: bool) -> String {
        todo!()
        //currency.default_cent_string(plural_form)
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
            return Ok(String::from(ZERO[self.properties.declenation.index()]));
        }

        // handling negative values
        let mut words = vec![];
        if num.is_negative() {
            words.push(String::from(MINUS));
            num = -num;
        }

        // iterate over thousands
        for (i, triplet) in self.split_thousands(num).iter().enumerate().rev() {
            let hundreds = (triplet / 100 % 10) as usize;
            let tens = (triplet / 10 % 10) as usize;
            let units = (triplet % 10) as usize;

            if hundreds > 0 {
                words.push(String::from(
                    HUNDREDS[hundreds - 1][self.properties.declenation.index()],
                ));
            }

            let properties = GrammaticalProperties {
                gender: match i {
                    0 => self.properties.gender,
                    1 => Gender::Feminine, //тисяча жіночого роду
                    _ => Gender::Masculine,
                },
                ..self.properties
            }
            .agreement_with_units(tens, units);

            if tens == 1 {
                words.push(String::from(TEENS[units][properties.declenation.index()]));
            } else {
                if tens > 1 {
                    words.push(String::from(TENS[tens - 2][properties.declenation.index()]));
                }
                if units == 1 || units == 2 {
                    words.push(String::from(
                        GENDERED[units - 1][properties.gender.index()]
                            [properties.declenation.index()],
                    ));
                } else if units > 0 {
                    words.push(String::from(
                        UNITS[units - 3][properties.declenation.index()],
                    ));
                }
            }

            if i != 0 && triplet != &0 {
                if i + 1 > MEGA_BASES.len() {
                    return Err(Num2Err::CannotConvert);
                }
                if i == 1 {
                    words.push(String::from(
                        THOUSAND[properties.number.index()][properties.declenation.index()],
                    ))
                } else {
                    words.push(format!(
                        "{}{}",
                        MEGA_BASES[i - 2],
                        MEGA_DESINENSES[properties.number.index()][properties.declenation.index()]
                    ));
                }
            }
        }

        Ok(words.join(" "))
    }

    fn float_to_cardinal(&self, _num: BigFloat) -> Result<String, Num2Err> {
        todo!()
        /*let integral_part = num.int();
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
                0 => String::from(if self.prefer_oh { "oh" } else { "zero" }),
                i => String::from(UNITS[i as usize - 1]),
            });
        }
        Ok(words.join(" "))*/
    }
}

impl Language for Ukrainian {
    fn to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num.is_inf_pos() {
            Ok(String::from(INFINITY[self.properties.declenation.index()]))
        } else if num.is_inf_neg() {
            Ok(format!(
                "мінус {}",
                INFINITY[self.properties.declenation.index()]
            ))
        } else if num.frac().is_zero() {
            self.int_to_cardinal(num)
        } else {
            self.float_to_cardinal(num)
        }
    }

    fn to_ordinal(&self, _num: BigFloat) -> Result<String, Num2Err> {
        todo!()
        /*let cardinal_word = self.to_cardinal(num)?;

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

        Ok(words.join(" "))*/
    }

    fn to_ordinal_num(&self, _num: BigFloat) -> Result<String, Num2Err> {
        todo!()
        /*Ok(format!(
            "{}{}",
            num.to_u128().unwrap(),
            match (num % BigFloat::from(10)).to_u64().unwrap() {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
        ))*/
    }

    fn to_year(&self, _num: BigFloat) -> Result<String, Num2Err> {
        todo!()
        /*if !num.frac().is_zero() {
            return Err(Num2Err::FloatingYear);
        }

        let mut num = num;

        let mut suffix = "";
        if num.is_negative() {
            num = num.inv_sign();
            suffix = " BC";
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
                String::from("hundred")
            } else if low < 10 {
                format!("oh-{}", self.int_to_cardinal(BigFloat::from(low))?)
            } else {
                self.int_to_cardinal(BigFloat::from(low))?
            };

            format!("{} {}", high_word, low_word)
        };

        Ok(format!("{}{}", year_word, suffix))*/
    }

    fn to_currency(&self, _num: BigFloat, _currency: Currency) -> Result<String, Num2Err> {
        todo!()
        /*if num.is_inf() {
            Ok(format!(
                "{}an infinity of {}",
                if num.is_negative() { "minus " } else { "" },
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
                    "{} and {} {}",
                    integral_word, cents_words, cents_suffix
                ))
            }
        }*/
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_cardinal() {
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Ukrainian)
                .cardinal()
                .to_words(),
            Ok(String::from("нуль"))
        );
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Ukrainian)
                .prefer("р")
                .cardinal()
                .to_words(),
            Ok(String::from("нуля"))
        );
        assert_eq!(
            Num2Words::new(1)
                .lang(Lang::Ukrainian)
                .prefer("loc")
                .cardinal()
                .to_words(),
            Ok(String::from("одному"))
        );
        assert_eq!(
            Num2Words::new(1)
                .lang(Lang::Ukrainian)
                .prefer("f")
                .cardinal()
                .to_words(),
            Ok(String::from("одна"))
        );
        assert_eq!(
            Num2Words::new(1)
                .lang(Lang::Ukrainian)
                .prefer("f")
                .prefer("ins")
                .cardinal()
                .to_words(),
            Ok(String::from("одною"))
        );
        assert_eq!(
            Num2Words::new(2)
                .lang(Lang::Ukrainian)
                .prefer("f")
                .prefer("acc")
                .cardinal()
                .to_words(),
            Ok(String::from("дві"))
        );
        assert_eq!(
            Num2Words::new(918654321).lang(Lang::Ukrainian).prefer("f").prefer("dat").cardinal().to_words(),
            Ok(String::from("девʼятистам вісімнадцяти мільйонам шестистам пʼятдесяти чотирьом тисячам трьомстам двадцяти одній"))
        );
        assert_eq!(
            Num2Words::new(918654321).lang(Lang::Ukrainian).prefer("ч").prefer("о").cardinal().to_words(),
            Ok(String::from("девʼятьмастами вісімнадцятьма мільйонами шістьмастами пʼятдесятьма чотирма тисячами трьомастами двадцятьма одним"))
        );
    }

    #[test]
    fn test_declenation_agreement() {
        assert_eq!(
            GrammaticalProperties {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declenation: Declenation::Nominative
            }
            .agreement_with_units(0, 0),
            GrammaticalProperties {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Plural,
                declenation: Declenation::Nominative
            }
        );
        assert_eq!(
            GrammaticalProperties {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declenation: Declenation::Nominative
            }
            .agreement_with_units(0, 1),
            GrammaticalProperties {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declenation: Declenation::Nominative
            }
        );
        assert_eq!(
            GrammaticalProperties {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Singular,
                declenation: Declenation::Nominative
            }
            .agreement_with_units(8, 2),
            GrammaticalProperties {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Plural,
                declenation: Declenation::Genitive
            }
        );
        assert_eq!(
            GrammaticalProperties {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declenation: Declenation::Dative
            }
            .agreement_with_units(1, 1),
            GrammaticalProperties {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Plural,
                declenation: Declenation::Dative
            }
        );
        assert_eq!(
            GrammaticalProperties {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Singular,
                declenation: Declenation::Instrumental
            }
            .agreement_with_units(5, 4),
            GrammaticalProperties {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Plural,
                declenation: Declenation::Instrumental
            }
        );
    }
}
