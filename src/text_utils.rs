use crate::Config;
use ratatui::prelude::{Line, Stylize};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(config: &Config) -> Box<Vec<Line<'static>>> {
    let file = File::open(config.file.clone()).expect("no such file");
    let buf = BufReader::new(file);
    let textloglines = buf
        .lines()
        .map(|l| stylize(Line::from(l.unwrap())))
        .collect();
    Box::new(textloglines)
}

pub fn stylize<'a>(l: Line<'a>) -> Line<'a> {
    l.clone().red()
}
