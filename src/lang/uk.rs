use crate::{num2words::Num2Err, Currency, Language};
use num_bigfloat::BigFloat;
use std::str::FromStr;

// Source: Ukrainian Orthography 2019 / Український Правопис 2019
// § 38. Constructed numerals / Складні числівники
// § 105. Cardinal numerals declension / Відмінювання кількісних числівників
// § 106. Ordinal numerals declension / Відмінювання порядкових числівників
// § 107. Fractional numerals declension / Відмінювання дробових числівників

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum Declension {
    #[default]
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Instrumental,
    Locative,
}

impl Declension {
    fn index(&self) -> usize {
        use Declension::*;
        match self {
            Nominative => 0,
            Genitive => 1,
            Dative => 2,
            Accusative => 3,
            Instrumental => 4,
            Locative => 5,
        }
    }
}

impl FromStr for Declension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Declension::*;

        Ok(match s.to_lowercase().as_str() {
            "н" | "називний" | "nom" | "nominative" => Nominative,
            "р" | "родовий" | "gen" | "genitive" => Genitive,
            "д" | "давальний" | "dat" | "dative" => Dative,
            "з" | "знахідний" | "acc" | "accusative" => Accusative,
            "о" | "орудний" | "ins" | "instrumental" => Instrumental,
            "м" | "місцевий" | "loc" | "locative" => Locative,
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Gender::*;
        Ok(match s.to_lowercase().as_str() {
            "ч" | "чол" | "чоловічий" | "m" | "masculine" => Masculine,
            "ж" | "жін" | "жіночий" | "f" | "feminine" => Feminine,
            "с" | "сер" | "середній" | "n" | "neuter" => Neuter,
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GrammaticalNumber::*;
        Ok(match s.to_lowercase().as_str() {
            "од" | "однина" | "sing" | "singular" => Singular,
            "мн" | "множина" | "pl" | "plural" => Plural,
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
pub struct Ukrainian {
    gender: Gender,
    number: GrammaticalNumber,
    declension: Declension,
}

impl Ukrainian {
    fn masculine(&self) -> Self {
        Self {
            gender: Gender::Masculine,
            ..*self
        }
    }
    fn feminine(&self) -> Self {
        Self {
            gender: Gender::Feminine,
            ..*self
        }
    }
    fn set_declension(&self, declension: Declension) -> Self {
        Self {
            declension,
            ..*self
        }
    }
    fn singular(&self) -> Self {
        Self {
            number: GrammaticalNumber::Singular,
            ..*self
        }
    }
    fn plural(&self) -> Self {
        Self {
            number: GrammaticalNumber::Plural,
            ..*self
        }
    }
    fn is_plural(&self) -> bool {
        self.number == GrammaticalNumber::Plural
    }

    fn agreement_with_num(&self, num: BigFloat) -> Ukrainian {
        let num = num.to_u64().unwrap_or_default(); //0 and inf has the same plural properties
        let tail = num % 100;
        let units = tail % 10;
        let tens = tail / 10;
        self.agreement_with_units(tens as usize, units as usize)
    }

    fn agreement_with_units(&self, tens: usize, units: usize) -> Ukrainian {
        if units == 0 || units > 4 || tens == 1 {
            if self.declension == Declension::Nominative {
                self.plural().set_declension(Declension::Genitive)
            } else {
                self.plural()
            }
        } else if units == 1 {
            self.singular()
        } else {
            //units in 2..4
            self.plural()
        }
    }
}

const MINUS: &str = "мінус";

const INFINITY: [&str; 6] = [
    "нескінченність",
    "нескінченності",
    "нескінченності",
    "нескінченність",
    "нескінченністю",
    "нескінченності",
];

const ZERO: [&str; 6] = ["нуль", "нуля", "нулю", "нуль", "нулем", "нулі"];

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
    "перш",
    "друг",
    "трет",
    "четверт",
    "пʼят",
    "шост",
    "сьом",
    "восьм",
    "девʼят",
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
    "девʼятнадцят",
];

const TEENS_FLEXIONS: [&str; 6] = ["ь", "и", "и", "ь", "ьма", "и"];

#[rustfmt::skip]
const TENS: [[&str; 6]; 8] = [
    [ "двадцять",   "двадцяти",    "двадцяти",    "двадцять",   "двадцятьма",     "двадцяти"    ],
    [ "тридцять",   "тридцяти",    "тридцяти",    "тридцять",   "тридцятьма",    "тридцяти"    ],
    [ "сорок",      "сорока",      "сорока",      "сорок",      "сорока",        "сорока"      ],
    [ "пʼятдесят",  "пʼятдесяти",  "пʼятдесяти",  "пʼятдесят",  "пʼятдесятьма",  "пʼятдесяти"  ],
    [ "шістдесят",  "шістдесяти",  "шістдесяти",  "шістдесят",  "шістдесятьма",  "шістдесяти"  ],
    [ "сімдесят",   "сімдесяти",   "сімдесяти",   "сімдесят",  "сімдесятьма",   "сімдесяти"   ],
    [ "вісімдесят", "вісімдесяти", "вісімдесяти", "вісімдесят", "вісімдесятьма", "вісімдесяти" ],
    [ "девʼяносто", "девʼяноста",  "девʼяноста",  "девʼяносто", "девʼяноста",    "девʼяноста"  ],
];

const ORDINAL_TENS_BASES: [&str; 9] = [
    "десят",
    "двадцят",
    "тридцят",
    "сороков",
    "пʼятдесят",
    "шістдесят",
    "сімдесят",
    "вісімдесят",
    "девʼяност",
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
const ADJECTIVE_HARD_FLEXIONS_SINGULAR: [[&str; 6]; 3] = [
    ["ий", "ого", "ому", "ий", "им",  "ому" ],
    ["а",  "ої",  "ій",  "у",  "ою",  "ій"  ],
    ["е",  "ого", "ому", "е",  "им",  "ому" ], 
];

const ADJECTIVE_HARD_FLEXIONS_PLURAL: [&str; 6] = ["і", "их", "им", "их", "ими", "их"];

#[rustfmt::skip]
const ADJECTIVE_SOFT_FLEXIONS_SINGULAR: [[&str; 6]; 3] = [
    ["ій", "ього", "ьому", "ій", "ім",  "ьому" ],
    ["я",  "ьої",  "ій",   "ю",  "ьою", "ій"   ],
    ["є",  "ього", "ьому", "є",  "ім",  "ьому" ], 
];

const ADJECTIVE_SOFT_FLEXIONS_PLURAL: [&str; 6] = ["і", "іх", "ім", "іх", "іми", "іх"];

#[rustfmt::skip]
const ORDINAL_HARD_FLEXIONS_SINGULAR_SHORT: [[&str; 6]; 3] = [
    ["й", "го", "му", "й", "м",  "му" ],
    ["а", "ї",  "й",  "у", "ою", "й"  ],
    ["е", "го", "му", "е", "м",  "му" ], 
];

#[rustfmt::skip]
const ORDINAL_SOFT_FLEXIONS_SINGULAR_SHORT: [[&str; 6]; 3] = [
    ["й", "го", "му", "й", "м",  "му" ],
    ["я", "ї",  "й",  "ю", "ою", "й"  ],
    ["є", "го", "му", "є", "м",  "му" ], 
];

const ORDINAL_FLEXIONS_PLURAL_SHORT: [&str; 6] = ["і", "х", "м", "х", "ми", "х"];

#[rustfmt::skip]
const NOUN_2ST_GROUP_HARD_DECLENSIONS: [[&str; 6]; 2] = [ //долар
    [ "",  "а",  "у",  "а", "ом",  "і"  ],
    [ "и", "ів", "ам", "и", "ами", "ах" ],
];

#[rustfmt::skip]
const NOUN_2ST_GROUP_SOFT_DECLENSIONS: [[&str; 6]; 2] = [ //юань
    [ "ь",  "я",  "ю", "я", "єм",  "і"  ],
    [ "і", "ів", "ям", "і", "ями", "ях" ],
];

#[rustfmt::skip]
const NOUN_1ST_GROUP_SOFT_DECLENSIONS_VOWEL: [[&str; 6]; 2] = [ //рупія
    [ "я", "ї", "ї",  "я", "єю",  "ї"  ],
    [ "ї", "й", "ям", "ї", "ями", "ях" ],
];

#[rustfmt::skip]
const NOUN_1ST_GROUP_HARD_DECLENSIONS: [[&str; 6]; 2] = [ //єна
    [ "а", "и", "і",  "а", "ою",  "і"  ],
    [ "и", "",  "ам", "и", "ами", "ах" ],
];

#[rustfmt::skip]
const HRYVNIAS: [[&str; 6]; 2] = [
    [ "гривня", "гривні",  "гривні",  "гривню", "гривнею",  "гривні"  ],
    [ "гривні", "гривень", "гривням", "гривні", "гривнями", "гривнях" ],
];

#[rustfmt::skip]
const KOPIYKAS: [[&str; 6]; 2] = [
    [ "копійка", "копійки", "копійці",  "копійку", "копійкою",  "копійці"  ],
    [ "копійки", "копійок", "копійкам", "копійки", "копійками", "копійках" ],
];

#[rustfmt::skip]
const YEAR: [[&str; 6]; 2] = [
    [ "рік",  "року",  "року",  "рік",  "роком",  "році" ],
    [ "роки", "років", "рокам", "роки", "роками", "роках" ],
];

impl Ukrainian {
    pub fn new(gender: Gender, number: GrammaticalNumber, declension: Declension) -> Self {
        Self {
            gender,
            number,
            declension,
        }
    }

    fn currencies(&self, currency: Currency) -> String {
        let number_idx = self.number.index();
        let declension_idx = self.declension.index();
        match currency {
            Currency::AED => format!(
                "дирхам{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::ARS
            | Currency::CLP
            | Currency::COP
            | Currency::MXN
            | Currency::PESO
            | Currency::PHP
            | Currency::UYU => String::from("песо"),
            Currency::AUD
            | Currency::CAD
            | Currency::DOLLAR
            | Currency::HKD
            | Currency::NZD
            | Currency::SGD
            | Currency::TWD
            | Currency::USD => format!(
                "долар{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::BRL => format!(
                "реал{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::CHF => format!(
                "франк{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::CNY => format!(
                "юан{}",
                NOUN_2ST_GROUP_SOFT_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::CRC => format!(
                "колон{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::DINAR | Currency::DZD | Currency::KWD => format!(
                "динар{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::EUR => String::from("євро"),
            Currency::GBP => format!(
                "фунт{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::IDR | Currency::INR => format!(
                "рупі{}",
                NOUN_1ST_GROUP_SOFT_DECLENSIONS_VOWEL[number_idx][declension_idx]
            ),
            Currency::ILS => {
                let adjective_flextion = if self.number == GrammaticalNumber::Plural {
                    ADJECTIVE_HARD_FLEXIONS_PLURAL
                } else {
                    ADJECTIVE_HARD_FLEXIONS_SINGULAR[Gender::Masculine.index()]
                }[self.declension.index()];
                format!(
                    "нов{} шекел{}",
                    adjective_flextion, NOUN_2ST_GROUP_SOFT_DECLENSIONS[number_idx][declension_idx]
                )
            }
            Currency::JPY => format!(
                "єн{}",
                NOUN_1ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::KRW => format!(
                "вон{}",
                NOUN_1ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::KZT => String::from("tenge"),
            Currency::MYR => format!(
                "рингіт{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::NOK => format!(
                "крон{}",
                NOUN_1ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::PEN => format!(
                "сол{}",
                NOUN_2ST_GROUP_SOFT_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::PLN => {
                let flextion = if self.number == GrammaticalNumber::Plural {
                    ADJECTIVE_HARD_FLEXIONS_PLURAL
                } else {
                    ADJECTIVE_HARD_FLEXIONS_SINGULAR[Gender::Masculine.index()]
                }[declension_idx];
                format!("злот{}", flextion)
            }
            Currency::QAR | Currency::RIYAL | Currency::SAR => format!(
                "ріал{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::RUB => format!(
                "рубл{}",
                NOUN_1ST_GROUP_SOFT_DECLENSIONS_VOWEL[number_idx][declension_idx]
            ),
            Currency::THB => format!(
                "бат{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::TRY => format!(
                "куруш{}",
                NOUN_1ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::UAH => String::from(HRYVNIAS[number_idx][declension_idx]),
            Currency::VND => format!(
                "донг{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::ZAR => format!(
                "ранд{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
        }
    }

    fn currency_properties(&self, currency: Currency) -> Ukrainian {
        match currency {
            Currency::INR
            | Currency::JPY
            | Currency::KRW
            | Currency::NOK
            | Currency::TRY
            | Currency::UAH => self.feminine(),
            _ => self.masculine(),
        }
    }

    fn currency_fraction(&self, currency: Currency) -> String {
        let number_idx = self.number.index();
        let declension_idx = self.declension.index();
        match currency {
            Currency::AED => format!(
                "філс{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::ARS
            | Currency::CLP
            | Currency::COP
            | Currency::MXN
            | Currency::PESO
            | Currency::PHP
            | Currency::UYU
            | Currency::BRL => String::from("сентаво"),
            Currency::AUD
            | Currency::CAD
            | Currency::DOLLAR
            | Currency::HKD
            | Currency::NZD
            | Currency::SGD
            | Currency::TWD
            | Currency::USD => format!(
                "цент{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::CHF => format!(
                "сантим{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::CNY => format!(
                "фен{}",
                NOUN_2ST_GROUP_SOFT_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::CRC => String::from("сантимо"),
            Currency::DINAR | Currency::DZD | Currency::KWD => format!(
                "філс{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::EUR => format!(
                "євроцент{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::GBP => format!(
                "пенс{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::IDR => format!(
                "сен{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::INR => format!(
                "пайс{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::ILS => format!(
                "агор{}",
                NOUN_1ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::JPY => format!(
                "сен{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::KRW => format!(
                "чон{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::KZT => format!(
                "тиїн{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::MYR => format!(
                "сен{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::NOK => String::from("оре"),
            Currency::PEN => String::from("сентімо"),
            Currency::PLN => format!(
                "грош{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::QAR | Currency::RIYAL | Currency::SAR => format!(
                "філс{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::RUB => format!(
                "копійк{}",
                NOUN_1ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::THB => format!(
                "cатанг{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::TRY => format!(
                "лір{}",
                NOUN_1ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
            Currency::UAH => String::from(KOPIYKAS[number_idx][declension_idx]),
            Currency::VND => String::from("су"),
            Currency::ZAR => format!(
                "цент{}",
                NOUN_2ST_GROUP_HARD_DECLENSIONS[number_idx][declension_idx]
            ),
        }
    }

    fn currency_fraction_properties(&self, currency: Currency) -> Ukrainian {
        match currency {
            Currency::ILS | Currency::TRY | Currency::RUB | Currency::UAH => self.feminine(),
            _ => self.masculine(),
        }
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
            return Ok(String::from(ZERO[self.declension.index()]));
        }

        // handling negative values
        let mut words = vec![];
        if num.is_negative() {
            words.push(String::from(MINUS));
            num = -num;
        }

        // iterate over thousands
        for (order, triplet) in self.split_thousands(num).iter().enumerate().rev() {
            let hundreds = (triplet / 100 % 10) as usize;
            let tens = (triplet / 10 % 10) as usize;
            let units = (triplet % 10) as usize;

            if hundreds > 0 {
                words.push(String::from(
                    HUNDREDS[hundreds - 1][self.declension.index()],
                ));
            }

            let properties = match order {
                0 => *self,           //the last group agrees with target word
                1 => self.feminine(), //тисяча is feminite
                _ => self.masculine(),
            }
            .agreement_with_units(tens, units);

            if tens == 1 {
                words.push(format!(
                    "{}{}",
                    TEENS_BASES[units],
                    TEENS_FLEXIONS[self.declension.index()]
                ));
            } else {
                if tens > 1 {
                    words.push(String::from(TENS[tens - 2][self.declension.index()]));
                }
                if units == 1 || units == 2 {
                    let props = if order == 0 { self } else { &properties };
                    words.push(String::from(
                        GENDERED[units - 1][props.gender.index()][props.declension.index()],
                    ));
                } else if units > 0 {
                    words.push(String::from(UNITS[units - 3][self.declension.index()]));
                }
            }

            if order != 0 && triplet != &0 {
                if order > MEGA_BASES.len() {
                    return Err(Num2Err::CannotConvert);
                }
                let mega_flexion = if order == 1 {
                    THOUSAND_FLEXIONS[properties.number.index()][properties.declension.index()]
                } else {
                    MEGA_FLEXIONS[properties.number.index()][properties.declension.index()]
                };
                words.push(format!("{}{}", MEGA_BASES[order - 1], mega_flexion));
            }
        }

        Ok(words.join(" "))
    }

    fn float_to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        let whole = num.int();
        let mut numerator = num.frac().abs();
        if numerator.is_zero() {
            return self.int_to_cardinal(whole);
        }
        let mut denominator = BigFloat::from(1);
        while !numerator.frac().is_zero() {
            //TODO: we should use non-floating point format because of limited precision
            numerator *= BigFloat::from(10);
            denominator *= BigFloat::from(10);
        }
        let whole_properties = self.agreement_with_num(whole);
        let whole_flexion = if whole_properties.number == GrammaticalNumber::Plural {
            ADJECTIVE_HARD_FLEXIONS_PLURAL
        } else {
            ADJECTIVE_HARD_FLEXIONS_SINGULAR[Gender::Feminine.index()]
        }[whole_properties.declension.index()];

        let whole_lang = whole_properties.feminine();
        let numerator_properties = self.agreement_with_num(numerator);
        let numerator_lang = numerator_properties.feminine();
        Ok(format!(
            "{} ціл{} {} {}",
            whole_lang.int_to_cardinal(whole)?,
            whole_flexion,
            numerator_lang.int_to_cardinal(numerator)?,
            numerator_lang.to_ordinal(denominator)?,
        ))
    }

    fn ordinal_flexion(&self, num: BigFloat) -> &'static str {
        let tail = (num % BigFloat::from(100)).to_u64().unwrap();
        let is_soft = tail % 10 == 3 && tail != 13; //третій - the only soft adjective in numbers
        let f = match (self.is_plural(), is_soft) {
            (true, true) => &ADJECTIVE_SOFT_FLEXIONS_PLURAL,
            (true, false) => &ADJECTIVE_HARD_FLEXIONS_PLURAL,
            (false, true) => &ADJECTIVE_SOFT_FLEXIONS_SINGULAR[self.gender.index()],
            (false, false) => &ADJECTIVE_HARD_FLEXIONS_SINGULAR[self.gender.index()],
        };
        f[self.declension.index()]
    }

    fn ordinal_flexion_short(&self, num: BigFloat) -> &'static str {
        let f = if self.is_plural() {
            &ORDINAL_FLEXIONS_PLURAL_SHORT
        } else {
            let tail = (num % BigFloat::from(100)).to_u64().unwrap();
            if tail % 10 == 3 && tail != 13 {
                //третій - the only soft adjective in numbers
                &ORDINAL_SOFT_FLEXIONS_SINGULAR_SHORT[self.gender.index()]
            } else {
                &ORDINAL_HARD_FLEXIONS_SINGULAR_SHORT[self.gender.index()]
            }
        };
        f[self.declension.index()]
    }
}

impl Language for Ukrainian {
    fn to_cardinal(&self, num: BigFloat) -> Result<String, Num2Err> {
        if num.is_inf_pos() {
            Ok(String::from(INFINITY[self.declension.index()]))
        } else if num.is_inf_neg() {
            Ok(format!("{MINUS} {}", INFINITY[self.declension.index()]))
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
        let last_non_empty = triplets.iter().position(|&t| t != 0).unwrap();

        //special case: one unit (thousand, million etc.) in the number
        if last_non_empty > 0
            && triplets[last_non_empty] == 1
            && triplets[last_non_empty + 1..].iter().all(|&t| t == 0)
        {
            return Ok(format!("{}н{flexion}", MEGA_BASES[last_non_empty - 1]));
        }

        // iterate over thousands
        for (order, triplet) in triplets.iter().enumerate().rev() {
            let hundreds = (triplet / 100 % 10) as usize;
            let tens = (triplet / 10 % 10) as usize;
            let units = (triplet % 10) as usize;

            if order == last_non_empty {
                if order != 0 {
                    //п’ятсоттридцятитрьохтисячний
                    let mut word = String::new();
                    if hundreds > 0 {
                        word.push_str(HUNDREDS[hundreds - 1][Declension::Genitive.index()]);
                    }
                    if tens == 1 {
                        word.push_str(&format!(
                            "{}{}",
                            TEENS_BASES[units],
                            TEENS_FLEXIONS[Declension::Genitive.index()]
                        ));
                    } else {
                        if tens > 1 {
                            word.push_str(TENS[tens - 2][Declension::Genitive.index()]);
                        }
                        match units {
                            1 => word.push_str(ONE_BASE),
                            2 => word.push_str(
                                GENDERED[1][Gender::Masculine.index()]
                                    [Declension::Genitive.index()],
                            ),
                            3..=9 => word.push_str(UNITS[units - 3][Declension::Genitive.index()]),
                            _ => (),
                        }
                    }
                    word.push_str(&format!("{}н{flexion}", MEGA_BASES[order - 1]));
                    words.push(word);
                } else if tens == 0 && units == 0 {
                    words.push(format!("{HUNDRED_BASE}{flexion}"));
                } else {
                    if hundreds > 0 {
                        words.push(String::from(
                            HUNDREDS[hundreds - 1][Declension::Nominative.index()],
                        ));
                    }
                    if tens == 1 {
                        words.push(format!("{}{flexion}", TEENS_BASES[units]));
                    } else if units == 0 {
                        words.push(format!("{}{flexion}", ORDINAL_TENS_BASES[tens - 1]));
                    } else {
                        if tens > 1 {
                            words
                                .push(String::from(TENS[tens - 2][Declension::Nominative.index()]));
                        }
                        let flexion = self.ordinal_flexion(BigFloat::from(units as u8));
                        words.push(format!("{}{flexion}", ORDINAL_UNIT_BASES[units - 1]));
                    }
                }
                break;
            }

            if hundreds > 0 {
                words.push(String::from(
                    HUNDREDS[hundreds - 1][Declension::Nominative.index()],
                ));
            }

            let properties = match order {
                1 => Ukrainian::default().feminine(), //тисяча is feminine
                _ => Ukrainian::default(),
            }
            .agreement_with_units(tens, units);

            if tens == 1 {
                words.push(format!(
                    "{}{}",
                    TEENS_BASES[units],
                    TEENS_FLEXIONS[Declension::Nominative.index()]
                ));
            } else {
                if tens > 1 {
                    words.push(String::from(TENS[tens - 2][Declension::Nominative.index()]));
                }
                if units == 1 || units == 2 {
                    words.push(String::from(
                        GENDERED[units - 1][properties.gender.index()]
                            [Declension::Nominative.index()],
                    ));
                } else if units > 0 {
                    words.push(String::from(
                        UNITS[units - 3][Declension::Nominative.index()],
                    ));
                }
            }

            if order != 0 && triplet != &0 {
                let mega_flexion = if order == 1 {
                    THOUSAND_FLEXIONS[properties.number.index()][properties.declension.index()]
                } else {
                    MEGA_FLEXIONS[properties.number.index()][properties.declension.index()]
                };
                words.push(format!("{}{}", MEGA_BASES[order - 1], mega_flexion));
            }
        }

        Ok(words.join(" "))
    }

    fn to_ordinal_num(&self, num: BigFloat) -> Result<String, Num2Err> {
        let flexion = self.ordinal_flexion_short(num);
        Ok(format!("{}-{flexion}", num.to_u128().unwrap()))
    }

    fn to_year(&self, num: BigFloat) -> Result<String, Num2Err> {
        if !num.frac().is_zero() {
            return Err(Num2Err::CannotConvert);
        }
        if num.is_inf() {
            return Err(Num2Err::InfiniteYear);
        }
        let year_lang = self.masculine();
        Ok(if num > BigFloat::from(0) {
            format!(
                "{} {}",
                year_lang.to_ordinal(num)?,
                YEAR[self.number.index()][self.declension.index()]
            )
        } else {
            format!(
                "{} {} до н.е.",
                year_lang.to_ordinal(-num)?,
                YEAR[self.number.index()][self.declension.index()]
            )
        })
    }

    fn to_currency(&self, num: BigFloat, currency: Currency) -> Result<String, Num2Err> {
        if num.is_inf() {
            let currency_lang = self.currency_properties(currency);
            let target_lang = currency_lang.agreement_with_num(num);
            Ok(format!(
                "{} {}",
                currency_lang.to_cardinal(num)?,
                target_lang.currencies(currency)
            ))
        } else {
            let whole = num.int();
            let fraction = num.frac();
            if fraction.is_zero() || num.is_inf() {
                let currency_lang = self.currency_properties(currency);
                let target_lang = currency_lang.agreement_with_num(whole);
                Ok(format!(
                    "{} {}",
                    currency_lang.int_to_cardinal(whole)?,
                    target_lang.currencies(currency)
                ))
            } else if whole.is_zero() {
                let fraction = fraction * BigFloat::from(100).int();
                let currency_lang = self.currency_fraction_properties(currency);
                let target_lang = currency_lang.agreement_with_num(fraction);
                Ok(format!(
                    "{} {}",
                    currency_lang.int_to_cardinal(fraction)?,
                    target_lang.currency_fraction(currency)
                ))
            } else {
                Ok(format!(
                    "{} {}",
                    self.to_currency(whole, currency)?,
                    self.to_currency(fraction, currency)?,
                ))
            }
        }
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
            Num2Words::new(18000000)
                .lang(Lang::Ukrainian)
                .cardinal()
                .to_words(),
            Ok(String::from("вісімнадцять мільйонів"))
        );
        assert_eq!(
            Num2Words::new(1000)
                .lang(Lang::Ukrainian)
                .cardinal()
                .to_words(),
            Ok(String::from("одна тисяча"))
        );
        assert_eq!(
            Num2Words::new(-1024)
                .lang(Lang::Ukrainian)
                .prefer("р")
                .cardinal()
                .to_words(),
            Ok(String::from("мінус одної тисячі двадцяти чотирьох"))
        );
    }

    #[test]
    fn test_ordinal_num() {
        assert_eq!(
            Num2Words::new(0)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .to_words(),
            Ok(String::from("0-й"))
        );
        assert_eq!(
            Num2Words::new(23)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .prefer("ж")
                .prefer("орудний")
                .to_words(),
            Ok(String::from("23-ою"))
        );
        assert_eq!(
            Num2Words::new(1000)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .prefer("множина")
                .prefer("давальний")
                .to_words(),
            Ok(String::from("1000-м"))
        );
        assert_eq!(
            Num2Words::new(13)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .prefer("множина")
                .prefer("давальний")
                .to_words(),
            Ok(String::from("13-м"))
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
    fn test_declension_agreement() {
        assert_eq!(
            Ukrainian {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declension: Declension::Nominative
            }
            .agreement_with_units(0, 0),
            Ukrainian {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Plural,
                declension: Declension::Genitive
            },
            "failed agreement: 0"
        );
        assert_eq!(
            Ukrainian {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declension: Declension::Nominative
            }
            .agreement_with_units(0, 1),
            Ukrainian {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declension: Declension::Nominative
            },
            "failed agreement: 1"
        );
        assert_eq!(
            Ukrainian {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Singular,
                declension: Declension::Nominative
            }
            .agreement_with_units(8, 2),
            Ukrainian {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Plural,
                declension: Declension::Nominative
            },
            "failed agreement: 82"
        );
        assert_eq!(
            Ukrainian {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declension: Declension::Dative
            }
            .agreement_with_units(1, 1),
            Ukrainian {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Plural,
                declension: Declension::Dative
            },
            "failed agreement: 11"
        );
        assert_eq!(
            Ukrainian {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Singular,
                declension: Declension::Instrumental
            }
            .agreement_with_units(5, 4),
            Ukrainian {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Plural,
                declension: Declension::Instrumental
            },
            "failed agreement: 54"
        );
        assert_eq!(
            Ukrainian {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Singular,
                declension: Declension::Nominative
            }
            .agreement_with_units(1, 8),
            Ukrainian {
                gender: Gender::Masculine,
                number: GrammaticalNumber::Plural,
                declension: Declension::Genitive
            },
            "failed agreement: 18"
        );
        assert_eq!(
            Ukrainian {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Singular,
                declension: Declension::Nominative
            }
            .agreement_with_units(0, 1),
            Ukrainian {
                gender: Gender::Feminine,
                number: GrammaticalNumber::Singular,
                declension: Declension::Nominative
            },
            "failed agreement: 1 feminine"
        );
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(
            Num2Words::new(0).lang(Lang::Ukrainian).ordinal().to_words(),
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
            Num2Words::new(123456000)
                .lang(Lang::Ukrainian)
                .ordinal()
                .to_words(),
            Ok(String::from(
                "сто двадцять три мільйони чотирьохсотпʼятдесятишеститисячний"
            ))
        );
        assert_eq!(
            Num2Words::new(1000000)
                .lang(Lang::Ukrainian)
                .ordinal()
                .to_words(),
            Ok(String::from("мільйонний"))
        );
        assert_eq!(
            Num2Words::new(-10_000)
                .lang(Lang::Ukrainian)
                .prefer("ж")
                .ordinal()
                .to_words(),
            Err(Num2Err::NegativeOrdinal)
        );
    }

    #[test]
    fn test_currency() {
        assert_eq!(
            Num2Words::new(1000)
                .lang(Lang::Ukrainian)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("одна тисяча доларів"))
        );
        assert_eq!(
            Num2Words::new(0.01)
                .lang(Lang::Ukrainian)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("один цент"))
        );
        assert_eq!(
            Num2Words::new(333.02)
                .lang(Lang::Ukrainian)
                .currency(Currency::ILS)
                .to_words(),
            Ok(String::from("триста тридцять три нові шекелі дві агори"))
        );
        assert_eq!(
            Num2Words::new(934.42)
                .lang(Lang::Ukrainian)
                .currency(Currency::UAH)
                .prefer("орудний")
                .to_words(),
            Ok(String::from(
                "девʼятьмастами тридцятьма чотирма гривнями сорока двома копійками"
            ))
        );
    }

    #[test]
    fn test_year() {
        assert_eq!(
            Num2Words::new(1.1).lang(Lang::Ukrainian).year().to_words(),
            Err(num2words::Num2Err::FloatingYear)
        );
        assert_eq!(
            Num2Words::new(2023).lang(Lang::Ukrainian).year().to_words(),
            Ok(String::from("дві тисячі двадцять третій рік"))
        );
        assert_eq!(
            Num2Words::new(-67).lang(Lang::Ukrainian).year().to_words(),
            Ok(String::from("шістдесят сьомий рік до н.е."))
        );
    }

    #[test]
    fn test_float() {
        assert_eq!(
            Num2Words::new(1.1)
                .lang(Lang::Ukrainian)
                .cardinal()
                .to_words(),
            Ok(String::from("одна ціла одна десята")),
            "1.1 default"
        );
        assert_eq!(
            Num2Words::new(1.1)
                .lang(Lang::Ukrainian)
                .prefer("орудний")
                .prefer("жіночий")
                .cardinal()
                .to_words(),
            Ok(String::from("одною цілою одною десятою")),
            "1.1 f ins"
        );
        assert_eq!(
            Num2Words::new(-12.321)
                .lang(Lang::Ukrainian)
                .prefer("давальний")
                .prefer("множина")
                .cardinal()
                .to_words(),
            Ok(String::from(
                "мінус дванадцяти цілим трьомстам двадцяти одній тисячній"
            ))
        );
        assert_eq!(
            Ukrainian::new(
                Gender::Neuter,
                GrammaticalNumber::Singular,
                Declension::Accusative
            )
            .float_to_cardinal(BigFloat::from(973.0)),
            Ok(String::from("девʼятсот сімдесят три"))
        );
    }

    #[test]
    fn test_infinity() {
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Ukrainian)
                .prefer("д")
                .cardinal()
                .to_words(),
            Ok(String::from("нескінченності"))
        );
        assert_eq!(
            Num2Words::new(f64::NEG_INFINITY)
                .lang(Lang::Ukrainian)
                .prefer("о")
                .cardinal()
                .to_words(),
            Ok(String::from("мінус нескінченністю"))
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Ukrainian)
                .ordinal()
                .to_words(),
            Err(num2words::Num2Err::InfiniteOrdinal)
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Ukrainian)
                .ordinal_num()
                .to_words(),
            Err(num2words::Num2Err::InfiniteOrdinal)
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Ukrainian)
                .year()
                .to_words(),
            Err(num2words::Num2Err::InfiniteYear)
        );
        assert_eq!(
            Num2Words::new(f64::INFINITY)
                .lang(Lang::Ukrainian)
                .currency(Currency::DOLLAR)
                .to_words(),
            Ok(String::from("нескінченність доларів"))
        );
    }
}
