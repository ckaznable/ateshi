use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::Widget,
};

pub struct Hint(pub u16, pub u16);

impl Widget for Hint {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [area] = Layout::vertical([Constraint::Length(1)])
            .flex(Flex::Center)
            .areas(area);

        Line::raw(format!(
            "Terminal size required Height = {} Width = {}",
            self.0, self.1
        ))
        .render(area, buf);
    }
}
