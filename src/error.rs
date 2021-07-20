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

/// See [Errors](https://docs.sendwyre.com/docs/errors)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    /// A unique identifier for this exception. This is very helpful when
    /// contacting support.
    exception_id: String,

    /// The category of the exception.
    #[serde(rename = "type")]
    kind: ApiErrorKind,

    /// A more granular specification than `type`.
    error_code: Option<String>,

    /// A human-friendly description of the problem.
    message: String,

    /// Indicates the language of the exception message.
    language: String,

    /// In rare cases, an exception may signal `true` here to indicate a
    /// transient problem. This means the request can be safely re-attempted.
    transient: bool,
}

/// See [Error Types](https://docs.sendwyre.com/docs/errors#error-types)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum ApiErrorKind {
    /// The action failed due to problems with the request.
    ValidationException,

    /// A value was invalid.
    InvalidValueException,

    /// A required field was missing.
    FieldRequiredException,

    /// You requested the use of more funds in the specified currency than were
    /// available.
    InsufficientFundsException,

    /// You lack sufficient privilege to perform the requested action.
    AccessDeniedException,

    /// There was a problem completing your transfer request.
    TransferException,

    /// An MFA action is required to complete the request. In general you
    /// should not get this exception while using API keys.
    MFARequiredException,

    /// Please contact us at support@sendwyre.com to resolve this!
    CustomerSupportException,

    /// You referenced something that could not be located.
    NotFoundException,

    /// Your requests have exceeded your usage restrictions. Please contact us
    /// if you need this increased.
    RateLimitException,

    /// The account has had a locked placed on it for potential fraud reasons.
    /// The customer should [contact Wyre support](https://support.sendwyre.com/hc/en-us)
    /// for follow-up.
    AccountLockedException,

    /// The account or IP has been blocked due to detected malicious behavior.
    LockoutException,

    /// A problem with our services internally. This should rarely happen.
    UnknownException,
}
