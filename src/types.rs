use serde::{de, Deserialize, Serialize};
use serde_json::{Error as DError, Value};
use std::collections::BTreeMap;
use thiserror::Error;
use tokio_tungstenite::tungstenite::Error as TungError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Websockets Error {0}")]
    WebsocketError(#[from] TungError),
    #[error("Serde Error {0}")]
    SerdeError(#[from] DError),
}

