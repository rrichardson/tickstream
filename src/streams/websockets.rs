use anyhow::{anyhow, Result, Context};
use futures::{ stream::Stream, SinkExt, StreamExt };
use serde::de::DeserializeOwned;
use serde_json;
use futures_util::pin_mut;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use async_stream::try_stream;
use url::Url;
use backoff::{ ExponentialBackoff, future::retry as backoff_retry };

pub async fn subscribe<'de, T, U, F>(url: &String, sub_msg: String, translate: F) -> Result<impl Stream<Item = Result<U>>>
    where
    T: DeserializeOwned + Unpin,
    U: Unpin,
    F: Fn(&T) -> Result<U>,
{
    let url : Url = url.parse()?;
    let s = try_stream! {
        loop {
            let (sock, _resp) = backoff_retry(ExponentialBackoff::default(), || async { Ok(connect_async(&url).await.context("failed to connect to remote WS server")?) }).await?;
            let (mut wr, rd) = sock.split();
            //let req = serde_json::to_string(&sub_msg)?;
            wr.send(Message::Text(sub_msg.clone().into())).await.context("failed to send message")?;
            pin_mut!(rd);
            while let Some(m) = rd.next().await {
                let msg = match m? {
                    Message::Text(txt) => {
                        let t = serde_json::from_str::<T>(txt.as_ref()).context("failed to deserializes from websockets text message")?;
                        translate(&t)
                    },
                    Message::Binary(bin) => {
                        let t = serde_json::from_slice::<T>(bin.as_ref()).context("failed to deserialize from websocket bin message")?;
                        translate(&t)
                    },
                    o => {println!("{:?}", o); Err(anyhow!("Unexpected msg: {:?}", o)) }
                }?;
                yield msg;
            }
        }
    };

    Ok(s)
}

/*
pub async fn get_token(api_key: &str, secret_key: &str) -> Result<String> {
    let url : Url = REST_URL.parse::<Url>()?.join("private")?.join(TOKEN_PATH)?;
    let res = reqwest::get(url).await?;
    Ok(res.text().await?)
}
*/
