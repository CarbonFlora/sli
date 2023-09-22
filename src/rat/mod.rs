pub mod handler;
pub mod ui;

use crate::{parse::Slideshow, rat::ui::ui};

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::{stdout, Stdout};

// These type aliases are used to make the code more readable by reducing repetition of the generic
// types. They are not necessary for the functionality of the code.
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub fn tui() -> Result<()> {
    let mut terminal = setup_terminal()?; //init
    let result = run(&mut terminal); //this is the runtime loop.
    restore_terminal(terminal)?; //cleanup

    if let Err(err) = result {
        eprintln!("{err:?}");
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?; //this effectively exists to clear the screen.
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn run(terminal: &mut Terminal) -> Result<()> {
    let mut cache = Slideshow::from_cli()?;
    loop {
        terminal.draw(ui)?;
        if Slideshow::handle_events(&mut cache)?.is_break() {
            return Ok(());
        }
    }
}

fn restore_terminal(mut terminal: Terminal) -> Result<()> {
    disable_raw_mode()?; //this re-enables the default actions of the terminal.
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
