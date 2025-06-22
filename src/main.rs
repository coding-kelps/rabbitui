mod cli;
mod app;
mod ui;
mod widgets;
mod client;
mod models;
mod views;

use app::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    app.run(&mut terminal);

    ratatui::restore();

    Ok(())
}
