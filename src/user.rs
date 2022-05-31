use serde::{Deserialize, Serialize};

use crate::Address;

/// A Wyre User object indicating approval status
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The Wyre id of the user
    pub id: String,
    /// The [approval status](ApprovalStatus) of the user
    pub status: ApprovalStatus,
    /// The time the user was created at
    pub created_at: i64,
    /// The user's cryptocurrency deposit addresses
    pub deposit_addresses: DepositAddresses,
    /// The user's cryptocurrency total balances
    pub total_balances: UserBalances,
    /// The user's cryptocurrency available balances
    pub available_balances: UserBalances,
    /// The status of the user's fields
    pub fields: UserFields,
}

/// Indication of status for the submitted user fields
// What happens if a field is omitted? Does Wyre return an error, null, or empty string?
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserFields {
    /// The status of the firstName field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Status<String>>,
    /// The status of the lastName field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Status<String>>,
    /// The status of the residenceAddress field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_address: Option<Status<Address>>,
}

/// Generic status object
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status<T> {
    /// A representation of the underlying KYC data. Actual format depends on the type of field
    pub value: T,
    /// One of:
    ///
    /// OPEN: The field is awaiting user data. This is the initial state before any information has been submitted, or if there were correctable problems with a previous submission.
    ///
    /// SUBMITTED: The field value has been uploaded and accepted.
    // I don't know why they have submitted and approved as separate things, or why they're used differently, it just is
    pub status: ApprovalStatus,
    /// A message indicating the nature of a correctable problem. Accompanied by an OPEN status.
    pub error: Option<String>,
}

/// Balances of user cryptocurrencies
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModifyUser {
    /// List of blockchains to connect the user to. Supported: `BTC`, `ETH`, `ALL`. Defaults to none
    /// (empty list).
    pub blockchains: Vec<String>,
    /// If true, returns immediately. This skips the default behavior of waiting up to 5 seconds
    /// for processing to complete, and so will always result in a PENDING user.
    pub immediate: bool,
    /// Maps field IDs to their respective values
    pub fields: ModifyUserFields,
    /// Array of scopes to bias the view returned after the user is created. Only valid scope is currently [`TRANSFER`](UserScopes::Transfer)
    pub scopes: UserScopes,
}

/// User fields object, used in [`ModifyUser`]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModifyUserFields {
    /// The user's first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The user's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The users's address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_address: Option<Address>,
}

/// The current approval status of a user field
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApprovalStatus {
    /// Waiting on data to be submitted to one or more fields.
    ///
    /// This is the initial state before any information has been submitted, or if there were correctable problems with a previous submission.
    Open,
    /// Information has been fully submitted and is waiting on review from Wyre.
    Pending,
    /// The Account has been closed and may not transact. Customer service followup is necessary for further actions.
    Closed,
    /// Information has been reviewed and accepted by Wyre
    Approved,
    /// The field value has been uploaded and accepted.
    Submitted,
}

/// User scopes (currently only [`Transfer`](UserScopes::Transfer) is supported)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserScopes {
    /// General access to the Transfers API. Access to this scope is required for all transfers.
    Transfer,
    /// Access to create payment methods (attach bank accounts) to Users
    ACH,
    /// Higher Limits Card Processing
    DebitCardL2,
}
