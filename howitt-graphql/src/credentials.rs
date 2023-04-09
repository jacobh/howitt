#[derive(Debug, PartialEq, Eq)]
pub enum Credentials {
    Key(String),
}
impl Credentials {
    pub fn parse_auth_header_value(value: &str) -> Result<Credentials, ()> {
        if value.to_lowercase().starts_with("key ") {
            return Ok(Credentials::Key(value[4..].to_string()));
        }
        return Err(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Key Asdf1", Ok(Credentials::Key(String::from("Asdf1"))) ; "Parses API Key")]
    #[test_case("key Asdf1", Ok(Credentials::Key(String::from("Asdf1"))) ; "case insensitive")]
    #[test_case("asdf", Err(()) ; "Fails for missing prefix")]
    #[test_case("NotKey asdf", Err(()) ; "Fails for unknown prefix")]
    fn parse_auth_header_value_works(input: &'static str, expected: Result<Credentials, ()>) {
        let result = Credentials::parse_auth_header_value(input);
        assert_eq!(expected, result);
    }
}
