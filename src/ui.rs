use crate::{
    app::{App, SelectedTab},
};
use ratatui::{
    prelude::*,
    buffer::Buffer,
    layout::Rect,
    widgets::{Widget, Paragraph, Tabs},
};
use strum::IntoEnumIterator;

const ASCII: &str = r#"
   ___       __   __   _ ______     _ 
  / _ \___ _/ /  / /  (_)_  __/_ __(_)
 / , _/ _ `/ _ \/ _ \/ / / / / // / / 
/_/|_|\_,_/_.__/_.__/_/ /_/  \_,_/_/  
                                      
"#;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [banner, tabs] = Layout::vertical([
                Constraint::Length(6),
                Constraint::Fill(1),
            ])
            .areas(area);

        Paragraph::new(ASCII)
            .render(banner, buf);

        let [tab_titles, tab_content] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .areas(tabs);

        let selected_tab_index = self.selected_tab as usize;

        Tabs::new(SelectedTab::iter().map(SelectedTab::title))
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(tab_titles, buf);

        match self.selected_tab {
            SelectedTab::Overview => {},
            SelectedTab::Exchanges => {},
            SelectedTab::Queues => {
                self.queues.render(tab_content, buf);
            },
        }
    }
}

impl SelectedTab {
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .into()
    }
}
