use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::{ops::ControlFlow, time::Duration};

use crate::parse::Slideshow;

impl Slideshow {
    pub fn handle_events(&mut self) -> Result<ControlFlow<()>> {
        if !event::poll(Duration::from_millis(100))? {
            return Ok(ControlFlow::Continue(()));
        }

        let event = event::read()?;

        if let Event::Key(key) = event {
            return Ok(self.execute_key(key));
        }

        Ok(ControlFlow::Continue(()))
    }

    fn execute_key(&mut self, key_event: KeyEvent) -> ControlFlow<()> {
        match key_event.code {
            KeyCode::Char('q') => return ControlFlow::Break(()),
            KeyCode::Char('j') | KeyCode::Char('l') | KeyCode::Char(' ') => self.next_slide(),
            KeyCode::Char('k') | KeyCode::Char('h') | KeyCode::Backspace => self.previous_slide(),
            _ => (),
        };
        return ControlFlow::Continue(());
    }

    fn next_slide(&mut self) {
        if self.index < self.slides.len() {
            self.index += 1;
        }
    }

    fn previous_slide(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }
}
