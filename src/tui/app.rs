use std::collections::HashMap;

use crossbeam_channel::Receiver;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::backend::Backend;
use tui::Frame;

use crate::api::model::{MarketStream, Ticker};

use super::grid::Grid;
use super::market_summary::MarketSummary;

pub struct App {
    should_quit: bool,
    trading_pairs: Vec<String>,
    ticker: HashMap<String, Ticker>,
}

impl App {
    pub fn new(trading_pairs: Vec<String>) -> Self {
        App {
            trading_pairs,
            should_quit: false,
            ticker: HashMap::new(),
        }
    }

    pub fn render<B>(&mut self, f: &mut Frame<B>)
    where
        B: Backend,
    {
        Grid::with_cell_size(25, 4)
            .split(f.size())
            .into_iter()
            .zip(self.trading_pairs.iter())
            .for_each(|(cell, ticker)| {
                if let Some(t) = self.ticker.get(ticker) {
                    f.render_widget(MarketSummary::new(t), cell);
                }
            });
    }

    pub fn update(&mut self, market_data: &Receiver<MarketStream>) {
        market_data.try_iter().for_each(|msg| {
            let MarketStream::Ticker(t) = msg;
            self.ticker.insert(t.symbol.to_lowercase(), t);
        });
    }

    pub fn handle_key(&mut self, ev: &KeyEvent) {
        match ev.code {
            KeyCode::Char(c) if c == 'c' && ev.modifiers.contains(KeyModifiers::CONTROL) => {
                self.quit();
            }
            _ => {}
        }
    }

    pub fn can_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
