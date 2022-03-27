use std::str::FromStr;

pub enum Output {
    Cardinal,
    Currency,
    Ordinal,
    OrdinalNum,
    Year,
}

impl FromStr for Output {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "cardinal" => Ok(Output::Cardinal),
            "currency" => Ok(Output::Currency),
            "ordinal" => Ok(Output::Ordinal),
            "ordinal_num" => Ok(Output::OrdinalNum),
            "year" => Ok(Output::Year),
            _ => Err(()),
        }
    }
}
