use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

type CraBAscii = [[char; 9]; 5];
/// fg, bg
type CrabTheme = [[(Color, Color); 9]; 5];

const CRAB: CraBAscii = [
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', '▟', ' ', ' ', ' ', ' ', ' ', '▙', ' '],
    [' ', ' ', '▐', '●', '█', '●', '▌', ' ', ' '],
    ['/', '-', '▐', '█', '█', '█', '▌', '-', '\\'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
];

const KANI: CraBAscii = [
    [' ', ' ', '4', '4', '.', '5', ' ', ' ', ' '],
    ['▟', ' ', ' ', '▂', '▁', '▂', ' ', ' ', '▙'],
    [' ', '▗', '▄', '▄', '▄', '▄', '▄', '▖', ' '],
    ['▞', ' ', '▛', '▾', '█', '▾', '▜', ' ', '▚'],
    ['/', '-', '█', '▙', '▂', '▟', '█', '-', '\\'],
];

use Color::*;
const CRAB_THEME: CrabTheme = [
    [(Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset)],
    [(Reset, Reset), (Red, Reset), (Reset, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Reset, Reset), (Red, Reset), (Reset, Reset)],
    [(Reset, Reset), (Reset, Reset), (Red, Reset), (Black, Red), (Red, Reset), (Black, Red), (Red, Reset), (Reset, Reset), (Reset, Reset)],
    [(Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset), (Red, Reset)],
    [(Reset, Reset), (Reset, Reset), (Red, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Red, Reset), (Reset, Reset), (Reset, Reset)],
];

const AQUA_PINK: Color = Color::Rgb(235, 172, 219);
const AQUA_BLUE: Color = Color::Rgb(48, 57, 113);
const WHITE: Color = Color::Rgb(255, 255, 255);
const AQUA: Color = Color::Rgb(176, 234, 253);
const KANI_THEME: CrabTheme = [
    [(Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset), (Reset, Reset)],
    [(Red, Reset), (Red, Reset), (WHITE, Reset), (WHITE, Reset), (WHITE, Reset), (WHITE, Reset), (WHITE, Reset), (Red, Reset), (Red, Reset)],
    [(Reset, Reset), (AQUA, Reset), (AQUA_PINK, WHITE), (AQUA_PINK, WHITE), (AQUA_PINK, WHITE), (AQUA_PINK, WHITE), (AQUA_PINK, WHITE), (AQUA, Reset), (Reset, Reset)],
    [(AQUA_PINK, Reset), (Reset, Reset), (AQUA_PINK, Red), (WHITE, Red), (Red, Reset), (WHITE, Red), (AQUA_PINK, Red), (Reset, Reset), (AQUA_PINK, Reset)],
    [(Red, Reset), (Red, Reset), (AQUA_BLUE, Reset), (AQUA_BLUE, WHITE), (AQUA_BLUE, WHITE), (AQUA_BLUE, WHITE), (AQUA_BLUE, Reset), (Red, Reset), (Red, Reset)],
];

pub struct Crab(pub bool);

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

        let (ascii, theme) = if self.0 { (KANI, KANI_THEME) } else { (CRAB, CRAB_THEME) };

        ascii.iter().enumerate().for_each(|(y, chunk)| {
            chunk.iter().enumerate().for_each(|(x, c)| {
                if *c == ' ' {
                    return;
                }

                let pos = (left + x as u16, top + y as u16);
                if let Some(cell) = buf.cell_mut(pos) {
                    let theme = theme[y][x];
                    cell.set_char(*c).set_fg(theme.0).set_bg(theme.1);
                }
            })
        });
    }
}
