use std::{env, io::Stdout, time::Duration};

use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Flex, Layout, Size},
    Frame,
};

use tokio::time::interval;
use util::lcm_of_multiple;
use widget::{crab::Crab, hint::Hint, track::Track};

mod util;

mod widget {
    pub mod crab;
    pub mod hint;
    pub mod track;
}

type Tracks = [Option<u32>; 4];
const DEF_TRACKS: Tracks = [Some(15), Some(30), Some(60), Some(120)];
const TRACK_NUM: usize = DEF_TRACKS.len();

const DEF_TICK_RATE: f32 = 90.;

type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let (tracks, aqua) = get_tracks();
    let use_def_tracks = tracks.iter().all(|t| t.is_none());
    let tracks = if use_def_tracks { DEF_TRACKS } else { tracks };

    let terminal = ratatui::init();
    let app = App::with_tracks(tracks).enable_easter_egg(aqua);
    let res = app.run(terminal).await;
    ratatui::restore();
    res
}

fn get_tracks() -> (Tracks, bool) {
    env::args()
        .skip(1)
        .enumerate()
        .filter(|(i, _)| *i < TRACK_NUM)
        .fold(([None, None, None, None], false), |mut tracks, (i, s)| {
            tracks.0[i] = if s == "44.5" || s == "445" {
                tracks.1 = true;
                Some(45)
            } else {
                Some(s.parse::<u32>().expect("argument must be a number"))
            };

            tracks
        })
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    tracks: Tracks,
    tracks_offset: [u16; TRACK_NUM],
    offset: u16,
    frame: usize,
    aqua: bool,
}

impl App {
    const REQUIRED_WIDTH: u16 = Crab::WIDTH + 1;

    pub fn with_tracks(tracks: Tracks) -> Self {
        Self {
            tracks,
            ..Default::default()
        }
    }

    pub fn enable_easter_egg(mut self, aqua: bool) -> Self {
        self.aqua = aqua;
        self
    }

    pub async fn run(mut self, mut terminal: Terminal) -> Result<()> {
        let fps = self.render_fps();

        let mut render_interval = interval(Duration::from_secs_f32(1. / fps));
        let mut tick_interval = interval(Duration::from_secs_f32(1. / DEF_TICK_RATE));

        let mut events = EventStream::new();
        let tracks = self.tracks();

        while !self.should_quit {
            tokio::select! {
                Some(Ok(event)) = events.next() =>  self.handle_event(&event),
                _ = tick_interval.tick() => {
                    if let Ok(area) = terminal.size() {
                        self.on_tick(area);
                    }
                },
                _ = render_interval.tick() => {
                    if let Ok(area) = terminal.size() {
                        self.on_render(fps);
                        terminal.try_draw(|frame| self.draw(frame, area, tracks))?;
                    }
                },
            }
        }

        Ok(())
    }

    fn tracks(&self) -> u16 {
        self.tracks.iter().fold(0, |acc, t| {
            if t.is_some() {
                acc.saturating_add(1)
            } else {
                acc
            }
        })
    }

    fn render_fps(&self) -> f32 {
        let l = self.tracks.map(|t| t.unwrap_or_default() as u64);
        lcm_of_multiple(&l)
            .map(|s| s as f32)
            .unwrap_or_else(|| self.max_fps())
            .min(240.)
    }

    fn max_fps(&self) -> f32 {
        self.tracks
            .iter()
            .copied()
            .map(|t| t.unwrap_or_default())
            .max()
            .unwrap_or(60)
            .min(240) as f32
    }

    fn on_render(&mut self, max_fps: f32) {
        self.frame = if self.frame == usize::MAX {
            1
        } else {
            self.frame + 1
        };

        self.tracks
            .iter()
            .filter_map(|track| *track)
            .enumerate()
            .for_each(|(i, fps)| {
                if self.frame % (max_fps / fps as f32) as usize == 0 {
                    self.tracks_offset[i] = self.offset;
                }
            })
    }

    #[inline]
    fn on_tick(&mut self, area: Size) {
        self.offset = if self.offset >= area.width - Crab::WIDTH {
            0
        } else {
            self.offset + 1
        };
    }

    fn draw(&self, frame: &mut Frame, size: Size, tracks: u16) -> std::io::Result<()> {
        let area = frame.area();

        let track_height = Crab::HEIGHT + 2;
        let required_height: u16 = track_height * tracks;
        if size.height < required_height || size.width < Self::REQUIRED_WIDTH {
            frame.render_widget(Hint(required_height, Self::REQUIRED_WIDTH), area);
            return Ok(());
        }

        let layout = Layout::vertical([
            Constraint::Length(track_height),
            Constraint::Length(track_height),
            Constraint::Length(track_height),
            Constraint::Length(track_height),
        ])
            .flex(Flex::Center)
            .split(area);

        self.tracks
            .iter()
            .filter_map(|track| *track)
            .enumerate()
            .for_each(|(i, track)| {
                frame.render_widget(Track(track, self.tracks_offset[i], self.aqua), layout[i]);
            });

        Ok(())
    }

    #[allow(clippy::single_match)]
    fn handle_event(&mut self, event: &Event) {
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::Char('q') => self.should_quit = true,
                _ => {}
            }
        }
    }
}
