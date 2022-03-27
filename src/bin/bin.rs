use ::num2words::{Currency, Lang, Num2Words};
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

fn num2words(num: String) -> Option<Num2Words> {
    if let Ok(num) = num.parse::<i64>() {
        Some(Num2Words::new(num))
    } else if let Ok(num) = num.parse::<f64>() {
        Some(Num2Words::new(num))
    } else {
        None
    }
}

fn handle_cmd(n: String, mut args: std::env::Args) {
    if let Some(mut num) = num2words(n) {
        loop {
            match args.next() {
                Some(arg) => match arg.as_str() {
                    "--lang" | "-l" => match args.next() {
                        Some(l) => {
                            if let Ok(v) = Lang::from_str(l.as_str()) {
                                num = num.lang(v);
                            } else {
                                eprintln!("Error: invalid language");
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
                            if let Ok(v) = Currency::from_str(t.as_str()) {
                                num = num.currency(v);
                            } else {
                                match t.as_str() {
                                    "cardinal" => {
                                        num = num.cardinal();
                                    }
                                    "ordinal" => {
                                        num = num.ordinal();
                                    }
                                    "ordinal_num" => {
                                        num = num.ordinal_num();
                                    }
                                    "year" => {
                                        num = num.year();
                                    }
                                    _ => {
                                        eprintln!("Error: invalid to tag");
                                        return;
                                    }
                                }
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

        match num.to_words() {
            Ok(v) => println!("{}", v),
            Err(err) => eprintln!("Error: {}", err.to_string()),
        }
    } else {
        eprintln!("Error: cannot parse number");
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    match args.next() {
        Some(num) => match num.as_str() {
            "--help" | "-h" => help(),
            "--version" | "-v" => {
                println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            }
            _ => handle_cmd(num, args),
        },
        None => help(),
    }
}
