use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::api::model::Ticker;

pub struct MarketSummary<'a> {
    ticker: &'a Ticker,
}

impl<'a> MarketSummary<'a> {
    pub fn new(ticker: &'a Ticker) -> Self {
        Self { ticker }
    }
}

impl<'a> Widget for MarketSummary<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                &self.ticker.symbol,
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Plain);

        block.render(area, buf);

        buf.set_string(
            area.x + 1,
            area.y + 1,
            format!("{}", self.ticker.last_price),
            Style::default().add_modifier(Modifier::BOLD),
        );

        let (prefix, color) = if self.ticker.price_change.is_sign_negative() {
            ("", Color::Red)
        } else {
            ("+", Color::Green)
        };

        buf.set_string(
            area.x + 1,
            area.y + 2,
            format!(
                "{}{} ({}{}%)",
                prefix, self.ticker.price_change, prefix, self.ticker.price_change_percent
            ),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        );
    }
}
