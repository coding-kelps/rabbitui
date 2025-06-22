mod cli;
mod app;
mod ui;
mod widgets;
mod client;
mod models;
mod views;

use app::App;
use tokio::runtime::Runtime;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    let rt = Runtime::new()?;

    rt.block_on(async { app.run(&mut terminal).await })?;

    ratatui::restore();

    Ok(())
}
