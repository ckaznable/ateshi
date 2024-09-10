use std::{io::Stdout, time::Duration};

use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use ratatui::backend::CrosstermBackend;

const CRAB: [[char; 7]; 4] = [
    ['▟', ' ', '●', ' ', '●', ' ', '▙'],
    [' ', '▚', '▄', '▄', '▄', '▄', ' '],
    [' ', '░', '▒', '▓', '▒', '░', ' '],
    [' ', '▝', ' ', ' ', ' ', '▝', ' '],
];

type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let terminal = ratatui::init();
    let app = App::default();
    app.run(terminal).await?;
    ratatui::restore();
    Ok(())
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(mut self, mut terminal: Terminal) -> Result<()> {
        let mut interval =
            tokio::time::interval(Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND));
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => self.draw(&mut terminal)?,
                Some(Ok(event)) = events.next() =>  self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal) -> Result<()> {
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::Char('q') => self.should_quit = true,
                _ => {}
            }
        }
    }
}

