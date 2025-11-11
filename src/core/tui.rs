use crossterm::{
    event::{poll, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

use crate::{
    core::{difficulty::DIFFICULTY, events::handle_inputs, state::State},
    view::app::render_app,
};

////////////////////////////////////////

pub fn run_tui(difficulty: Option<DIFFICULTY>, is_full_screen: bool) -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = State::new(difficulty, is_full_screen);
    let res = main_loop(&mut terminal, &mut state);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

////////////////////

fn main_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    state: &mut State,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| render_app(f, state))?;

        if poll(Duration::from_secs(1))? && handle_inputs(state)? {
            return Ok(());
        }
    }
}
