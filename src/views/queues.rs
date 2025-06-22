use ratatui::{
    prelude::*,
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{
        Cell, Row, Table, Widget,
        TableState
    },
};
use crate::{client::ManagementClient, models::QueueInfo};
use crate::models::Rowable;
use std::sync::{Arc, mpsc};

#[derive(Debug)]
pub struct QueuesView<M>
where
    M: ManagementClient
{
    table_state:        TableState,
    state:              Vec<QueueInfo>,
    fetched_state_chan: mpsc::Receiver<Vec<QueueInfo>>,
    client:             Arc<M>,
}

impl<M> QueuesView<M>
where
    M: ManagementClient
{
    pub fn new(client: Arc<M>, fetched_state_chan: mpsc::Receiver<Vec<QueueInfo>>) -> Self
    {
        Self {
            table_state: TableState::default().with_selected(0),
            state: vec![],
            fetched_state_chan: fetched_state_chan,
            client: client,
        }
    }
}

impl<M> QueuesView<M>
where
    M: ManagementClient
{
    pub fn update(&mut self) {
        if let Some(s) = self.fetched_state_chan.try_iter().next() {
            self.state = s;
        }
    }
}

impl<M> Widget for &mut QueuesView<M>
where
    M: ManagementClient
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.update();

        let header = QueueInfo::headers()
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.state.iter().map(|r| {
            r.to_row()
                .iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .bottom_margin(1)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ],
        )
        .header(header);
        
        StatefulWidget::render(table, area, buf, &mut self.table_state);
    }
}
