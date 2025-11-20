use std::{io, rc::Rc};

use crate::core::state::{State, LENGTH_USIZE};

use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::layout::Rect;
use sabita::core::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};

////////////////////////////////////////

/// Returns 'true' if TUI should stop
pub fn handle_inputs(state: &mut State) -> io::Result<bool> {
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
            KeyCode::Char('y') => {
                if let Some(dialog_data) = state.confirmation_dialog_data.as_ref() {
                    let on_confirm = Rc::clone(&dialog_data.callbacks.on_confirm);
                    on_confirm(state)
                }
            }
            KeyCode::Char('n') => match state.confirmation_dialog_data.as_ref() {
                Some(dialog_data) => {
                    let on_cancel = Rc::clone(&dialog_data.callbacks.on_cancel);
                    on_cancel(state);
                }
                None => state.ask_new_game(),
            },
            KeyCode::Char('r') => state.ask_reset(),
            KeyCode::Char('+') => state.increase_difficulty(),
            KeyCode::Char('-') => state.decrease_difficulty(),
            KeyCode::Char('s') => state.ask_solve(),
            KeyCode::Char('f') => state.toggle_fullscreen(),
            KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
            _ => {}
        }
    } else if let Event::Mouse(mouse) = event {
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            if let Some(clickable_area) = state.clickable_area {
                if state.confirmation_dialog_data.is_some() {
                    click_on_confirmation_modal_action(
                        mouse.column,
                        mouse.row,
                        clickable_area,
                        state,
                    );
                } else if let Some((row, col)) =
                    screen_pos_to_grid_pos(mouse.column, mouse.row, clickable_area, state)
                {
                    state.cursor_row = row;
                    state.cursor_col = col;
                }
            }
        }
    }

    Ok(false)
}

////////////////////

fn screen_pos_to_grid_pos(
    x: u16,
    y: u16,
    grid_area: Rect,
    state: &State,
) -> Option<(usize, usize)> {
    let margin = 2;

    // Check if click is within the grid area
    if x < grid_area.x + margin || y < grid_area.y + margin {
        return None;
    }

    let rel_x = x.saturating_sub(grid_area.x + margin);
    let rel_y = y.saturating_sub(grid_area.y + margin);

    // Calculate cell dimensions
    let grid_width = grid_area.width.saturating_sub(margin * 2);
    let grid_height = grid_area.height.saturating_sub(margin * 2);

    let cell_width = grid_width / LENGTH_DIMENSION as u16;
    let cell_height = grid_height / LENGTH_DIMENSION as u16;

    let col = (rel_x / cell_width) as usize;
    let row = (rel_y / cell_height) as usize;

    let is_clickable = state.original_grid.values[row][col] == TO_BE_SOLVED;

    if is_clickable && row <= LENGTH_USIZE && col <= LENGTH_USIZE {
        Some((row, col))
    } else {
        None
    }
}

fn click_on_confirmation_modal_action(x: u16, y: u16, modal_area: Rect, state: &mut State) {
    let margin = 0;

    if x < modal_area.x + margin
        || x > modal_area.width + modal_area.x
        || y < modal_area.y + margin
        || y > modal_area.y + modal_area.height
    {
        return;
    }

    let rel_x = x.saturating_sub(modal_area.x + margin);

    let modal_width = modal_area.width.saturating_sub(margin * 2);
    let button_width = modal_width / 2;

    let dialog_data = state.confirmation_dialog_data.as_ref().unwrap();

    if rel_x < button_width {
        let on_confirm = Rc::clone(&dialog_data.callbacks.on_confirm);
        on_confirm(state);
    } else {
        let on_cancel = Rc::clone(&dialog_data.callbacks.on_cancel);
        on_cancel(state);
    }
}
