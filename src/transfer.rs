//! This module corresponds to the [Transfers and Exchanges API](https://docs.sendwyre.com/docs/transfer-resources)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// See [Create Transfer - Parameters](https://docs.sendwyre.com/docs/create-transfer#parameters)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransfer {
    /// An SRN representing an account that the funds will be retrieved from.
    pub source: String,

    /// The amount to withdrawal from the source, in units of `sourceCurrency`.
    /// Only include `sourceAmount` OR `destAmount`, not both.
    pub source_amount: Option<f32>,

    /// The currency (ISO 3166-1 alpha-3) to withdrawal from the source wallet.
    pub source_currency: String,

    /// An email address, cellphone number, digital currency address or bank
    /// account to send the digital currency to. For bitcoin address use
    /// "bitcoin:[address]". Note: cellphone numbers are assumed to be a US
    /// number, for international numbers include a '+' and the country code as
    /// the prefix.
    pub dest: String,

    /// Specifies the total amount of currency to deposit (as defined in
    /// `depositCurrency`). Only include `sourceAmount` OR `destAmount`, not
    /// both.
    pub dest_amount: Option<f32>,

    /// The currency (ISO 3166-1 alpha-3) to deposit. if not provided, the
    /// deposit will be the same as the withdrawal currency (no exchange
    /// performed).
    pub dest_currency: Option<String>,

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
    pub source_amount: f32,
    pub source_currency: String,
    pub dest: String,
    pub dest_amount: f32,
    pub dest_currency: String,
    pub status: String,
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
    pub fees: HashMap<String, f32>, // currency => amount
    pub total_fees: f32,
    // pub blockchain_tx: ???,
    pub message: Option<String>,
    pub custom_id: Option<String>,
}
