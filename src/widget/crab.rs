use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

type CrabAscii = [[char; 7]; 4];
/// fg, bg
type CrabTheme = [[(Color, Color); 7]; 4];

const CRAB: CrabAscii = [
    ['▟', ' ', '●', ' ', '●', ' ', '▙'],
    [' ', '▚', '▄', '▄', '▄', '▞', ' '],
    [' ', '▐', '▒', '▓', '▒', '▌', ' '],
    [' ', '▘', ' ', ' ', ' ', '▝', ' '],
];

use Color::*;
const CRAB_THEME: CrabTheme = [
    [(Red, Reset), (Reset, Reset), (Black, Reset), (Reset, Reset), (Black, Reset), (Reset, Reset), (Red, Reset)],
    [(Reset, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Reset, Reset)],
    [(Reset, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Reset, Reset)],
    [(Reset, Reset), (Red, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Red, Reset), (Reset, Reset)],
];

pub struct Crab;

impl Crab {
    pub const HEIGHT: u16 = CRAB.len() as u16;
    pub const WIDTH: u16 = CRAB[0].len() as u16;
}

impl Widget for Crab {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let top = area.top();
        let left = area.left();

        CRAB.iter().enumerate().for_each(|(y, chunk)| {
            chunk.iter().enumerate().for_each(|(x, c)| {
                if *c == ' ' {
                    return;
                }

                let pos = (left + x as u16, top + y as u16);
                if let Some(cell) = buf.cell_mut(pos) {
                    cell.set_char(*c).set_fg(CRAB_THEME[y][x].0);
                }
            })
        });
    }
}
