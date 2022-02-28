use clap::{arg, crate_version, Command};

pub struct Args {
    pub ui_fps: u32,
    pub ws_api: Vec<String>,
    pub trading_pairs: Vec<String>,
}

impl Args {
    pub fn new() -> Self {
        let matches = Command::new("coin-ticker")
            .version(crate_version!())
            .args(&[
                arg!(ui_fps: -f --fps <FPS> "Sets the UI refresh rate")
                    .default_value("1")
                    .required(false),
                arg!(ws_api: -a --api <WS_APIS> "Sets Binance WebSocket URLs")
                    .default_values(&[
                        "wss://stream8.binance.com:9443",
                        "wss://stream7.binance.com:9443",
                        "wss://stream6.binance.com:9443",
                        "wss://stream5.binance.com:9443",
                        "wss://stream4.binance.com:9443",
                        "wss://stream3.binance.com:9443",
                        "wss://stream2.binance.com:9443",
                        "wss://stream1.binance.com:9443",
                        "wss://stream.binance.com:9443",
                    ])
                    .multiple_values(true)
                    .required(false),
                arg!(trading_pairs: -p --pairs <TRADING_PAIRS> "Sets trading pairs to track")
                    .default_values(&[
                        "ethbtc", "ethusdt", "btcusdt", "ltcusdt", "xrpusdt", "ltcbtc", "xrpbtc",
                        "bchusdt", "bchbtc", "linkusdt",
                    ])
                    .multiple_values(true)
                    .required(false),
            ])
            .get_matches();

        Self {
            ui_fps: matches.value_of("ui_fps").unwrap().parse().unwrap(),
            ws_api: matches
                .values_of("ws_api")
                .map(|vals| vals.collect::<Vec<_>>())
                .unwrap_or_default()
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
            trading_pairs: matches
                .values_of("trading_pairs")
                .map(|vals| vals.collect::<Vec<_>>())
                .unwrap_or_default()
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        }
    }
}
