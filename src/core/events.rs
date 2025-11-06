use std::io;

use crate::core::state::State;

use crossterm::event::{self, Event, KeyCode};

////////////////////////////////////////

/// Returns 'true' if TUI should stop
pub fn handle_keyboard_events(state: &mut State) -> io::Result<bool> {
    let event = event::read()?;

    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
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

    Ok(false)
}
