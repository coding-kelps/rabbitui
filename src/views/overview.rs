use ratatui::{
    buffer::Buffer, layout::Rect, prelude::*, text::Text, widgets::{
        Axis, Block, Dataset, GraphType, Paragraph, Widget
    }
};
use crate::{
    client::ManagementClient,
    models::{
        Overview, OverviewMessageRates, OverviewQueueTotals, RateContainer,
    },
    widgets::Chart,
};
use std::sync::{mpsc, Arc};


#[derive(Debug)]
pub struct OverviewView<M>
where
    M: ManagementClient
{
    message_rate_chart: Chart,
    disk_io_chart:      Chart,
    fetched_state_chan: mpsc::Receiver<Overview>,
    client:             Arc<M>,
}

impl<M> OverviewView<M>
where
    M: ManagementClient
{
    pub fn new(client: Arc<M>, fetched_state_chan: mpsc::Receiver<Overview>) -> Self
    {
        Self {
            message_rate_chart: Chart::new(vec![
                    (String::from("Total"), Color::Red),
                    (String::from("Ready"), Color::Yellow),
                    (String::from("Unacked"), Color::LightBlue),
                ],100),
            disk_io_chart:      Chart::new(vec![
                    (String::from("Disk read"), Color::Magenta),
                    (String::from("Disk write"), Color::LightGreen),
                ],100),
            fetched_state_chan: fetched_state_chan,
            client: client,
        }
    }

    pub fn update(&mut self) {
        if let Some(update) = self.fetched_state_chan.try_iter().next() {
            self.message_rate_chart.update(vec![
                update.queue_totals.messages,
                update.queue_totals.messages_ready,
                update.queue_totals.messages_unacked,
            ]);
            self.disk_io_chart.update(vec![
                update.message_stats.disk_writes_details.rate,
                update.message_stats.disk_reads_details.rate,
            ]);
        }
    }
}

impl<M> Widget for &mut OverviewView<M>
where
    M: ManagementClient
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.update();

        let [message_chart_area, disk_io_chart_area] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).areas(area);

        self.message_rate_chart.render(message_chart_area, buf);
        self.disk_io_chart.render(disk_io_chart_area, buf);
    }
}
