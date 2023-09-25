use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph, Wrap,
    },
};
use std::{io::Stdout, time::TryFromFloatSecsError};

use crate::parse::Slideshow;

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;

pub fn ui(frame: &mut Frame, cache: &Slideshow) {
    cache.render_slide(frame);
}

fn calculate_layout(area: Rect) -> Vec<Vec<Rect>> {
    let main_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(0), Constraint::Length(0)])
        .split(area)
        .iter()
        .map(|&area| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Min(0), Constraint::Length(0)])
                .split(area)
                .to_vec()
        })
        .collect_vec();
    main_areas
}

impl Slideshow {
    fn render_slide(&self, frame: &mut Frame) {
        let layout = calculate_layout(frame.size());
        self.render_slide_borders(self.current_slide(), frame, layout[0][0]);
    }

    fn current_slide(&self) -> Paragraph {
        let paragraph = Paragraph::new("The Quick brown gox jumped over the lazy dog.")
            .wrap(Wrap { trim: true });

        paragraph
    }

    fn render_slide_borders(&self, paragraph: Paragraph, frame: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title(
                Title::from("Zi Hao Liang")
                    .position(Position::Bottom)
                    .alignment(Alignment::Left),
            )
            .title(
                Title::from(format!("{} / {}", self.index + 1, self.slides.len()))
                    .position(Position::Bottom)
                    .alignment(Alignment::Right),
            );
        frame.render_widget(paragraph.block(block), area)
    }
}
