use bincode;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::Cursor;
extern crate tickstream;

use tickstream::streams::Chunk2;
use tickstream::vendor::binance_ws as bn;
//
// Named, Vendor specific Platforms specify :
// Root URL, Any setup, preloading, auth, etc.
fn main() {
    let bd1: Vec<bn::BookDepthUpdate> = (100..200)
        .map(|i| bn::BookDepthUpdate {
            event: "BookDepthUpdate".to_owned(),
            event_time: i * 1000 as u64,
            symbol: "BTCUSD".to_owned(),
            first_update_id: i + 400,
            last_update_id: i + 500,
            bids: vec![
                ((i * 30 - 5).into(), 100.into()),
                ((i * 30).into(), 100.into()),
                ((i * 30 + 5).into(), 100.into()),
            ],
            asks: vec![
                ((i * 40 - 5).into(), 100.into()),
                ((i * 40).into(), 100.into()),
                ((i * 40 + 5).into(), 100.into()),
            ],
        })
        .collect();
    let bd2: Vec<bn::BookDepthUpdate> = (300..400)
        .map(|i| bn::BookDepthUpdate {
            event: "BookDepthUpdate".to_owned(),
            event_time: i * 1000 as u64,
            symbol: "BTCUSD".to_owned(),
            first_update_id: i + 400,
            last_update_id: i + 500,
            bids: vec![
                ((i * 30 - 5).into(), 100.into()),
                ((i * 30).into(), 100.into()),
                ((i * 30 + 5).into(), 100.into()),
            ],
            asks: vec![
                ((i * 40 - 5).into(), 100.into()),
                ((i * 40).into(), 100.into()),
                ((i * 40 + 5).into(), 100.into()),
            ],
        })
        .collect();
    let td1: Vec<bn::Trade> = (200..300)
        .map(|i| bn::Trade {
            event: "Trade".to_owned(),
            event_time: i * 1000 as u64,
            symbol: "BTCUSD".to_owned(),
            price: Decimal::new((i * 500 + 7777) as i64, 6),
            quantity: 1000.into(),
            buyer: 42,
            seller: 142,
            trade_time: i * 1000 as u64,
            maker: true,
            _ignore: None,
        })
        .collect();
    let td2: Vec<bn::Trade> = (400..500)
        .map(|i| bn::Trade {
            event: "Trade".to_owned(),
            event_time: i * 1000 as u64,
            symbol: "BTCUSD".to_owned(),
            price: Decimal::new((i * 500 + 7777) as i64, 6),
            quantity: 1000.into(),
            buyer: 42,
            seller: 142,
            trade_time: i * 1000 as u64,
            maker: true,
            _ignore: None,
        })
        .collect();

    let chunks: Vec<Chunk2<bn::BookDepthUpdate, bn::Trade>> = vec![
        Chunk2::A(bd1),
        Chunk2::B(td1),
        Chunk2::A(bd2),
        Chunk2::B(td2),
    ];

    let mut buff = Cursor::new(vec![0; 10000000]);
    bincode::serialize_into(&mut buff, &chunks).expect("serialize_into");
    let result: Vec<Chunk2<bn::BookDepthUpdate, bn::Trade>> =
        bincode::deserialize(buff.get_ref()).expect("deserialize");

    assert_eq!(chunks, result);
    println!("success!");

    /*
    let book_stream = BookStream<BinanceBase>::start().await?;
    let trade_stream = TradeStream<BinanceBase>::start().await?;
    let book_writer = MixedFileSink2<Book, Trade>::(book_stream, trade_stream).await?;
    */
}

/* when receiving...
 *
 * {
 *   let read_stream = book_reader(MixedFileSource2<BookStream, TradeStream>::new()?;
 *   while let Some(item) = read_stream.next().await? {
 *      match item {
 *          Either::Left(book) => { ... },
 *          Either::Right(trade) => { ... },
 *      }
 *   }
 *
 *
 */
