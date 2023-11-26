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
                declenation: if self.declenation == Declenation::Nominative {
                    Declenation::Genitive
                } else {
                    self.declenation
                },
                ..*self
            }
        } else if units == 1 {
            GrammaticalProperties {
                number: GrammaticalNumber::Singular,
                ..*self
            }
        } else {  //units in 2..4
            GrammaticalProperties {
                number: GrammaticalNumber::Plural,
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

const ORDINAL_ZERO_BASE: &str = "нульов";

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

const ONE_BASE: &str = "одно";

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

const ORDINAL_UNIT_BASES: [&str; 9] = [
    "перш", "друг", "трет", "четверт", "пʼят", "шост", "сьом", "восьм", "девʼят"
];

const TEENS_BASES: [&str; 10] = [
     "десят",
     "одинадцят",
     "дванадцят",  
     "тринадцят",  
     "чотирнадцят",
     "пʼятнадцят", 
     "шістнадцят", 
     "сімнадцят",  
     "вісімнадцят",
     "девʼятнадцят"
];

const TEENS_FLEXIONS: [&str; 6] = [
     "ь", "и", "и", "ь", "ьма", "и"        
];

/*#[rustfmt::skip]
const TEENS: [[&str; 6]; 10] = [
    [ "десять",        "десяти",        "десяти",        "десять",        "десятьма",        "десяти"        ],
    [ "одинадцять",    "одинадцяти",    "одинадцяти",    "одинадцять",    "одинадцятьма",    "одинадцяти"    ],
    [ "дванадцять",    "дванадцяти",    "дванадцяти",    "дванадцять",    "дванадцятьма",    "дванадцяти"    ],
    [ "тринадцять",    "тринадцяти",    "тринадцяти",    "тринадцять",    "тринадцятьма",    "тринадцяти"    ],
    [ "чотирнадцять",  "чотирнадцяти",  "чотирнадцяти",  "чотирнадцять",  "чотирнадцятьма",  "чотирнадцяти"  ],
    [ "пʼятнадцять",   "пʼятнадцяти",   "пʼятнадцяти",   "пʼятнадцять",   "пʼятнадцятьма",   "пʼятнадцяти"   ],
    [ "шістнадцять",   "шістнадцяти",   "шістнадцяти",   "шістнадцять",   "шістнадцятьма",   "шістнадцяти"   ],
    [ "сімнадцять",    "сімнадцяти",    "сімнадцяти",    "сімнадцять",    "сімнадцятьма",    "сімнадцяти"    ],
    [ "вісімнадцять",  "вісімнадцяти",  "вісімнадцяти",  "вісімнадцять",  "вісімнадцятьма",  "вісімнадцяти"  ],
    [ "девʼятнадцять", "девʼятнадцяти", "девʼятнадцяти", "девʼятнадцять", "девʼятнадцятьма", "девʼятнадцяти" ],
];*/

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

const ORDINAL_TENS_BASES: [&str; 9] = [
    "десят", "двадцят", "тридцят", "сороков", "пʼятдесят", "шістдесят", "сімдесят", "вісімдесят", "девʼяност"
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

const HUNDRED_BASE: &str = "сот";

#[rustfmt::skip]
const THOUSAND_FLEXIONS: [[&str; 6]; 2] = [
    [ "а", "і", "і",  "у", "ею",  "і"  ],
    [ "і", "",  "ам", "і", "ами", "ах" ],
];

// Number names by "rule n-1" from https://uk.wikipedia.org/wiki/Іменні_назви_степенів_тисячі
const MEGA_BASES: [&str; 21] = [
    "тисяч",
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
const MEGA_FLEXIONS: [[&str; 6]; 2] = [
    [ "",  "а",  "у",  "",  "ом",  "і" ],
    [ "и", "ів", "ам", "и", "ами", "и" ],
];

#[rustfmt::skip]
const ORDINAL_HARD_FLEXIONS_SINGULAR: [[&str; 6]; 3] = [
    ["ий", "ого", "ому", "ий", "им",  "ому" ],
    ["а",  "ої",  "ій",  "у",  "ою",  "ій"  ],
    ["е",  "ого", "ому", "е",  "им",  "ому" ], 
];

const ORDINAL_HARD_FLEXIONS_PLURAL: [&str; 6] = [
    "і",  "их",  "им",  "их", "ими", "их",
];

#[rustfmt::skip]
const ORDINAL_SOFT_FLEXIONS_SINGULAR: [[&str; 6]; 3] = [
    ["ій", "ього", "ьому", "ій", "ім",  "ьому" ],
    ["я",  "ьої",  "ій",   "ю",  "ьою", "ій"   ],
    ["є",  "ього", "ьому", "є",  "ім",  "ьому" ], 
];

const ORDINAL_SOFT_FLEXIONS_PLURAL: [&str; 6] = [
    "і",  "іх",   "ім",   "іх", "іми", "іх"
];


impl Ukrainian {
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
                words.push(format!("{}{}", TEENS_BASES[units], TEENS_FLEXIONS[self.properties.declenation.index()]));
            } else {
                if tens > 1 {
                    words.push(String::from(TENS[tens - 2][self.properties.declenation.index()]));
                }
                if units == 1 || units == 2 {
                    words.push(String::from(
                        GENDERED[units - 1][self.properties.gender.index()]
                            [self.properties.declenation.index()],
                    ));
                } else if units > 0 {
                    words.push(String::from(
                        UNITS[units - 3][self.properties.declenation.index()],
                    ));
                }
            }

            if i != 0 && triplet != &0 {
                if i > MEGA_BASES.len() {
                    return Err(Num2Err::CannotConvert);
                }
                let mega_flexion = if i==1 {
                    THOUSAND_FLEXIONS[properties.number.index()][properties.declenation.index()]
                } else {
                    MEGA_FLEXIONS[properties.number.index()][properties.declenation.index()]
                };
                words.push(
                    format!("{}{}",
                    MEGA_BASES[i - 1],
                    mega_flexion
                ));
            }
        }

        Ok(words.join(" "))
    }

    fn float_to_cardinal(&self, _num: BigFloat) -> Result<String, Num2Err> {
        todo!()
    }

    fn ordinal_flexion(&self, num: BigFloat) -> &'static str {
        let tail = (num % BigFloat::from(100)).to_u64().unwrap();
        let ends_with_3 = tail%10==3 && tail!=13; //третій - the only soft adjective in numbers        
        let f = match (self.properties.number, ends_with_3) {
            (GrammaticalNumber::Plural, true) => ORDINAL_SOFT_FLEXIONS_PLURAL,
            (GrammaticalNumber::Plural, false) => ORDINAL_HARD_FLEXIONS_PLURAL,
            (_, true) => ORDINAL_SOFT_FLEXIONS_SINGULAR[self.properties.gender.index()],
            (_, false) => ORDINAL_HARD_FLEXIONS_SINGULAR[self.properties.gender.index()],
        };
        f[self.properties.declenation.index()]
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

    fn to_ordinal(&self, mut num: BigFloat) -> Result<String, Num2Err> {
        let flexion = self.ordinal_flexion(num);

        // special case zero
        if num.is_zero() {
            return Ok(format!("{ORDINAL_ZERO_BASE}{flexion}"));
        }

        // handling negative values
        let mut words = vec![];
        if num.is_negative() {
            words.push(String::from(MINUS));
            num = -num;
        }

        let triplets = self.split_thousands(num);
        let last_non_empty = triplets.iter().position(|&t|t!=0).unwrap();

        // iterate over thousands
        for (i, triplet) in triplets.iter().enumerate().rev() {
            let hundreds = (triplet / 100 % 10) as usize;
            let tens = (triplet / 10 % 10) as usize;
            let units = (triplet % 10) as usize;

            if i == last_non_empty {
                if i!=0 {
                    //п’ятсоттридцятитрьохтисячний
                    let mut word = String::new();
                    if hundreds>0 {
                        word.push_str(HUNDREDS[hundreds-1][Declenation::Genitive.index()]);
                    }
                    if tens==1 {
                        word.push_str(&format!("{}{}", TEENS_BASES[units], TEENS_FLEXIONS[Declenation::Genitive.index()]));
                    } else {
                        if tens>1 {
                            word.push_str(TENS[tens-2][Declenation::Genitive.index()]);
                        }
                        match units {
                            1 => word.push_str(ONE_BASE),
                            2 => word.push_str(GENDERED[1][Gender::Masculine.index()][Declenation::Genitive.index()]),
                            3..=9 => word.push_str(UNITS[units-3][Declenation::Genitive.index()]),
                            _ => (),
                        }
                    }
                    word.push_str(&format!("{}н{flexion}", MEGA_BASES[i-1]));
                    words.push(word);
                } else if tens==0 && units==0 {
                    words.push(format!("{HUNDRED_BASE}{flexion}"));
                } else {
                    if hundreds>0 {
                        words.push(String::from(HUNDREDS[hundreds-1][Declenation::Nominative.index()]));
                    }
                    if tens==1 {
                        words.push(format!("{}{flexion}", TEENS_BASES[units]));
                    } else if units==0 {
                        words.push(format!("{}{flexion}", ORDINAL_TENS_BASES[tens-1]));
                    } else {
                        if tens>1 {
                            words.push(String::from(TENS[tens-2][Declenation::Nominative.index()]));
                        }
                        let flexion = self.ordinal_flexion(BigFloat::from(units as u8));
                        words.push(format!("{}{flexion}", ORDINAL_UNIT_BASES[units-1]));
                    }                        
                }
                break;
            }

            if hundreds > 0 {
                words.push(String::from(
                    HUNDREDS[hundreds - 1][Declenation::Nominative.index()],
                ));
            }

            let properties = GrammaticalProperties {
                gender: match i {
                    1 => Gender::Feminine, //тисяча жіночого роду
                    _ => Gender::Masculine,
                },
                number: GrammaticalNumber::Singular,
                declenation: Declenation::Nominative,
            }
            .agreement_with_units(tens, units);

            if tens == 1 {
                words.push(format!("{}{}", TEENS_BASES[units], TEENS_FLEXIONS[Declenation::Nominative.index()]));
            } else {
                if tens > 1 {
                    words.push(String::from(TENS[tens - 2][Declenation::Nominative.index()]));
                }
                if units == 1 || units == 2 {
                    words.push(String::from(
                        GENDERED[units - 1][properties.gender.index()]
                            [Declenation::Nominative.index()],
                    ));
                } else if units > 0 {
                    words.push(String::from(
                        UNITS[units - 3][Declenation::Nominative.index()],
                    ));
                }
            }

            if i != 0 && triplet != &0 {
                let mega_flexion = if i==1 {
                    THOUSAND_FLEXIONS[properties.number.index()][properties.declenation.index()]
                } else {
                    MEGA_FLEXIONS[properties.number.index()][properties.declenation.index()]
                };
                words.push(
                    format!("{}{}",
                    MEGA_BASES[i - 1],
                    mega_flexion
                ));

            }
        }

        Ok(words.join(" "))
    }

    fn to_ordinal_num(&self, num: BigFloat) -> Result<String, Num2Err> {
        let flexion = self.ordinal_flexion(num);
        Ok(format!("{}-{flexion}", num.to_u128().unwrap()))
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
        assert_eq!(
            Num2Words::new(18000000).lang(Lang::Ukrainian).cardinal().to_words(),
            Ok(String::from("вісімнадцять мільйонів"))
        );

    }

    #[test]
    fn test_ordinal_num() {
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .to_words(),
            Ok(String::from("0-ий"))
        );
        assert_eq!(
            Num2Words::new(23)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .prefer("ж")
                .prefer("орудний")
                .to_words(),
            Ok(String::from("23-ьою"))
        );
        assert_eq!(
            Num2Words::new(1000)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .prefer("множина")
                .prefer("давальний")
                .to_words(),
            Ok(String::from("1000-им"))
        );
        assert_eq!(
            Num2Words::new(13)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .prefer("множина")
                .prefer("давальний")
                .to_words(),
            Ok(String::from("13-им"))
        );
        assert_eq!(
            Num2Words::new(321)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .prefer("жіночий")
                .to_words(),
            Ok(String::from("321-а"))
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
                declenation: Declenation::Genitive
            },
            "failed agreement: 0"
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
            },
            "failed agreement: 1"
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
                declenation: Declenation::Nominative
            },
            "failed agreement: 82"
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
            },
            "failed agreement: 11"
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
            },
            "failed agreement: 54"
        );
        assert_eq!(
            GrammaticalProperties {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declenation: Declenation::Nominative
            }
            .agreement_with_units(1, 8),
            GrammaticalProperties {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Plural,
                declenation: Declenation::Genitive
            },
            "failed agreement: 18"
        );
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Ukrainian)
                .ordinal()
                .to_words(),
            Ok(String::from("нульовий"))
        );
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Ukrainian)
                .prefer("р")
                .ordinal()
                .to_words(),
            Ok(String::from("нульового"))
        );
        assert_eq!(
            Num2Words::new(1)
                .lang(Lang::Ukrainian)
                .prefer("loc")
                .ordinal()
                .to_words(),
            Ok(String::from("першому"))
        );
        assert_eq!(
            Num2Words::new(1)
                .lang(Lang::Ukrainian)
                .prefer("f")
                .ordinal()
                .to_words(),
            Ok(String::from("перша"))
        );
        assert_eq!(
            Num2Words::new(1)
                .lang(Lang::Ukrainian)
                .prefer("f")
                .prefer("ins")
                .ordinal()
                .to_words(),
            Ok(String::from("першою"))
        );
        assert_eq!(
            Num2Words::new(2)
                .lang(Lang::Ukrainian)
                .prefer("f")
                .prefer("acc")
                .ordinal()
                .to_words(),
            Ok(String::from("другу"))
        );
        assert_eq!(
            Num2Words::new(918654321).lang(Lang::Ukrainian).prefer("f").prefer("dat").ordinal().to_words(),
            Ok(String::from("девʼятсот вісімнадцять мільйонів шістсот пʼятдесят чотири тисячі триста двадцять першій"))
        );
        assert_eq!(
            Num2Words::new(918654321).lang(Lang::Ukrainian).prefer("ч").prefer("о").ordinal().to_words(),
            Ok(String::from("девʼятсот вісімнадцять мільйонів шістсот пʼятдесят чотири тисячі триста двадцять першим"))
        );
        assert_eq!(
            Num2Words::new(123456000).lang(Lang::Ukrainian).ordinal().to_words(),
            Ok(String::from("сто двадцять три мільйони чотирьохсотпʼятдесятишеститисячний"))
        );
        assert_eq!(
            Num2Words::new(1000000).lang(Lang::Ukrainian).ordinal().to_words(),
            Ok(String::from("одномільйонний"))
        );
    }
}
