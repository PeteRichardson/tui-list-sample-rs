use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::{
    Backend, Buffer,
    Constraint::{self, Length},
    Layout, Line, Rect, Span, Style, Stylize, Terminal, Widget,
};
use ratatui::widgets::{Block, List, Paragraph, Wrap};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    pub file1: String,
    pub file2: String,
}

#[derive(Debug, Default)]

pub struct App<'a> {
    state: AppState,
    nav: List<'a>,
    text: Paragraph<'a>,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum AppState {
    #[default]
    Running, // The app is running
    Quit, // The user has requested the app to quit
}

impl App<'_> {
    pub fn new(_config: &Config) -> Self {
        let items1: Vec<&str> = vec!["Item 1", "Item 2", "Item 3"];
        let text = App::load_text();
        Self {
            state: AppState::Running,
            nav: List::new(items1).block(Block::bordered().title("Steps")), //.style(Style::new().white().on_black())
            //.wrap(Wrap { trim: true }),
            text: Paragraph::new(*text)
                .block(Block::bordered().title("Log"))
                .style(Style::new().white().on_black())
                .wrap(Wrap { trim: true }),
        }
    }

    fn load_text() -> Box<Vec<Line<'static>>> {
        Box::new(vec![
            Line::from(vec![
                Span::raw("First"),
                Span::styled("line", Style::new().green().italic()),
                ".".into(),
            ]),
            Line::from("Second line".red()),
            "Third line".into(),
        ])
    }

    /// This is the main event loop for the app.
    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.size()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    const fn is_running(&self) -> bool {
        matches!(self.state, AppState::Running)
    }

    /// Handle any events that have occurred since the last time the app was rendered.
    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 60.0);
        if event::poll(timeout)? {
            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                        && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
                    {
                        self.state = AppState::Quit;
                    };
                }
            }
        }
        Ok(())
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::Min;
        // calculate rects where widgets should be rendered
        let widget_areas: Vec<Rect> = Layout::horizontal([Length(20), Min(0)])
            .areas::<2>(area)
            .to_vec();
        (&self.nav).render(widget_areas[0], buf);
        (&self.text).render(widget_areas[1], buf);
    }
}
