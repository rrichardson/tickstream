#![feature(type_alias_impl_trait)]
use futures::stream::Stream;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use anyhow::Result;

pub mod streams;
pub mod types;
pub mod vendor;

use streams::StreamDatum;
pub use vendor::*;

// mostly for documentation purposes
type Price = Decimal;
type Quantity = Decimal;
type BookList = Vec<(Price, Quantity)>;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Trade {
    pub event: String,   // Event type
    pub event_time: u64, // Event time
    pub symbol: String,
    pub price: Price,
    pub quantity: Quantity,
    pub buyer: u32,
    pub seller: u32,
    pub trade_time: u64,
    pub maker: bool,
}

impl StreamDatum for Trade {
    const ID: u16 = 500;
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BookUpdate {
    pub event: String,        // Event type
    pub event_time: u64,      // Event time
    pub symbol: String,       // Symbol
    pub first_update_id: u64, // First update ID in event
    pub last_update_id: u64,  // Last update ID in  event
    pub bids: BookList,
    pub asks: BookList,
}

impl StreamDatum for BookUpdate {
    const ID: u16 = 501;
}

#[async_trait]
trait Platform {
    type BookStream: Stream<Item = Result<BookUpdate>>;
    type TradeStream: Stream<Item = Result<Trade>>;
    async fn start_book_stream(instrument: &'static str) -> Result<Self::BookStream>;
    async fn start_trade_stream(instrument: &'static str) -> Result<Self::TradeStream>;
}
