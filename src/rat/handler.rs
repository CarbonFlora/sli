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
        return match key_event.code {
            KeyCode::Char('q') => ControlFlow::Break(()),
            _ => ControlFlow::Continue(()),
        };
    }
}
