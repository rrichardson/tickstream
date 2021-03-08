use serde::{de, Deserialize, Serialize};
use serde_json::{Error as DError, Value};
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
struct Post {
    rating: f32,
}

fn num_or_str<'de, D: de::Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom),
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number")),
        _ => return Err(de::Error::custom("wrong type")),
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Subscribe {
    pub event: String,
    pub subscription: Subscription,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscription {
    pub name: SubscriptionName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratecounter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SubscriptionName {
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "ohlc")]
    Ohlc,
    #[serde(rename = "openOrders")]
    OpenOrders,
    #[serde(rename = "ownTrades")]
    OwnTrades,
    #[serde(rename = "spread")]
    Spread,
    #[serde(rename = "ticket")]
    Ticker,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "*")]
    All,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    /// scaling decimal places for record keeping
    pub decimals: u32,
    /// scaling decimal places for output display
    pub display_decimals: u32,
    /// asset class
    pub aclass: String,
    /// alternate name
    pub altname: String,
}
#[derive(Debug, Deserialize)]
pub struct Fee {
    pub volume: f64,
    pub percent: f64,
}

#[derive(Debug, Deserialize)]
pub struct AssetPair {
    /// alternate pair name
    pub altname: String,
    /// WebSocket pair name (if available)
    pub wsname: Option<String>,
    /// asset class of base component
    pub aclass_base: String,
    /// asset id of base component
    pub base: String,
    /// asset class of quote component
    pub aclass_quote: String,
    /// asset id of quote component
    pub quote: String,
    /// volume lot size
    pub lot: String,
    /// scaling decimal places for pair
    pub pair_decimals: u32,
    /// scaling decimal places for volume
    pub lot_decimals: u32,
    /// amount to multiply lot volume by to get currency volume
    pub lot_multiplier: u32,
    /// leverage_buy = array of leverage amounts available when buying
    pub leverage_buy: Vec<f64>,
    /// leverage_sell = array of leverage amounts available when selling
    pub leverage_sell: Vec<f64>,
    /// fee schedule array in [volume, percent fee] tuples
    pub fees: Vec<Fee>,
    /// maker fee schedule array in [volume, percent fee] tuples (if on maker/taker)
    pub fees_maker: Option<Vec<Fee>>,
    /// volume discount currency
    pub fee_volume_currency: String,
    /// margin call level
    pub margin_call: u32,
    /// stop-out/liquidation margin level
    pub margin_stop: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candle {
    pub time: u64,
    #[serde(deserialize_with = "num_or_str")]
    pub open: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub high: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub low: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub close: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub vwap: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub volume: f64,
    pub count: u64,
}

#[derive(Clone, Copy, Debug)]
pub enum Interval {
    M1 = 1,
    M5 = 5,
    M15 = 15,
    M30 = 30,
    H1 = 60,
    H4 = 240,
    D1 = 1440,
    D7 = 10080,
    D15 = 21600,
}

impl Default for Interval {
    fn default() -> Self {
        Interval::M1
    }
}

#[derive(Debug, Deserialize)]
pub struct Order {
    #[serde(deserialize_with = "num_or_str")]
    pub price: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub volume: f64,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct Orders {
    /// ask side array of array entries(<price>, <volume>, <timestamp>)
    pub asks: Vec<Order>,
    /// bid side array of array entries(<price>, <volume>, <timestamp>)
    pub bids: Vec<Order>,
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub error: Vec<String>,
    pub result: T,
}

#[derive(Debug, Deserialize)]
pub struct OHLCResponse {
    #[serde(flatten)]
    pub data: BTreeMap<String, Vec<Candle>>,
    pub last: u64,
}

#[derive(Debug, Deserialize)]
pub struct TradeResponse {
    #[serde(flatten)]
    pub data: BTreeMap<String, Vec<Trade>>,
    pub last: String, // TODO: u64
}

#[derive(Debug, Deserialize)]
pub struct SpreadResponse {
    #[serde(flatten)]
    pub data: BTreeMap<String, Vec<Spread>>,
    pub last: u64,
}

#[derive(Debug, Deserialize)]
pub struct Spread {
    pub time: u64,
    #[serde(deserialize_with = "num_or_str")]
    pub bid: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub ask: f64,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    #[serde(deserialize_with = "num_or_str")]
    pub price: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub whole_lot_volume: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub lot_volume: f64,
}

#[derive(Debug, Deserialize)]
pub struct CloseLevel {
    #[serde(deserialize_with = "num_or_str")]
    pub price: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub volume: f64,
}

#[derive(Debug, Deserialize)]
pub struct TimeLevel {
    #[serde(deserialize_with = "num_or_str")]
    pub today: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub last24: f64,
}

#[derive(Debug, Deserialize)]
pub struct TickerPair {
    /// ask array(<price>, <whole lot volume>, <lot volume>),
    pub a: Level,
    /// bid array(<price>, <whole lot volume>, <lot volume>),
    pub b: Level,
    /// last trade closed array(<price>, <lot volume>),
    pub c: CloseLevel,
    /// volume array(<today>, <last 24 hours>),
    pub v: TimeLevel,
    /// volume weighted average price array(<today>, <last 24 hours>),
    pub p: TimeLevel,
    /// number of trades array(<today>, <last 24 hours>),
    pub t: [u32; 2],
    /// low array(<today>, <last 24 hours>),
    pub l: TimeLevel,
    /// high array(<today>, <last 24 hours>),
    pub h: TimeLevel,
    /// today's opening price
    #[serde(deserialize_with = "num_or_str")]
    pub o: f64,
}
#[derive(Debug, Deserialize)]
pub struct Time {
    /// server time as unix timestamp
    pub unixtime: u64,
    /// server time as RFC 1123 time format
    pub rfc1123: String,
}

#[derive(Debug, Deserialize)]
pub struct Trade {
    #[serde(deserialize_with = "num_or_str")]
    pub price: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub volume: f64,
    #[serde(deserialize_with = "num_or_str")]
    pub time: f64,
    pub side: TradeSide,
    pub type_: TradeType,
    pub miscellaneous: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum TradeSide {
    #[serde(rename = "b")]
    Buy,
    #[serde(rename = "s")]
    Sell,
}
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum TradeType {
    #[serde(rename = "m")]
    Market,
    #[serde(rename = "l")]
    Limit,
}
