use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Widget},
};

use super::crab::Crab;

pub struct Track(pub u32, pub u16);

impl Widget for Track {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title_top(format!("{}", self.0))
            .render(area, buf);

        let [_, area, _] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(Crab::HEIGHT),
            Constraint::Length(1),
        ])
        .areas(area);

        let [_, area] = Layout::horizontal([Constraint::Length(self.1), Constraint::Min(0)]).areas(area);
        Crab.render(area, buf);
    }
}
