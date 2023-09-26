use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph, Wrap,
    },
};

use crate::parse::Slide;

impl Slide {
    pub fn render(&self) -> String {
        let mut out = String::new();
        for line in self.lines.iter() {
            out += line;
        }
        out
    }
}
