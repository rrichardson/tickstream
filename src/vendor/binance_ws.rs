use crate::streams::StreamDatum;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// mostly for documentation purposes
type Price = Decimal;
type Quantity = Decimal;
type BookList = Vec<(Price, Quantity)>;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BookTicker {
    #[serde(rename = "u")]
    pub update_id: u32, // order book updateId
    #[serde(rename = "s")]
    pub symbol: String, // symbol
    #[serde(rename = "b")]
    pub best_bid: Price, // best bid price
    #[serde(rename = "B")]
    pub bist_bid_qty: Decimal, // best bid quantity
    #[serde(rename = "a")]
    pub best_ask: Price, // best ask price
    #[serde(rename = "A")]
    pub best_ask_qty: Decimal, // best ask quantity
}

impl StreamDatum for BookTicker {
    const ID: u16 = 100;
}

/// BookDepth update event in a stream
/// The length of bids/asks should be last_update_id - first_update_id
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BookDepthUpdate {
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    pub event_time: u64, // Event time
    #[serde(rename = "s")]
    pub symbol: String, // Symbol
    #[serde(rename = "U")]
    pub first_update_id: u64, // First update ID in event
    #[serde(rename = "u")]
    pub last_update_id: u64, // Last update ID in  event
    #[serde(rename = "b")]
    pub bids: BookList,
    #[serde(rename = "a")]
    pub asks: BookList,
}

impl StreamDatum for BookDepthUpdate {
    const ID: u16 = 101;
}

/// Order Book Item
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub last_update_id: u64,
    pub bids: BookList,
    pub asks: BookList,
}

impl StreamDatum for Book {
    const ID: u16 = 102;
}

/// Trade Item
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Trade {
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    pub event_time: u64, // Event time
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price: Price,
    #[serde(rename = "q")]
    pub quantity: Quantity,
    #[serde(rename = "b")]
    pub buyer: u32,
    #[serde(rename = "s")]
    pub seller: u32,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub maker: bool,
    #[serde(rename = "M")]
    pub _ignore: Option<bool>,
}

impl StreamDatum for Trade {
    const ID: u16 = 103;
}

/// Aggregate Trade Item
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct AggregateTrade {
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    pub event_time: u64, // Event time
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub trade_id: u32,
    #[serde(rename = "p")]
    pub price: Price,
    #[serde(rename = "q")]
    pub quantity: Quantity,
    #[serde(rename = "f")]
    pub first_trade: u32,
    #[serde(rename = "l")]
    pub last_trade: u32,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub maker: bool,
    #[serde(rename = "M")]
    pub _ignore: Option<bool>,
}

impl StreamDatum for AggregateTrade {
    const ID: u16 = 104;
}
