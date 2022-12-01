//! This module corresponds to the [Accounts API](https://docs.sendwyre.com/docs/account-resource)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::{Address, Amount, Currency};
use crate::payment_method::PaymentMethod;
use crate::DocumentType;

/// See [Get Master Account - Result Format](https://docs.sendwyre.com/docs/get-master-account#result-format)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct MasterAccount {
    pub id: String,
    pub srn: String,
    pub created_at: u64,
    pub updated_at: Option<u64>,
    pub deleted_at: Option<u64>,
    pub disabled_at: Option<u64>,
    pub locked_at: Option<u64>,
    // pub locked_reason: ???,
    pub under_review_at: Option<u64>,
    pub in_review_at: Option<u64>,
    pub compliance_approved_at: Option<u64>,
    pub status: AccountStatus,
    // pub stripe_account_id: ???,
    pub profile: MasterAccountProfile,
    pub payment_methods: Vec<PaymentMethod>,
    // pub identities: Vec<???>,
    pub deposit_addresses: HashMap<Currency, String>, // currency => adddress
    // pub ledgers: Vec<???>,
    // pub documents: Vec<???>,
    // pub srn_limits: Vec<???>,
    // pub cellphone: ???,
    pub pusher_channel: String,
    pub email: String,
    // pub session: ???,
    // pub loginAt: ???,
    // pub lastLoginIp: ???,
    // pub lastLoginLocation: ???,
    // pub loc: ???,
    // pub email_identity: ???,
    // pub total_balances: ???,
    // pub available_balances: ???,
    pub verified: bool,
    #[serde(rename = "type")]
    pub kind: String,
}

/// See [Get Master Account - Result Format](https://docs.sendwyre.com/docs/get-master-account#result-format)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct MasterAccountProfile {
    pub first_name: String,
    pub last_name: String,
    pub language: String,
    pub address: Address,
    pub business_account: bool,
    // pub tax_id: ???,
    // pub doing_business_as: ???,
    // pub website: ???,
    // pub partner_link: ???,
    // pub ssn: ???,
    // pub date_of_birth: ???,
    // pub notify_email: true,
    pub notify_cellphone: bool,
    // pub notify_apns_device: ???,
    pub onboarding_dashboard_completed: bool,
    pub display_currency: String,
    // pub cpf_number: ???,
    #[serde(rename = "type")]
    pub kind: String,
    pub vertical: String,
    // pub ethereum_verification_address: ???,
    // pub company_title: ???,
    // pub partner_display_name: ???,
    // pub company_name: ???,
    // pub company_registration_number: ???,
    // pub occupation: ???,
    // pub purpose_of_account: ???,
    pub country: String,
    // pub name: ???
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct Account {
    pub id: String,
    pub status: AccountStatus,
    #[serde(rename = "type")]
    pub kind: AccountType,
    pub country: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub deposit_addresses: HashMap<Currency, String>, // currency => address
    pub total_balances: HashMap<Currency, Amount>,
    pub available_balances: HashMap<Currency, Amount>,
    pub profile_fields: Vec<ProfileField>,
}

/// See [Create Account - Parameters](https://docs.sendwyre.com/docs/create-account#parameters).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccount {
    /// The type of account, currently `INDIVIDUAL` is the only supported value.
    #[serde(rename = "type")]
    pub kind: AccountType,

    /// The country of the account holder. For individuals this is the country
    /// of residence (currently we only support US accounts).
    pub country: String,

    /// An array of the Fields submitted at the time of Account creation. You
    /// can submit as many or as few fields as you need at the time of account
    /// creation.
    pub profile_fields: Vec<CreateProfileField>,

    /// Supply your own Account ID when creating noncustodial accounts. This
    /// field is used to track which account referred the new account into our
    /// system.
    pub referrer_account_id: Option<String>,

    /// When true, the newly created account will be a custodial subaccount
    /// owner by the caller. Otherwise, the account will be a standalone
    /// non-custodial account. (Defaults to `true`)
    pub subaccount: Option<bool>,

    /// if true prevents all outbound emails to the account. This includes all
    /// communications listed [here](https://docs.sendwyre.com/docs/customer-emails-messaging)
    /// (defaults to `false`).
    pub disable_email: Option<bool>,
}

/// See [Update Account - Parameters](https://docs.sendwyre.com/docs/submit-account-info#parameters).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccount {
    /// An array containing objects of fieldIds and values.
    pub profile_fields: Vec<CreateProfileField>,
}

/// See [Account Status](https://docs.sendwyre.com/docs/account-resource#account-status)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    /// Waiting on action from you or the Account holder. This is the initial
    /// state before all information has been submitted, or after any has
    /// failed to pass verifications.
    Open,

    /// Information has been fully submitted and is waiting on review from
    /// Wyre. The Account cannot yet transact.
    Pending,

    /// Information has been reviewed and accepted by Wyre. The Account is now
    /// approved to transact.    
    Approved,

    /// The Account has been closed and may not transact.
    Closed,
}

/// See [Account Types](https://docs.sendwyre.com/docs/account-resource#account-types)
///
/// Currently only `INDIVIDUAL` accounts are supported through our API.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    /// Used to register a personal account.
    Individual,

    /// Used to register a business entity account.
    Business,
}

/// See [Account Fields](https://docs.sendwyre.com/docs/account-resource#account-fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileField {
    /// The specific datapoint encapsulated by the field.
    pub field_id: ProfileFieldId,

    /// A representation of the underlying KYC data.
    #[serde(flatten)]
    pub value: ProfileFieldType,

    /// A message to the accountholder regarding the field.
    pub note: Option<String>,

    /// When the field was last updated.
    pub updated_t: Option<u64>,

    /// The current verification status of the field.
    pub status: ProfileFieldStatus,
}

/// See [Account Fields](https://docs.sendwyre.com/docs/account-resource#account-fields)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileField {
    /// The specific datapoint encapsulated by the field.
    pub field_id: ProfileFieldId,

    /// A representation of the underlying KYC data.
    #[serde(flatten)]
    pub value: ProfileFieldType,
}

/// See [Field Statuses](https://docs.sendwyre.com/docs/account-resource#field-statuses)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProfileFieldStatus {
    /// Waiting on action from you or the Account holder. This is the initial
    /// state before any information has been submitted, or after it has failed
    /// to pass verifications.
    Open,

    /// Information has been fully submitted and is waiting on review from Wyre.
    Pending,

    /// Information has been reviewed and accepted by Wyre.
    Approved,
}

/// See [Field Types](https://docs.sendwyre.com/docs/account-resource#field-types)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    rename_all = "SCREAMING_SNAKE_CASE",
    tag = "fieldType",
    content = "value"
)]
pub enum ProfileFieldType {
    /// A basic string.
    String(Option<String>),

    /// A full cellphone number including country code (e.g. `+15554445555`).
    Cellphone(Option<String>),

    /// A correctly formatted email address.
    Email(Option<String>),

    /// An address object.
    Address(Option<Address>),

    /// Specifies a particular day. Format is `YYYY-MM-DD` (e.g. `1992-12-15`).
    Date(Option<String>),

    /// A binary document. Documents should be uploaded via the [Upload Document](https://docs.sendwyre.com/v3/docs/upload-document)
    /// API. The value returned by the API will be an array of document IDs
    /// associated with that field.
    ///
    /// All document IDs uploaded will be retained by the field until review.
    /// During review, invalid/unacceptable document IDs will be deleted. Once
    /// the field status is `APPROVED`, then only the actually approved
    /// document IDs shall remain.
    Document(Vec<String>),

    /// A Payment Method record. Payment methods are created using our Payment
    /// Method APIs. The value returned by the API will be a string containing
    /// the Payment Method ID associated with that field.
    PaymentMethod(Option<String>),
}

/// See [Field IDs](https://docs.sendwyre.com/v3/docs/account-resource#field-ids)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProfileFieldId {
    /// The full legal name of the account holder (corresponding value must be
    /// [`ProfileFieldType::String`]).
    IndividualLegalName,

    /// The cellphone number of the account holder (corresponding value must be
    /// [`ProfileFieldType::Cellphone`]).
    IndividualCellphoneNumber,

    /// The email address of the account holder (corresponding value must be
    /// [`ProfileFieldType::Email`]).
    IndividualEmail,

    /// The residence address of the account holder (corresponding value must
    /// be [`ProfileFieldType::Address`]).
    IndividualResidenceAddress,

    /// A scan or photo of a drivers license or passport of the account holder
    /// (corresponding value must be [`ProfileFieldType::Document`]).
    IndividualGovernmentId,

    /// The account holder's date of birth (corresponding value must be
    /// [`ProfileFieldType::Date`]).
    IndividualDateOfBirth,

    /// The account holder's social security number (corresponding value must
    /// be [`ProfileFieldType::String`]).
    IndividualSsn,

    /// A payment method that the account holder owns (corresponding value must
    /// be [`ProfileFieldType::PaymentMethod`]).
    IndividualSourceOfFunds,

    /// A utility bill or bank statement. `individualProofOfAddress` will start
    /// in the `PENDING` state as we will attempt to use the
    /// `individualSourceOfFunds` to fill this requirement. If we aren’t able
    /// to verify the address from the user’s bank account we will mark the
    /// `individualProofOfAddress` field as `OPEN`. When this field is `OPEN`,
    /// the user should be displayed the note on the field and prompted to
    /// upload a document (corresponding value must be [`ProfileFieldType::Document`]).
    IndividualProofOfAddress,

    /// Used to verify a payment method after our compliance team has requested
    /// further verification (corresponding value must be [`ProfileFieldType::Document`]).
    IndividualAchAuthorizationForm,
}

impl std::fmt::Display for ProfileFieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ProfileFieldId::*;

        match self {
            IndividualLegalName => write!(f, "individualLegalName"),
            IndividualCellphoneNumber => write!(f, "individualCellphoneNumber"),
            IndividualEmail => write!(f, "individualEmail"),
            IndividualResidenceAddress => write!(f, "individualResidenceAddress"),
            IndividualGovernmentId => write!(f, "individualGovernmentId"),
            IndividualDateOfBirth => write!(f, "individualDateOfBirth"),
            IndividualSsn => write!(f, "individualSsn"),
            IndividualSourceOfFunds => write!(f, "individualSourceOfFunds"),
            IndividualProofOfAddress => write!(f, "individualProofOfAddress"),
            IndividualAchAuthorizationForm => write!(f, "individualAchAuthorizationForm"),
        }
    }
}

/// See [Upload Document - Parameters](https://docs.sendwyre.com/docs/upload-document#parameters)
#[derive(Debug, Clone)]
pub struct UploadDocument<D> {
    /// the field id that the uploaded document is associated with. See list of
    /// valid values here. Only values of type `DOCUMENT` will accept a
    /// document upload.
    pub field_id: ProfileFieldId,

    /// For `individualGovernmentId`s, you could specify the document type.
    /// Possible values are `GOVT_ID`, `DRIVING_LICENSE`, `PASSPORT_CARD` and
    /// `PASSPORT`.
    pub document_type: Option<DocumentType>,

    /// For `individualGovernmentId`s, you could specify the document sub type.
    /// Possible values are `FRONT` and `BACK`. For `individualGovernmentId`,
    /// it is required to upload both `FRONT` and `BACK` for `GOVT_ID`,
    /// `DRIVING_LICENSE` and `PASSPORT_CARD`.
    pub document_sub_type: Option<DocumentSubType>,

    /// The document to upload (maximum file upload size is 7.75MB).
    pub document: D,

    /// The content type of the document. See [Supported Document Types](https://docs.sendwyre.com/docs/upload-document#section-supported-document-types).
    ///
    /// - application/pdf
    /// - image/jpeg
    /// - image/png
    /// - application/msword
    /// - application/vnd.openxmlformats-officedocument.wordprocessingml.document
    pub content_type: String,
}

/// See [`UploadDocument`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum DocumentSubType {
    Front,
    Back,
}
