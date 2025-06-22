use ratatui::{
    buffer::Buffer, layout::Rect, prelude::*, text::Text, widgets::{
        Axis, Block, Chart as RatatuiChart, Dataset, GraphType, Paragraph, Widget
    }
};

#[derive(Debug)]
struct ChartData
{
    pub title: String,
    data: Vec<(f64, f64)>,
    max_x:  usize,
}

impl ChartData
{
    fn new(title: String, max_x: usize) -> Self {
        Self {
            title: title,
            data: Vec::<(f64, f64)>::with_capacity(max_x),
            max_x: max_x,
        }
    }

    fn push(&mut self, y: f64) {
        let next_x = self.data.len() as f64;
        self.data.push((next_x, y));

        if self.data.len() > self.max_x {
            self.data.remove(0);
            for (i, (x, _)) in self.data.iter_mut().enumerate() {
                *x = i as f64;
            }
        }
    }

    fn max_y(&self) -> f64 {
        self.data
            .iter()
            .map(|&(_, y)| y)
            .fold(0.0_f64, f64::max)
    }

    fn as_data_points(&self) -> &[(f64, f64)] {
        &self.data
    }

    fn last_to_string(&self) -> String {
        match self.data.iter().last() {
            Some(&(_, v)) => v.to_string(),
            None => "-".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Chart {
    data: Vec<(ChartData, ratatui::style::Color)>,
    max_x: usize,
}

impl Chart {
    pub fn new(data: Vec<(String, ratatui::style::Color)>, max_x: usize) -> Self {
        Self {
            data: data.into_iter().map(|(t, c)| (ChartData::new(t, max_x), c)).collect(),
            max_x: max_x,
        }
    }

    pub fn update(&mut self, data_points: Vec<f64>) {
        self.data.iter_mut()
            .zip(data_points.into_iter())
            .for_each(|((d, _), p)| d.push(p));
    }

    fn max_y(&self) -> f64 {
        self.data.iter()
            .map(|(d, _)| d.max_y())
            .fold(0.0_f64, f64::max)
    }
}

impl Widget for &mut Chart
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [chart, legend] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(20),
        ]).areas(area);

        let dataset = self.data.iter()
            .map(|(d, c)|
                Dataset::default()
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(c.clone()))
                    .graph_type(GraphType::Line)
                    .data(d.as_data_points())
            )
            .collect();

        RatatuiChart::new(dataset)
            .block(Block::bordered())
            .x_axis(Axis::default().bounds([0.0, (self.max_x as f64) * 1.2]))
            .y_axis(Axis::default().bounds([0.0, self.max_y() * 1.2]))
            .render(chart, buf);

        Paragraph::new(Text::from(
                self.data.iter()
                    .map(|(d, c)|
                        Line::from(format!("{} {}", d.title, d.last_to_string()))
                            .style(Style::default()
                            .fg(c.clone()))
                    )
                    .collect::<Vec<Line>>()
            ))
            .block(Block::bordered())
            .render(legend, buf);
    }
}
