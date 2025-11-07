use std::io;

use crate::core::state::State;

use crossterm::event::{self, Event, KeyCode};

////////////////////////////////////////

/// Returns 'true' if TUI should stop
pub fn handle_keyboard_events(state: &mut State) -> io::Result<bool> {
    let event = event::read()?;

    if let Event::Key(key) = event {
        match key.code {
            // Moving
            KeyCode::Up | KeyCode::Char('k') => state.move_cell_top(),
            KeyCode::Down | KeyCode::Char('j') => state.move_cell_bottom(),
            KeyCode::Left | KeyCode::Char('h') => state.move_cell_left(),
            KeyCode::Right | KeyCode::Char('l') => state.move_cell_right(),

            KeyCode::Tab => state.move_next_cell(),
            KeyCode::BackTab => state.move_previous_cell(),

            // Filling
            KeyCode::Char(c) if c.is_ascii_digit() => {
                if let Some(d) = c.to_digit(10) {
                    state.set_number(d as u8);
                }
            }
            KeyCode::Backspace | KeyCode::Delete | KeyCode::Char('0') => state.clear_cell(),

            // App
            KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
            KeyCode::Char('r') => state.reset(),
            _ => {}
        }
    }

    Ok(false)
}
