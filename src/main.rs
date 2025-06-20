mod cli;
mod app;
mod ui;
mod widgets;

use app::App;

fn main() {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    let _ = app.run(&mut terminal);

    ratatui::restore();
}
