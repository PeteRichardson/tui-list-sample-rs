use crate::Config;
use ratatui::prelude::{Line, Stylize};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(config: &Config) -> Box<Vec<Line<'static>>> {
    let file = File::open(config.file.clone()).expect("no such file");
    let buf = BufReader::new(file);
    let textloglines = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| Line::from(l).red())
        .collect();
    Box::new(textloglines)
}

// fn apply_style(l: &mut Line) -> Line {
//     l
// }

// pub fn stylize(unstyled_lines: &mut Vec<Line<'static>>) -> Box<Vec<Line<'static>>> {
//     unstyled_lines.iter_mut().map(apply_style).collect()
// }
