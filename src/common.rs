use serde::{Deserialize, Serialize};

/// An address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// A valid street address
    pub street1: Option<String>,

    /// Additional street address
    pub street2: Option<String>,

    /// The city name
    pub city: Option<String>,

    /// A valid state code, it must be two uppercase letter. Ex CA
    pub state: Option<String>,

    /// A valid US zipcode
    pub postal_code: Option<String>,

    /// The country code (alpha-2 country code)
    pub country: Option<String>,
}
