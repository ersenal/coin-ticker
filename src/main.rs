#![feature(adt_const_params)]

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use tokio::task::spawn_blocking;
use tokio::{select, signal, spawn};

mod api;
mod config;
mod ticker_worker;
mod tui;

use ticker_worker::TickerWorker;

use crate::tui::tui;
use crate::{config::Args};

#[tokio::main]
async fn main() {
    Args::new();

    let can_terminate = Arc::new(AtomicBool::new(false));
    let can_terminate_2 = can_terminate.clone();

    let mut ticker_worker = TickerWorker::new();
    let market_data = ticker_worker.rx.clone();
    let ui = spawn_blocking(move || tui(can_terminate_2, market_data));
    let ticker_worker = spawn(async move { ticker_worker.start().await });

    select! {
        _ = ui => {},
        _ = ticker_worker => {}
        _ = signal::ctrl_c() => {}
    };

    can_terminate.store(true, std::sync::atomic::Ordering::Relaxed);
}
