use crate::Config;
use ratatui::prelude::{Line, Stylize};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(config: Config) -> Result<Vec<Line<'static>>, std::io::Error> {
    let buf = BufReader::new(File::open(config.input_file_path)?);
    let textloglines: Vec<Line> = buf.lines().map(|s| stylize(s.unwrap())).collect();
    Ok(textloglines)
}

pub fn stylize<'a>(s: String) -> Line<'a> {
    if s.contains("Section ") {
        Line::from(s).clone().white()
    } else {
        Line::from(s).clone().dark_gray()
    }
}
