use crate::types::{ Subscription, Subscribe, Error };
use anyhow::{anyhow, Result, Context};
use futures::{ stream::Stream, SinkExt, StreamExt };
use serde::de::DeserializeOwned;
use serde_json;
use std::marker::PhantomData;
use futures_util::{pin_mut};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use async_stream::try_stream;
use url::Url;

const REST_URL : &str = "https://api.kraken.com/0/";
const WS_AUTH_URL : &str = "wss://ws-auth.kraken.com";
const WS_PUB_URL : &str = "wss://ws.kraken.com";
const TOKEN_PATH : &str = "GetWebSocketsToken";

pub struct PayloadDecoder<T> {
  _phan: PhantomData<T>,
}

pub async fn subscribe<T: DeserializeOwned + Unpin>(sub: Subscription, private: bool) -> Result<impl Stream<Item = Result<T>>> {

    let url : Url = if private {
        WS_AUTH_URL.parse()?
    } else {
        WS_PUB_URL.parse()?
    };

    let mut sub_msg = Subscribe { subscription: sub, event: "subscribe".into() };

    let s = try_stream! {
        loop {
            if private {
                let token = Some(get_token("", "").await.context("wat")?);
                sub_msg.subscription.token = token;
            }
            let (sock, _resp) = connect_async(&url).await.context("failed to connect to remote WS server")?;
            let (mut wr, rd) = sock.split();
            let req = serde_json::to_string(&sub_msg)?;
            wr.send(Message::Text(req.into())).await.context("failed to send message")?;
            pin_mut!(rd);
            while let Some(m) = rd.next().await {
                let msg = match m? {
                    Message::Text(txt) => {
                        serde_json::from_str::<T>(txt.as_ref()).context("failed to deserializes from websockets text message")
                    },
                    Message::Binary(bin) => {
                        serde_json::from_slice::<T>(bin.as_ref()).context("failed to deserialize from websocket bin message")
                    },
                    o => {println!("{:?}", o); Err(anyhow!("Unexpected msg: {:?}", o)) }
                }?;
                yield msg;
            }
        }
    };

    Ok(s)
}

pub async fn get_token(api_key: &str, secret_key: &str) -> Result<String> {
    let url : Url = REST_URL.parse::<Url>()?.join("private")?.join(TOKEN_PATH)?;
    let res = reqwest::get(url).await?;
    Ok(res.text().await?)
}
