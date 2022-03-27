use std::str::FromStr;

/// Type of the output `num2words` give
pub enum Output {
    /// Number in cardinal form, e.g., `forty-two`
    Cardinal,
    /// Number in currency form, e.g., `forty-two dollars`
    Currency,
    /// Number in ordinal form, e.g., `forty-second`
    Ordinal,
    /// Number in ordinal form written in number, e.g., `42nd`
    OrdinalNum,
    /// Number in year form, e.g., `nineteen oh-one`
    Year,
}

impl FromStr for Output {
    type Err = ();

    /// Parses a string to return a value of this type
    ///
    ///
    /// | &str          | Output               |
    /// | ------------- | -------------------- |
    /// | `cardinal`    | `Output::Cardinal`   |
    /// | `currency`    | `Output::Currency`   |
    /// | `ordinal`     | `Output::Ordinal`    |
    /// | `ordinal_num` | `Output::OrdinalNum` |
    /// | `year`        | `Output::Year`       |
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
