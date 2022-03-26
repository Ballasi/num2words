use num2words::num2words;
use std::env;

const HELP: &str = r#"Usage: num2words <number> [--lang|-l lang] [--to|-t to]

For more information about all of the available languages and to
tags, please type the following:
    $ num2words --help lang
    $ num2words --help to"#;

const HELP_LANG: &str = r#"Available languages:
    * en: English

This list can be expanded! Do not hesitate to contribute!
https://github.com/Ballasi/num2words"#;

const HELP_TO: &str = r#"Available to tags:
    * cardinal
    * ordinal
    * ordinal_num
    * year
    * any currency

Currencies available:
    * AUD
    * CAD
    * DOLLAR (generic, non-localized dollar)
    * EUR
    * GBP
    * USD"#;

fn help() {
    println!("{}", HELP)
}

fn help_lang() {
    println!("{}", HELP_LANG)
}

fn help_to() {
    println!("{}", HELP_TO)
}

fn handle_help(mut args: std::env::Args) {
    match args.next() {
        Some(cmd) => match cmd.as_str() {
            "lang" => help_lang(),
            "to" => help_to(),
            _ => help(),
        },
        None => help(),
    }
}

fn handle_cmd(num: String, mut args: std::env::Args) {
    let mut lang = String::from("en");
    let mut to = String::from("cardinal");

    loop {
        match args.next() {
            Some(arg) => match arg.as_str() {
                "--lang" | "-l" => match args.next() {
                    Some(l) => lang = l,
                    None => {
                        help();
                        return;
                    }
                },
                "--to" | "-t" => match args.next() {
                    Some(t) => to = t,
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

    match num.parse::<i64>() {
        Ok(num) => match num2words!(num, lang = lang.as_str(), to = to.as_str()) {
            Ok(words) => println!("{}", words),
            Err(err) => print_err(err),
        },
        _ => match num.parse::<f64>() {
            Ok(num) => match num2words!(num, lang = lang.as_str(), to = to.as_str()) {
                Ok(words) => println!("{}", words),
                Err(err) => print_err(err),
            },
            _ => println!("Error: cannot parse number"),
        },
    }
}

fn print_err(err: num2words::Num2Err) {
    println!(
        "Error: {}",
        match err {
            num2words::Num2Err::CannotConvert => "cannot convert number",
            num2words::Num2Err::InvalidLang => "invalid language",
            num2words::Num2Err::InvalidToTag => "invalid to tag",
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
            "--help" | "-h" => handle_help(args),
            _ => handle_cmd(num, args),
        },
        None => help(),
    }
}
