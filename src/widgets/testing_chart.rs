use ratatui::{
    prelude::*,
    buffer::Buffer,
    layout::Rect,
    widgets::{Widget, Dataset, GraphType, Chart, Block, Axis},
};

pub struct TestingChart {}

impl Widget for &TestingChart {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let dataset = Dataset::default()
            .marker(symbols::Marker::HalfBlock)
            .style(Style::new().fg(Color::Blue))
            .graph_type(GraphType::Bar)
            // a bell curve
            .data(&[
                (0., 0.4), (10., 2.9),
                (20., 13.5),
                (30., 41.1),
                (40., 80.1),
                (50., 100.0),
                (60., 80.1),
                (70., 41.1),
                (80., 13.5),
                (90., 2.9),
                (100., 0.4),
            ]);
        
            Chart::new(vec![dataset])
                .block(Block::bordered().title_top(Line::from("Bar chart").cyan().bold().centered()))
                .x_axis(
                    Axis::default()
                        .style(Style::default().gray())
                        .bounds([0.0, 100.0])
                        .labels(["0".bold(), "50".into(), "100.0".bold()]),
                )
                .y_axis(
                    Axis::default()
                        .style(Style::default().gray())
                        .bounds([0.0, 100.0])
                        .labels(["0".bold(), "50".into(), "100.0".bold()]),
                )
                .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)))
                .render(area, buf);
    }
}
