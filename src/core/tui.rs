use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::{core::state::State, view::app::main_page};

////////////////////////////////////////

pub fn run_tui() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = State::new();
    let res = run_app(&mut terminal, &mut state);

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

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    state: &mut State,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| main_page(f, state))?;

        // TODO externalize events

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Up | KeyCode::Char('k') => state.move_cursor(-1, 0),
                KeyCode::Down | KeyCode::Char('j') => state.move_cursor(1, 0),
                KeyCode::Left | KeyCode::Char('h') => state.move_cursor(0, -1),
                KeyCode::Right | KeyCode::Char('l') => state.move_cursor(0, 1),
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    if let Some(d) = c.to_digit(10) {
                        state.set_number(d as u8);
                    }
                }
                KeyCode::Backspace | KeyCode::Delete | KeyCode::Char('0') => state.clear_cell(),
                _ => {}
            }
        }
    }
}
