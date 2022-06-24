use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Address;

/// A Wyre User object indicating approval status
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The Wyre id of the user
    pub id: String,
    /// The [approval status](ApprovalStatus) of the user
    pub status: UserStatus,
    /// The time the user was created at
    pub created_at: i64,
    /// The user's cryptocurrency deposit addresses
    pub deposit_addresses: DepositAddresses,
    /// The user's cryptocurrency total balances
    pub total_balances: UserBalances,
    /// The user's cryptocurrency available balances
    pub available_balances: UserBalances,
    /// The status of the user's fields
    pub fields: HashMap<UserFieldId, UserField>,
}

/// The field IDs your specific integration has to support depend on your
/// [integration type](https://docs.sendwyre.com/docs/users#integration-options).
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
pub enum UserFieldId {
    /// First name of the end user
    FirstName,
    ///Last name of the end user
    LastName,
    // It's not liking cell phones, but they're not required for the TRANSFER scope
    // /// The cellphone number of the end user
    // Cellphone,
    /// The email address of the end user
    Email,
    /// The residence address of the end user
    ResidenceAddress,
    /// The date of birth of the person, e.g. 1990-01-01
    DateOfBirth,
}

/// Object indicating the current status of a [user field](UserField)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserField {
    /// A representation of the underlying KYC data. Actual format depends on the type of field
    pub value: Option<UserFieldType>,
    /// One of:
    ///
    /// OPEN: The field is awaiting user data. This is the initial state before any information has
    /// been submitted, or if there were correctable problems with a previous submission.
    ///
    /// SUBMITTED: The field value has been uploaded and accepted.
    pub status: UserFieldStatus,
    /// A message indicating the nature of a correctable problem. Accompanied by an OPEN status.
    pub error: Option<String>,
}

/// The field type is hard-coded to each field ID and determines the JSON format and upfront
/// validation rules on it.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum UserFieldType {
    /// A basic string.
    String(Option<String>),

    // Because there is no tag provided, these all deserialize to a String. So, to avoid
    // confusion or unexpected behavior, we'll just keep everything simple
    // /// A full cellphone number including country code (e.g. `+15554445555`).
    // Cellphone(Option<String>),
    // /// A correctly formatted email address.
    // Email(Option<String>),
    // /// Specifies a particular day. Format is `YYYY-MM-DD` (e.g. `1992-12-15`).
    // Date(Option<String>),
    /// An address object.
    Address(Option<Address>),
}

/// Balances of user cryptocurrencies
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UserBalances {
    /// Bitcoin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btc: Option<i64>,
    /// Ethereum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth: Option<f64>,
}

/// Blockchain addresses for deposit
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DepositAddresses {
    /// Ethereum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth: Option<String>,
    /// Bitcoin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btc: Option<String>,
}

/// Values used for the `create_user` and `update_user` methods
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModifyUser {
    /// List of blockchains to connect the user to. Supported: `BTC`, `ETH`, `ALL`. Defaults to
    /// none (empty list).
    pub blockchains: Vec<String>,
    /// If true, returns immediately. This skips the default behavior of waiting up to 5 seconds
    /// for processing to complete, and so will always result in a PENDING user.
    pub immediate: bool,
    /// Maps field IDs to their respective values
    pub fields: HashMap<UserFieldId, UserFieldType>,
    /// Array of scopes to bias the view returned after the user is created. Only valid scope is
    /// currently [`TRANSFER`](UserScopes::Transfer)
    pub scopes: Vec<UserScope>,
}

/// The KYC status of a user
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    /// The User has been closed and may not transact.
    Closed,

    /// Waiting on action from you or the User. This is the initial
    /// state before all information has been submitted, or after any has
    /// failed to pass verifications.
    Open,

    /// Information has been fully submitted and is waiting on review from
    /// Wyre. The User cannot yet transact.
    Pending,

    /// Information has been reviewed and accepted by Wyre. The User is now
    /// approved to transact.
    ///
    /// The `UserStatus` being `Approved` does not mean the user is able to submit transactions.
    /// The user status is used for compliance reasons. The `status` field of all [`UserField`]s
    /// need to be [`Submitted`](UserFieldStatus) before a user is ready to transact.
    Approved,
}

/// The current approval status of a user field
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserFieldStatus {
    /// Waiting on data to be submitted to one or more fields.
    ///
    /// This is the initial state before any information has been submitted, or if there were
    /// correctable problems with a previous submission.
    Open,

    /// The field value has been uploaded and accepted.
    Submitted,
}

/// User scopes (currently only [`Transfer`](UserScopes::Transfer) is supported)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserScope {
    /// General access to the Transfers API. Access to this scope is required for all transfers.
    ///
    /// For Swaps, user's must have submitted and approved Name, Address, and Date of Birth.
    Transfer,
    /// Access to create payment methods (attach bank accounts) to Users
    ///
    /// For bank transfer onramps, full KYC including ID and bank account verification.
    ACH,
    /// Higher Limits Card Processing
    ///
    /// For higher limit card purchases, full KYC including ID verification.
    DebitCardL2,
}
impl ToString for UserScope {
    fn to_string(&self) -> String {
        match self {
            UserScope::Transfer => "TRANSFER",
            UserScope::ACH => "ACH",
            UserScope::DebitCardL2 => "DEBIT_CARD_L2",
        }
        .to_owned()
    }
}
