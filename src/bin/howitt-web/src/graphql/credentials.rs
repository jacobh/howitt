#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Couldn't parse auth header")]
pub struct AuthHeaderParseError {}

#[derive(Debug, PartialEq, Eq)]
pub enum Credentials {
    Key(String),
    BearerToken(String),
}
impl Credentials {
    pub fn parse_auth_header_value(value: &str) -> Result<Credentials, AuthHeaderParseError> {
        if value.to_lowercase().starts_with("key ") {
            Ok(Credentials::Key(value[4..].to_string()))
        } else if value.to_lowercase().starts_with("bearer ") {
            Ok(Credentials::BearerToken(value[7..].to_string()))
        } else {
            Err(AuthHeaderParseError {})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Key Asdf1", Ok(Credentials::Key(String::from("Asdf1"))) ; "Parses API Key")]
    #[test_case("Bearer qweRty23", Ok(Credentials::BearerToken(String::from("qweRty23"))) ; "Parses Token")]
    #[test_case("key Asdf1", Ok(Credentials::Key(String::from("Asdf1"))) ; "case insensitive")]
    #[test_case("asdf", Err(AuthHeaderParseError {}) ; "Fails for missing prefix")]
    #[test_case("NotKey asdf", Err(AuthHeaderParseError {}) ; "Fails for unknown prefix")]
    fn parse_auth_header_value_works(
        input: &'static str,
        expected: Result<Credentials, AuthHeaderParseError>,
    ) {
        let result = Credentials::parse_auth_header_value(input);
        assert_eq!(expected, result);
    }
}
