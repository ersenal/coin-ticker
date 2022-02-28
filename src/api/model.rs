use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Deserialize)]
#[serde(tag = "e")]
pub enum MarketStream {
    #[serde(rename = "24hrTicker")]
    Ticker(Ticker),
}

#[serde_as]
#[derive(Deserialize)]
pub struct Ticker {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    #[serde_as(as = "DisplayFromStr")]
    pub price_change: f32,
    #[serde(rename = "P")]
    #[serde_as(as = "DisplayFromStr")]
    pub price_change_percent: f32,
    #[serde(rename = "c")]
    #[serde_as(as = "DisplayFromStr")]
    pub last_price: f64,
}
