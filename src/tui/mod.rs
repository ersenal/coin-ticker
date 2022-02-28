use crossbeam_channel::Receiver;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::{io::Stdout, time::Duration};
use std::{
    io::{stdout, Result},
    time::Instant,
};
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app;
mod grid;
mod market_summary;

use crate::{api::model::MarketStream, config::Args};
use app::App;

pub fn tui(can_terminate: Arc<AtomicBool>, market_data: Receiver<MarketStream>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = event_loop(&mut terminal, can_terminate, market_data);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    can_terminate: Arc<AtomicBool>,
    market_data: Receiver<MarketStream>,
) -> Result<()> {
    let Args {
        trading_pairs,
        ui_fps,
        ..
    } = Args::new();

    let poll_timeout = Duration::from_secs(1) / ui_fps;
    let mut last_frame = Instant::now();

    let mut app = App::new(trading_pairs);

    while !app.can_quit() {
        if crossterm::event::poll(
            poll_timeout
                .checked_sub(last_frame.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0)),
        )? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(&key);
            }
        }

        if last_frame.elapsed() >= poll_timeout {
            app.update(&market_data);
            terminal.draw(|f| app.render(f))?;
            last_frame = Instant::now();
        }

        if can_terminate.load(std::sync::atomic::Ordering::Relaxed) {
            app.quit();
        }
    }

    Ok(())
}
