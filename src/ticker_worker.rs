use crossbeam_channel::{bounded, Receiver, Sender};
use rand::Rng;
use serde_json::from_str;
use tokio_tungstenite::tungstenite::{Message, Result};
use url::Url;

use crate::api::model::MarketStream;
use crate::api::ws_client::WsClient;
use crate::api::ws_ops::*;
use crate::config::Args;

pub struct TickerWorker {
    pub rx: Receiver<MarketStream>,
    tx: Sender<MarketStream>,
}

impl TickerWorker {
    pub fn new() -> Self {
        let (tx, rx) = bounded::<MarketStream>(1024);
        TickerWorker { tx, rx }
    }

    pub async fn start(&mut self) {
        let Args {
            ws_api,
            trading_pairs,
            ..
        } = Args::new();

        loop {
            let api_url = &ws_api[rand::thread_rng().gen_range(0..ws_api.len())];
            let mut client = WsClient::new(Url::parse(format!("{}/ws", api_url).as_str()).unwrap());
            if self.connect(&mut client, &trading_pairs).await.is_err() {
                let _ = client.disconnect().await;
            }
        }
    }

    async fn connect(&self, client: &mut WsClient, trading_pairs: &[String]) -> Result<()> {
        client.connect().await?;

        let subs: Vec<String> = trading_pairs
            .iter()
            .map(|s| format!("{}@ticker", s))
            .collect();
        let sub_msg: Message = Subscribe::new(subs).into();
        client.send(sub_msg).await?;

        while let Some(msg) = client.receive().await {
            match msg? {
                Message::Text(m) => {
                    if let Ok(data) = from_str::<MarketStream>(&m) {
                        let _ = self.tx.try_send(data);
                    }
                }
                Message::Binary(_) | Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => {}
                Message::Close(_) => break,
            };
        }

        Ok(())
    }
}
