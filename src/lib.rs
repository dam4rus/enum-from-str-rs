use ::std::fmt::{Display, Formatter};
use ::std::error::Error;

type Result = ::std::fmt::Result;

/// Error indicating that a string cannot be converted to an enum type
#[derive(Debug, Clone, Copy)]
pub struct ParseEnumVariantError {
    enum_name: &'static str,
}

impl ParseEnumVariantError {
    pub fn new(enum_name: &'static str) -> Self {
        Self { enum_name }
    }
}

impl Display for ParseEnumVariantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Cannot convert string to {}", self.enum_name)
    }
}

impl Error for ParseEnumVariantError {
    fn description(&self) -> &str {
        "Cannot convert string to enum"
    }
}

#[cfg(test)]
mod test {
    use super::ParseEnumVariantError;
    use enum_from_str_derive::FromStr;

    #[derive(FromStr)]
    enum Test {
        #[from_str="foo"]
        Foo,
        Bar,
    }

    #[test]
    pub fn test_enum() {
        "foo".parse::<Test>().unwrap();
        "Bar".parse::<Test>().unwrap();
    }
}