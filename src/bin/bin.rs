use ::num2words::{Currency, Lang, Num2Words, Output};
use std::env;
use std::str::FromStr;

const HELP: &str = r#"NAME:
    num2words - convert numbers into words

USAGE:
    num2words <number> [options]
    num2words --help

VERSION:
    v{{VERSION}}

COMMANDS:
GLOBAL OPTIONS:
    --lang value, -l value   set language (default: "en")
    --to output, -t output   set output (default: "cardinal")
    --help, -h               show help
    --version, -v            print the version

AVAILABLE LANGUAGES:
    en: English

AVAILABLE OUTPUTS:
    cardinal:      forty-two (42)
    ordinal:       forty-second (42)
    ordinal_num:   42nd (42)
    year:          nineteen oh-one (1901)
    currency:      forty-two dollars and one cent (42.01)

AVAILABLE CURRENCIES:
    AUD      australian dollar
    CAD      canadian dollar
    DOLLAR   dollar
    EUR      euro
    GBP      pound
    USD      US dollar"#;

fn help() {
    println!("{}", HELP.replace("{{VERSION}}", env!("CARGO_PKG_VERSION")))
}

fn parse_number_and_print(num: String, lang: Lang, output: Output, currency: Currency) {
    match num.parse::<i64>() {
        Ok(num) => match Num2Words::new(num)
            .lang(lang)
            .currency(currency)
            .output(output)
            .to_words()
        {
            Ok(words) => println!("{}", words),
            Err(err) => print_err(err),
        },
        _ => match num.parse::<f64>() {
            Ok(num) => match Num2Words::new(num)
                .lang(lang)
                .currency(currency)
                .output(output)
                .to_words()
            {
                Ok(words) => println!("{}", words),
                Err(err) => print_err(err),
            },
            _ => println!("Error: cannot parse number"),
        },
    }
}

fn handle_cmd(num: String, mut args: std::env::Args) {
    let mut lang = Lang::English; //String::from("en");
    let mut to = Output::Cardinal; //String::from("cardinal");
    let mut currency = Currency::DOLLAR;

    loop {
        match args.next() {
            Some(arg) => match arg.as_str() {
                "--lang" | "-l" => match args.next() {
                    Some(l) => {
                        if let Ok(v) = Lang::from_str(l.as_str()) {
                            lang = v
                        } else {
                            println!("Error: invalid language");
                            return;
                        }
                    }
                    None => {
                        help();
                        return;
                    }
                },
                "--to" | "-t" => match args.next() {
                    Some(t) => {
                        if let Ok(v) = Output::from_str(t.as_str()) {
                            to = v
                        } else if let Ok(v) = Currency::from_str(t.as_str()) {
                            to = Output::Currency;
                            currency = v;
                        } else {
                            println!("Error: invalid to tag");
                            return;
                        }
                    }
                    None => {
                        help();
                        return;
                    }
                },
                _ => continue,
            },
            None => break,
        }
    }

    parse_number_and_print(num, lang, to, currency)
}

fn print_err(err: num2words::Num2Err) {
    println!(
        "Error: {}",
        match err {
            num2words::Num2Err::CannotConvert => "cannot convert number",
            num2words::Num2Err::NegativeOrdinal => "cannot treat negative number as ordinal",
            num2words::Num2Err::FloatingOrdinal => "cannot treat float as ordinal",
            num2words::Num2Err::FloatingYear => "cannot treat float as year",
        }
    )
}

fn main() {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(num) => match num.as_str() {
            "--help" | "-h" => help(),
            "--version" | "-v" => println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            _ => handle_cmd(num, args),
        },
        None => help(),
    }
}
