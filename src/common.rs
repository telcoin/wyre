use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

/// System Resource Name is a typed identifier that may reference any object within the Wyre
/// platform. Many of our API calls and data schemas leverage SRNs in order to add flexibility
/// and decouple services. All SRNs follow the same URI-like format:
///
/// `type:identifier`
#[derive(Debug, Clone)]
pub enum SystemResourceName {
    /// A Wyre account, e.g. `account:AC_XXXXXXXX`
    Account(String),
    /// A Wyre user, e.g. `user:US_XXXXXXXX`
    User(String),
    /// A single wallet that can hold cryptocurrency, e.g. `wallet:WA_XXXXXXXX`
    Wallet(String),
    /// A transfer (possibly including a conversion) of currency
    Transfer(String),
    /// A payment method such as a bank account, e.g. `paymentmethod:PA_XXXXXXXX`
    PaymentMethod(String),
    /// This is attached as a suffix to the payment method when pulling funds into and account via ACH.
    ///
    /// Example:
    /// `"source": "paymentmethod:PA-W7YN28ABCHT:ach"`
    AchPaymentMethod(String),
    /// An email address, e.g. `email:dev@sendwyre.com`
    Email(String),
    /// A cellphone number, e.g. `cellphone:+15555555555`
    CellPhone(String),
    /// Bitcoin blockchain addresses. e.g. `bitcoin:1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2`
    ///
    /// NOTE: TestWyre expects a Bitcoin testnet address i.e. `bitcoin:n4VQ5YdHf7hLQ2gWQYYrcxoE5B7nWuDFNF`.
    Bitcoin(String),
    /// Ethereum blockchain address. e.g.
    /// `ethereum:0xBB9bc244D798123fDe783fCc1C72d3Bb8C1894131`
    ///
    /// NOTE:
    /// Transfers of ERC-20 tokens use the "ethereum:" SRN.
    Ethereum(String),
    /// Avalanche (AVAX) blockchain addresses (X and C chains). e.g.
    ///
    /// X Chain:
    /// `avalanche:X-fuji159ney792ctzweqfhuc39rkp0h8fsmzjhu4fjk4`
    ///
    /// C Chain:
    /// `avalanche:0x6b53a58cf99b698afe78035e58f1a8f5f8235663`
    Avalanche(String),
    /// Stellar (XLM) blockchain address. e.g.
    /// `stellar:GD7WXI7AOAK2CIPZVBEFYLS2NQZI2J4WN4HFYQQ4A2OMFVWGWAL3IW7K:LEMNM383ACX`
    ///
    /// NOTE:
    /// Transfers from an external stellar address will require the User ID in the memo.
    Stellar(String),
    /// Algorand (ALGO and aUSDC) blockchain address.
    Algorand(String),
    /// Polygon (MATIC) blockchain address.
    Matic(String),
    /// Flow blockchain address. e.g.
    /// `flow:0xead892083b3e2c6c`
    Flow(String),
    /// Loopring blockchain address.
    Loopring(String),
}
impl ToString for SystemResourceName {
    fn to_string(&self) -> String {
        match self {
            SystemResourceName::PaymentMethod(identifier) => {
                format!("paymentmethod:{}", identifier)
            }
            SystemResourceName::AchPaymentMethod(identifier) => {
                format!("paymentmethod:{}:ach", identifier)
            }
            SystemResourceName::Email(identifier) => format!("email:{}", identifier),
            SystemResourceName::CellPhone(identifier) => format!("cellphone:{}", identifier),
            SystemResourceName::Bitcoin(identifier) => format!("bitcoin:{}", identifier),
            SystemResourceName::Ethereum(identifier) => format!("ethereum:{}", identifier),
            SystemResourceName::Avalanche(identifier) => format!("avalanche:{}", identifier),
            SystemResourceName::Stellar(identifier) => format!("stellar:{}", identifier),
            SystemResourceName::Algorand(identifier) => format!("algorand:{}", identifier),
            SystemResourceName::Matic(identifier) => format!("matic:{}", identifier),
            SystemResourceName::Flow(identifier) => format!("flow:{}", identifier),
            SystemResourceName::Loopring(identifier) => format!("loopring:{}", identifier),
            SystemResourceName::Account(identifier) => format!("account:{}", identifier),
            SystemResourceName::User(identifier) => format!("user:{}", identifier),
            SystemResourceName::Wallet(identifier) => format!("wallet:{}", identifier),
            SystemResourceName::Transfer(identifier) => format!("transfer:{}", identifier),
        }
    }
}

/// A financial amount (the value is not scaled)
pub type Amount = BigDecimal;

/// An address
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

/// See [Supported Currencies](https://docs.sendwyre.com/docs/supported-currencies-1)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    /// United States Dollar
    USD,
    /// Euro
    EUR,
    /// British Pound Sterling
    GBP,
    /// Australian Dollar
    AUD,
    /// Canadian Dollar
    CAD,
    /// New Zealand Dollar
    NZD,
    /// Argentine Peso
    ARS,
    /// Brazilian Real
    BRL,
    /// Swiss Franc
    CHF,
    /// Chilean Peso
    CLP,
    /// Colombian Peso
    COP,
    /// Czech Koruna
    CZK,
    /// Danish Krone
    DKK,
    /// Hong Kong Dollar
    HKD,
    /// Israeli New Shekel
    ILS,
    /// Indian Rupee
    INR,
    /// Icelandic Krona
    ISK,
    /// Japanese Yen
    JPY,
    /// South Korean Won
    KRW,
    /// Mexican Peso
    MXN,
    /// Malaysian Ringgit
    MYR,
    /// Norwegian Krone
    NOK,
    /// Philippine Peso
    PHP,
    /// Polish Zloty
    PLN,
    /// Swedish Krona
    SEK,
    /// Singapore Dollar
    SGD,
    /// Thai Baht
    THB,
    /// Vietnamese Dong
    VND,
    /// South African Rand
    ZAR,

    /// Bitcoin
    BTC,
    /// Ethereum
    ETH,
    /// Stellar
    XLM,
    /// Stellar USDC
    #[serde(rename = "sUSDC")]
    SUSDC,
    /// Avalanche
    AVAX,
    /// DAI
    DAI,
    /// Palm DAI
    #[serde(rename = "pDAI")]
    PDAI,
    /// USD Coin
    USDC,
    /// Matic USDC
    #[serde(rename = "mUSDC")]
    MUSDC,
    /// Liquid BTC
    #[serde(rename = "L-BTC")]
    LBTC,
    /// Tether
    USDT,
    /// Binance USD
    BUSD,
    /// Gemini Dollar
    GUSD,
    /// Paxos Standard
    PAX,
    /// Stably Dollar
    USDS,
    /// Aave
    AAVE,
    /// Compound
    COMP,
    /// Chainlink
    LINK,
    /// Wrapped Bitcoin
    WBTC,
    /// Basic Attention Token
    BAT,
    /// Curve
    CRV,
    /// Maker
    MKR,
    /// Synthetix
    SNX,
    /// UMA
    UMA,
    /// Uniswap
    UNI,
    /// yearn.finance
    YFI,
    /// Digital JPY
    GYEN,
    /// Digital USD
    ZUSD,
    /// Polygon
    MATIC,

    /// All other currencies.
    #[serde(other)]
    Other,
}

/// See [Webhooks - Callback Urls](https://docs.sendwyre.com/docs/webhooks#callback-urls).
/// This webhook payload is sent when it is created or is always sent for user
/// and payment method updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataCallbackPayload {
    /// A unique identifier for this webhook subscription
    pub subscription_id: String,

    /// An SRN for the entity that the callback was designated for
    pub trigger: String,
}

/// See [Webhooks - Callback Urls](https://docs.sendwyre.com/docs/webhooks#callback-urls).
/// Webhook callback payloads either contain the entity that was updated, or
/// only contains metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallbackPayload<T> {
    /// The metadata of the subscription
    Metadata(MetadataCallbackPayload),

    /// The updated entity
    Data(T),
}

/// See [`UploadDocument`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum DocumentType {
    GovtId,
    DrivingLicense,
    PassportCard,
    Passport,
}
