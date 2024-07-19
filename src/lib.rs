use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::{
    Backend, Buffer,
    Constraint::{self, Percentage},
    Layout, Line, Rect, Style, Stylize, Terminal, Widget,
};
use ratatui::widgets::{Block, List, Paragraph, Wrap};
use std::time::Duration;
mod text_utils;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use text_utils::load;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    pub input_file_path: PathBuf,
}

#[derive(Debug, Default)]
pub struct App<'a> {
    state: AppState,
    //toc: HashMap<Line<'a>, usize>,
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
    pub fn new(config: Config) -> Self {
        // text is the stylized text of the log file
        let text = App::load_text(config);

        // pattern to match the section headers
        let _pattern = Regex::new(r"^[\+]+ (Section [\d\.]+)").unwrap();

        // map of section headers to their line number (first line is line 1, not 0)
        // unfortunately hashmap is unordered, so we need to retain the original order
        let mut toc_records: HashMap<Line, usize> = HashMap::new();
        for (offset, line) in text.iter().enumerate() {
            toc_records.insert(line.clone(), offset + 1);
        }
        // nav_entries used to build the nav bar
        // ideally filtered by pattern and sorted by line number
        let nav_entries: Vec<Line<'_>> = toc_records.keys().cloned().collect();

        Self {
            state: AppState::Running,
            // toc: toc_records,  // eventually will need to retain this to enable navigation
            nav: List::new(nav_entries)
                .block(Block::bordered().title("Steps"))
                .style(Style::new().white().on_black()),
            text: Paragraph::new(text)
                .block(Block::bordered().title("Log"))
                .wrap(Wrap { trim: true }),
        }
    }

    fn load_text(config: Config) -> Vec<Line<'static>> {
        load(config).unwrap()
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
        if event::poll(timeout)? && event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
                {
                    self.state = AppState::Quit;
                };
            }
        }
        Ok(())
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::Min;
        // calculate rects where widgets should be rendered
        let widget_areas: Vec<Rect> = Layout::horizontal([Percentage(15), Min(0)])
            .areas::<2>(area)
            .to_vec();
        (&self.nav).render(widget_areas[0], buf);
        (&self.text).render(widget_areas[1], buf);
    }
}
