//! This module corresponds to the [Transfers and Exchanges API](https://docs.sendwyre.com/docs/transfer-resources)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::{Amount, Currency};

/// See [Create Transfer - Parameters](https://docs.sendwyre.com/docs/create-transfer#parameters)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransfer {
    /// An SRN representing an account that the funds will be retrieved from.
    pub source: String,

    /// The amount to withdrawal from the source, in units of `sourceCurrency`.
    /// Only include `sourceAmount` OR `destAmount`, not both.
    pub source_amount: Option<Amount>,

    /// The currency (ISO 3166-1 alpha-3) to withdrawal from the source wallet.
    pub source_currency: Currency,

    /// An email address, cellphone number, digital currency address or bank
    /// account to send the digital currency to. For bitcoin address use
    /// "bitcoin:[address]". Note: cellphone numbers are assumed to be a US
    /// number, for international numbers include a '+' and the country code as
    /// the prefix.
    pub dest: String,

    /// Specifies the total amount of currency to deposit (as defined in
    /// `depositCurrency`). Only include `sourceAmount` OR `destAmount`, not
    /// both.
    pub dest_amount: Option<Amount>,

    /// The currency (ISO 3166-1 alpha-3) to deposit. if not provided, the
    /// deposit will be the same as the withdrawal currency (no exchange
    /// performed).
    pub dest_currency: Option<Currency>,

    /// An optional user visible message to be sent with the transaction.
    pub message: Option<String>,

    /// An optional url that Wyre will POST a status callback to (see [Callbacks](https://docs.sendwyre.com/v3/docs/subscribe-webhook)
    /// for more information).
    pub notify_url: Option<String>,

    /// An optional parameter to automatically confirm the transfer order.
    pub auto_confirm: Option<bool>,

    /// An optional custom ID to tag the transfer.
    pub custom_id: Option<String>,

    /// When true, the amount indicated (source or dest) will be treated as
    /// already including the fees.
    pub amount_includes_fees: Option<bool>,

    /// Creates a quote transfer object, but does not execute a real transfer.
    pub preview: Option<bool>,

    /// When true, disables outbound emails/messages to the destination.
    pub mute_messages: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct Transfer {
    pub id: String,
    pub owner: String,
    pub source: String,
    pub source_amount: Amount,
    pub source_currency: Currency,
    pub dest: String,
    pub dest_amount: Amount,
    pub dest_currency: Currency,
    pub status: TransferStatus,
    // pub status_histories: ???,
    pub pending_sub_status: Option<String>,
    // pub failure_reason: ???,
    // pub reversal_reason: ???,
    // pub reversing_sub_status: ???,
    pub completed_at: Option<u64>,
    pub updated_at: Option<u64>,
    pub cancelled_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub exchange_rate: Option<f32>,
    pub fees: HashMap<Currency, Amount>,
    pub total_fees: f32,
    // pub blockchain_tx: ???,
    pub message: Option<String>,
    pub custom_id: Option<String>,
}

/// See [Transfer Lifecycle](https://docs.sendwyre.com/docs/transfer-resources#transfer-lifecycle)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransferStatus {
    /// A preview transfer. These transfers cannot be confirmed and funds will
    /// never move on them. They're created by specifying the 'preview=true'
    /// parameter at time of transfer creation.
    Preview,

    /// A transfer with a valid quote. This is the default state for newly
    /// created transfers. These transfers must be confirmed before they're
    /// executed. Transfers will wait `UNCONFIRMED` for some about of time,
    /// after which if they are sill `UNCONFIRMED` they will transition to
    /// `EXPIRED`.
    Unconfirmed,

    /// A transfer in the pending state means we're working on moving the money
    /// to its destination. (It does not require any further action from your
    /// side).
    Pending,

    /// Once a transfer is fully executed and the funds have been confirmed at
    /// the destination its status will change to `COMPLETED`.
    Completed,

    /// Any `UNCONFIRMED` transfer that is not confirmed inside their 30-second
    /// confirmation window will transition to `EXPIRED`.
    Expired,

    /// If a transfer cannot be completed for any reason its status will change
    /// to `FAILED`. If there's anything we can do to make sure the transfer
    /// goes through we will reach out via support channels before failing a
    /// transfer.
    Failed,

    /// If a transfer is reversed at a later time for any reason its status
    /// will change to `REVERSED`. This happens with ACH payouts, for example,
    /// where Wyre's banking partner may notify Wyre at a later time.
    Reversed,
}

impl Default for TransferStatus {
    fn default() -> Self {
        TransferStatus::Pending
    }
}
