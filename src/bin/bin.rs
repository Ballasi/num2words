use ::num2words::{Currency, Lang, Num2Words};
use std::env;
use std::str::FromStr;

const HELP: &str = r#"NAME:
    num2words - convert numbers into words

USAGE:
    num2words <number> [options]
    num2words --help

VERSION:
    {{VERSION}}

COMMANDS:
GLOBAL OPTIONS:
    -l, --lang [value]          set language (default: "en")
    -t, --to [output]           set output (default: "cardinal")
    -p, --prefer [preference]   add a language preference (default: none)
    -h, --help                  show help
    -v, --version               print the version

AVAILABLE LANGUAGES:
    en:      English
    fr:      French (France and Canada)
    fr_BE:   French (Belgium and the Democratic Republic of the Congo)
    fr_CH:   French (Swiss Confederation and Aosta Valley)
    uk:      Ukrainian

AVAILABLE OUTPUTS:
    cardinal:      forty-two (42)
    ordinal:       forty-second (42)
    ordinal_num:   42nd (42)
    year:          nineteen oh-one (1901)
    currency:      forty-two dollars and one cent (42.01)

AVAILABLE CURRENCIES:
    ISO 4217 code      - USD, EUR, GBP, etc.
    generic currencies - DINAR, DOLLAR, PESO, RIYAL"#;

fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let mut words = vec![];

    for num in version.split('.') {
        if let Ok(i) = String::from(num).parse::<i64>() {
            if let Ok(word) = Num2Words::new(i).prefer("oh").to_words() {
                words.push(word);
            }
        }
    }

    format!("v{} (version {})", version, words.join(" point "))
}

fn help() {
    println!("{}", HELP.replace("{{VERSION}}", get_version().as_str()))
}

fn handle_cmd(n: String, mut args: std::env::Args) {
    if let Some(mut num) = Num2Words::parse(&n) {
        while let Some(arg) = args.next() {
            match arg.as_str() {
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
                "--prefer" | "-p" => match args.next() {
                    Some(p) => num = num.prefer(p),
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
            }
        }

        match num.to_words() {
            Ok(v) => println!("{}", v),
            Err(err) => eprintln!("Error: {}", err),
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
                println!("{} {}", env!("CARGO_PKG_NAME"), get_version())
            }
            _ => handle_cmd(num, args),
        },
        None => help(),
    }
}
