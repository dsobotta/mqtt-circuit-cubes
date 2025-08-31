use std::{
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

use crossterm::event::{KeyCode, KeyEventKind};
use futures::io;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Gauge, Widget},
    DefaultTerminal, Frame, Terminal,
};

pub struct BatteryCube {
    id: String,
    powerA: i16,
    powerB: i16,
    powerC: i16,
}

enum Event {
    Input(crossterm::event::KeyEvent),
    Progress(f64),
}

fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().expect("failed to get key event") {
            crossterm::event::Event::Key(key_event) => {
                tx.send(Event::Input(key_event)).expect("failed to send key event");
            }
            _ => {}
        }
    }
}

fn run_background_thread(tx: mpsc::Sender<Event>) {
    let mut progress = 0_f64;
    let increment = 0.002_f64;
    loop {
        thread::sleep(Duration::from_millis(10));
        progress += increment;
        progress = progress.clamp(0.0, 1.0);
        tx.send(Event::Progress(progress)).expect("failed to send progress");
    }
}

pub struct App {
    exit: bool,
    progress_bar_color: Color,
    background_progress: f64,
    cubes: Vec<BatteryCube>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            progress_bar_color: Color::Green,
            background_progress: 0.0_f64,
            cubes: vec![
                BatteryCube {
                    id: "Tenka0ca1".to_string(),
                    powerA: -255,
                    powerB: 0,
                    powerC: 255,
                },
                BatteryCube {
                    id: "Tenka0ca2".to_string(),
                    powerA: -128,
                    powerB: 0,
                    powerC: 128,
                },
            ],
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal, rx: mpsc::Receiver<Event>) -> io::Result<()> {
        while !self.exit {
            match rx.recv().expect("failed to receive") {
                Event::Input(key_event) => {
                    self.handle_key_event(key_event).expect("failed to handle key event");
                }
                Event::Progress(progress) => {
                    self.background_progress = progress;
                }
            }
            terminal.draw(|frame| self.draw(frame)).expect("failed to draw frame");
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => {
                    self.exit = true;
                }
                KeyCode::Char('c') => {
                    self.progress_bar_color = Color::Yellow;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vertical_layout = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [title_area, gauge_area] = vertical_layout.areas(area);
        Line::from("Process overview").bold().render(title_area, buf);

        let instructions = Line::from(vec![
            " Change color ".into(),
            "<C>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ])
        .centered();

        let block = Block::bordered()
            .title(Line::from("Background process "))
            .title_bottom(instructions)
            .border_set(border::THICK);

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .block(block)
            .label(format!("Process 1: {:.2}", self.background_progress * 100_f64))
            .ratio(self.background_progress);

        progress_bar.render(
            Rect {
                x: gauge_area.left(),
                y: gauge_area.top(),
                width: gauge_area.width,
                height: 3,
            },
            buf,
        );
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app: App = App::new();

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let tx_to_background_progress_events = event_tx.clone();
    thread::spawn(move || {
        run_background_thread(tx_to_background_progress_events);
    });

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();

    app_result
}
