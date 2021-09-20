use rayon::prelude::*;
use termion::{color, style};

pub struct Report {
    lines: Vec<Line>,
    start: bool,
}

impl Report {
    pub fn new() -> Report {
        let lines = Vec::new();
        Report { lines, start: true }
    }

    pub fn insert(&mut self, title: String, text: String) {
        self.lines.push(Line::new(title, text));
    }

    pub fn render(&mut self) {
        if !self.start {
            self.clear_screen();
        }
        self.save_cursor();

        let title_col_size = self
            .lines
            .par_iter()
            .map(|line| line.title.len())
            .reduce(|| 0, |a, b| if a > b { a } else { b });

        for line in self.lines.iter() {
            let padding = title_col_size - line.title.len() + 4;
            line.render(padding);
        }

        self.start = false;
        self.lines.clear();
    }

    fn save_cursor(&self) {
        print!("{}", termion::cursor::Save);
    }

    fn clear_screen(&self) {
        print!(
            "{}{}",
            termion::cursor::Restore,
            termion::clear::AfterCursor
        );
    }
}

pub struct Line {
    title: String,
    text: String,
}

impl Line {
    pub fn new(title: String, text: String) -> Line {
        Line { title, text }
    }

    fn render(&self, padding: usize) {
        let padding_str = " ".repeat(padding);
        println!(
            "{}{}{}{}{} {}{}",
            padding_str,
            style::Bold,
            color::Fg(color::Blue),
            self.title,
            style::Reset,
            color::Fg(color::White),
            self.text
        );
    }
}
