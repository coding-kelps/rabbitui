use crate::{
    app::App,
    widgets::TestingChart,
};
use ratatui::{
    prelude::*,
    buffer::Buffer,
    layout::Rect,
    widgets::{Widget, Paragraph, Tabs},
};

const ASCII: &str = r#"
   ___       __   __   _ ______     _ 
  / _ \___ _/ /  / /  (_)_  __/_ __(_)
 / , _/ _ `/ _ \/ _ \/ / / / / // / / 
/_/|_|\_,_/_.__/_.__/_/ /_/  \_,_/_/  
                                      
"#;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [banner, tabs] = Layout::vertical([
                Constraint::Length(6),
                Constraint::Fill(1),
            ])
            .areas(area);

        Paragraph::new(ASCII)
            .render(banner, buf);
        self.render_tabs(tabs, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let [tab_titles, tab_content] = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .areas(area);

        Tabs::new(vec!["Overview", "Exchanges", "Queues"])
            .padding("", "")
            .divider(" ")
            .render(tab_titles, buf);

        self.render_overview(tab_content, buf);
    }

    fn render_overview(&self, area: Rect, buf: &mut Buffer) {
        let [message_chart, disk_chart] = Layout::vertical([
                Constraint::Ratio(1, 2),
                Constraint::Ratio(1, 2),
            ])
            .areas(area);

        TestingChart{}.render(message_chart, buf);
        TestingChart{}.render(disk_chart, buf);
    }
}
