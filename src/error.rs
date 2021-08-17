use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

use reqwest::Error as ReqwestError;
use serde::Deserialize;

/// Represents an error that can occur when making an API request.
#[derive(Debug)]
pub enum Error {
    /// An error that was reported by the Wyre API
    Api(ApiError),

    /// An error that ocurred during transport
    Transport(ReqwestError),
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Error::Transport(error)
    }
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

/// See [Errors](https://docs.sendwyre.com/docs/errors)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    /// A unique identifier for this exception. This is very helpful when
    /// contacting support.
    pub exception_id: String,

    /// The category of the exception. See [`exception`] module.
    #[serde(rename = "type")]
    pub kind: String,

    /// A more granular specification than `type`.
    pub error_code: Option<String>,

    /// A human-friendly description of the problem.
    pub message: String,

    /// Indicates the language of the exception message.
    pub language: String,

    /// In rare cases, an exception may signal `true` here to indicate a
    /// transient problem. This means the request can be safely re-attempted.
    pub transient: bool,
}

/// See [Error Types](https://docs.sendwyre.com/docs/errors#error-types)
pub mod exception {
    /// The action failed due to problems with the request.
    pub const VALIDATION: &str = "ValidationException";

    /// A value was invalid.
    pub const INVALID_VALUE: &str = "InvalidValueException";

    /// A required field was missing.
    pub const FIELD_REQUIRED: &str = "FieldRequiredException";

    /// You requested the use of more funds in the specified currency than were
    /// available.
    pub const INSUFFICIENT_FUNDS: &str = "InsufficientFundsException";

    /// You lack sufficient privilege to perform the requested action.
    pub const ACCESS_DENIED: &str = "AccessDeniedException";

    /// There was a problem completing your transfer request.
    pub const TRANSFER: &str = "TransferException";

    /// An MFA action is required to complete the request. In general you
    /// should not get this exception while using API keys.
    pub const MFA_REQUIRED: &str = "MFARequiredException";

    /// Please contact us at support@sendwyre.com to resolve this!
    pub const CUSTOMER_SUPPORT: &str = "CustomerSupportException";

    /// You referenced something that could not be located.
    pub const NOT_FOUND: &str = "NotFoundException";

    /// Your requests have exceeded your usage restrictions. Please contact us
    /// if you need this increased.
    pub const RATE_LIMIT: &str = "RateLimitException";

    /// The account has had a locked placed on it for potential fraud reasons.
    /// The customer should [contact Wyre support](https://support.sendwyre.com/hc/en-us)
    /// for follow-up.
    pub const ACCOUNT_LOCKED: &str = "AccountLockedException";

    /// The account or IP has been blocked due to detected malicious behavior.
    pub const LOCKOUT: &str = "LockoutException";

    /// A problem with our services internally. This should rarely happen.
    pub const UNKNOWN: &str = "UnknownException";

    /// The account has not been approved and cannot submit transactions.
    pub const ACCOUNT_HAS_NOT_BEEN_APPROVED_TO_TRANSACT: &str =
        "AccoutHasNotBeenApprovedToTransactException";
}
