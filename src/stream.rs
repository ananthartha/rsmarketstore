use futures_channel::mpsc::UnboundedSender;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite, tungstenite::protocol::Message};

use crate::Error;

#[derive(Debug, Deserialize, Default)]
pub struct Msg<T> {
    pub key: String,
    pub data: T,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct Request {
    pub streams: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Stream(UnboundedSender<Message>);

impl Stream {
    pub async fn subscribe(self: &Self, streams: Vec<String>) -> Result<(), Error> {
        let request = Request { streams };

        let msg = rmp_serde::to_vec(&request).map_err(|source| Error::RequestEncodingError {
            request: request.clone(),
            source,
        })?;

        self.0
            .unbounded_send(Message::binary(msg))
            .map_err(|source| Error::UnableToSendRequestError {
                request: request.clone(),
                source,
            })
    }
}

pub async fn connect<T>(
    endpoint: &str,
) -> Result<(Stream, impl futures_util::Stream<Item = Msg<T>>), tungstenite::error::Error>
where
    T: for<'a> Deserialize<'a>,
{
    let (ws_stream, _) = connect_async(endpoint).await?;
    let (upstream, downstream) = ws_stream.split();

    let (upstream_tx, upstream_rx) = futures_channel::mpsc::unbounded::<Message>();
    tokio::spawn(upstream_rx.map(Ok).forward(upstream));

    Ok((
        Stream(upstream_tx.clone()),
        downstream
            .map(move |message| {
                if let Ok(Message::Ping(data)) = message.as_ref() {
                    upstream_tx
                        .unbounded_send(Message::Pong(data.clone()))
                        .unwrap();
                }
                message
            })
            .filter_map(|message| async {
                if let Ok(Message::Binary(data)) = message {
                    return rmp_serde::from_slice::<Msg<T>>(&data).ok();
                }

                None
            }),
    ))
}
