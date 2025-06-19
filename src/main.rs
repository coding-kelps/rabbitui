mod cli;
mod app;

use app::App;

fn main() {
    let mut terminal = ratatui::init();

    let _ = App::default().run(&mut terminal);

    ratatui::restore();
}
