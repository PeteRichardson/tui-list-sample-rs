use crate::Config;
use ratatui::prelude::{Line, Stylize};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(config: &Config) -> Vec<Line<'static>> {
    let file = File::open(config.file.clone()).expect("no such file");
    let buf = BufReader::new(file);
    let textloglines = buf.lines().map(|s| stylize(s.unwrap())).collect();
    textloglines
}

pub fn stylize<'a>(s: String) -> Line<'a> {
    if s.contains("use ") {
        Line::from(s).clone().white()
    } else {
        Line::from(s).clone().dark_gray()
    }
}
