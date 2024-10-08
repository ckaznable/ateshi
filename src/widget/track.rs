use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Widget},
};

use super::crab::Crab;

/// fps, offset, eater egg flag
pub struct Track(pub u32, pub u16, pub bool);

impl Widget for Track {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title_top(format!("{}", self.0))
            .render(area, buf);

        let [area] = Layout::vertical([Constraint::Length(Crab::HEIGHT)]).flex(Flex::Center).areas(area);
        let [_, area] = Layout::horizontal([Constraint::Length(self.1), Constraint::Min(0)]).areas(area);
        Crab(self.2).render(area, buf);
    }
}
