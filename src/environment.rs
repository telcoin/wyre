use std::convert::TryFrom;
use std::str::FromStr;

/// See [Production/Test Environments](https://docs.sendwyre.com/docs/productiontest-environments).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Environment {
    /// Uses TestNet for crypto integrations, Plaid sandbox, and fake PII.
    Test,

    /// Uses live funds, accounts, and integrations.
    Production,
}

impl Environment {
    /// The url used to access the API.
    #[must_use]
    pub fn api_url(&self) -> &str {
        match self {
            Environment::Test => "https://api.testwyre.com",
            Environment::Production => "https://api.sendwyre.com",
        }
    }
}

impl FromStr for Environment {
    type Err = EnvironmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "dev" | "development" | "test" => Ok(Environment::Test),
            "prod" | "production" => Ok(Environment::Production),
            val => Err(EnvironmentParseError(val.to_owned())),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = EnvironmentParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<'a> TryFrom<&'a str> for Environment {
    type Error = EnvironmentParseError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Could not parse an environment, contains the original string.
#[derive(Debug)]
pub struct EnvironmentParseError(pub String);
