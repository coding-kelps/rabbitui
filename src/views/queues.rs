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
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct QueuesView<M>
where
    M: ManagementClient
{
    state:     TableState,
    data:      RwLock<Vec<QueueInfo>>,
    client:    Arc<M>,
}

impl<M> QueuesView<M>
where
    M: ManagementClient
{
    pub fn new(client: Arc<M>) -> Self
    {
        Self {
            state: TableState::default().with_selected(0),
            data: RwLock::new(vec![]),
            client: client,
        }
    }
}

impl<M> QueuesView<M>
where
    M: ManagementClient
{
    pub async fn update(&self) {
        let queues_info = self.client.get_queues_info().await;

        {
            let mut data = self.data.write().unwrap();

            *data = queues_info;
        }
    }
}

impl<M> Widget for &mut QueuesView<M>
where
    M: ManagementClient
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = QueueInfo::headers()
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let data: Vec<QueueInfo>;

        {
            data = (*self.data.read().unwrap()).clone();
        }

        let rows = data.iter().map(|r| {
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
        
        StatefulWidget::render(table, area, buf, &mut self.state);
    }
}
