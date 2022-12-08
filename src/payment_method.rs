//! This module corresponds to the [Payment Method API](https://docs.sendwyre.com/docs/payment-method-overview)

use serde::{Deserialize, Serialize};

use crate::{common::Currency, SystemResourceName};

/// See [Payment Method Statuses](https://docs.sendwyre.com/docs/payment-method-overview#payment-method-statuses).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMethodStatus {
    /// Payment Method has not been activated and is PENDING review on Wyre's
    /// side. No user action is required.
    Pending,

    /// Payment Method requires additional information from the user before
    /// being useful. The case where you would see this is on WIRE_TRANSFER
    /// payment methods when the bank statement is still required.
    AwaitingFollowup,

    /// Payment Method is active and ready for use.
    Active,

    /// Payment Method has been rejected by Wyre and cannot be used.
    Rejected,
}

/// See [Payment Method Types](https://docs.sendwyre.com/docs/payment-method-overview#payment-method-types).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMethodType {
    /// Wire transfer (LinkType: INTERNATIONAL_TRANSFER)
    WireTransfer,

    /// Transfer using the local banking system. In the case of US, this would
    /// be an ACH payment. (LinkType: LOCAL_TRANSFER)
    LocalTransfer,
}

/// See [ACH - Create Payment Method - Parameters](https://docs.sendwyre.com/docs/ach-create-payment-method-processor-token-model#parameters)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAchPaymentMethod {
    /// Token from Plaid via the `/processor/token/create` endpoint
    ///
    /// See [Create a Plaid Processor Token](https://docs.sendwyre.com/docs/ach-create-payment-method-processor-token-model#create-a-plaid-processor-token).
    pub plaid_processor_token: String,

    /// The only supported type is `LOCAL_TRANSFER`.
    pub payment_method_type: PaymentMethodType,

    /// The only supported country is `US`.
    pub country: AchPaymentMethodCountry,
}

/// See [ACH - Create Payment Method](https://docs.sendwyre.com/docs/ach-create-payment-method-processor-token-model)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchPaymentMethodCountry {
    /// United States
    US,
}

/// See [ACH - Create Payment Method - Result Format](https://docs.sendwyre.com/docs/ach-create-payment-method-processor-token-model#result-format)
/// and [Create Payment Method - Result Format](https://docs.sendwyre.com/docs/create-payment-method#result-format).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct PaymentMethod {
    pub id: String,
    pub owner: SystemResourceName,
    pub created_at: u64,
    pub name: String,
    pub default_currency: Currency,
    pub status: PaymentMethodStatus,
    // pub status_message: ???,
    // pub waiting_prompts: Vec<???>,
    pub link_type: String,
    pub beneficiary_type: String,
    pub supports_deposits: Option<bool>,
    // pub name_on_method: ???,
    pub last4_digits: String,
    pub brand: Option<String>,
    // pub expirationDisplay: ???,
    pub country_code: String,
    // pub nickname: ???,
    // pub rejectionMessage: ???,
    pub disabled: bool,
    pub supports_payment: bool,
    pub chargeable_currencies: Vec<Currency>,
    pub depositable_currencies: Vec<Currency>,
    // pub chargeFeeSchedule: ???,
    // pub depositFeeSchedule: ???,
    // pub minCharge: ???,
    // pub maxCharge: ???,
    // pub minDeposit: ???,
    // pub maxDeposit: ???,
    // pub documents: Vec<???>,
    pub srn: SystemResourceName,
}

/// See [List Payment Methods - Result Format](https://docs.sendwyre.com/docs/list-payment-methods#result-format)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct PaymentMethodList {
    pub data: Vec<PaymentMethod>,
    pub records_total: usize,
    pub position: usize,
    pub records_filtered: usize,
}
