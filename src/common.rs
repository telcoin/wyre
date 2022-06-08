use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

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
