use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph},
    Frame,
};
use sabita::core::constants::{LENGTH_DIMENSION, TO_BE_SOLVED};

use crate::core::state::{State, LENGTH_USIZE};

////////////////////////////////////////

const DARK_CELL_BG: Color = Color::Black;
const LIGHT_CELL_BG: Color = Color::Rgb(15, 15, 15);
const SELECTED_CELL_BG: Color = Color::Rgb(30, 30, 30);

const TEXT_TO_FILL_WIP_FG: Color = Color::Yellow;
const TEXT_TO_FILL_WRONG_FG: Color = Color::Red;
const TEXT_TO_FILL_GOOD_FG: Color = Color::Green;
const TEXT_STATIC_FG: Color = Color::White;

////////////////////////////////////////

pub fn render_grid(f: &mut Frame, state: &State, area: Rect) {
    // Create 9 rows
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Ratio(1, LENGTH_DIMENSION.into());
            LENGTH_DIMENSION.into()
        ])
        .split(area);

    let mut col_should_black = true;

    for row in 0..LENGTH_USIZE {
        if row == 3 || row == 6 {
            col_should_black = !col_should_black;
        }

        // Create 9 columns
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, LENGTH_DIMENSION.into());
                LENGTH_DIMENSION.into()
            ])
            .split(rows[row]);

        for col in 0..LENGTH_USIZE {
            let is_selected = state.cursor_row == row && state.cursor_col == col;

            let cell_value_fillable = state.grid_to_solve.values[row][col];
            let cell_value_original = state.original_grid.values[row][col];

            let mut text = String::from("-");
            if cell_value_fillable != TO_BE_SOLVED {
                text = cell_value_fillable.to_string();
            }

            if col == 3 || col == 6 {
                col_should_black = !col_should_black;
            }

            // Text style
            let mut style = Style::default().fg(TEXT_STATIC_FG);
            if cell_value_original == TO_BE_SOLVED {
                if let Some(status) = state.is_solved {
                    match status {
                        true => style = style.fg(TEXT_TO_FILL_GOOD_FG),
                        false => style = style.fg(TEXT_TO_FILL_WRONG_FG),
                    }
                } else {
                    style = style.fg(TEXT_TO_FILL_WIP_FG)
                }
            }
            if is_selected {
                style = style.bg(SELECTED_CELL_BG).add_modifier(Modifier::BOLD);
            } else {
                if !col_should_black {
                    style = style.bg(LIGHT_CELL_BG);
                } else {
                    style = style.bg(DARK_CELL_BG);
                }
            }

            // Create a block with the background style
            let block = Block::default().style(style);

            // Calculate the inner area for centering
            let inner = block.inner(cols[col]);

            // Render the block first
            f.render_widget(block, cols[col]);

            // Center the text vertically by calculating padding
            let text_height = 1; // Single line of text
            let available_height = inner.height;
            let vertical_padding = (available_height.saturating_sub(text_height)) / 2;

            // Create a new layout for vertical centering
            let vertical_center = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(vertical_padding),
                    Constraint::Length(text_height),
                    Constraint::Min(0),
                ])
                .split(inner);

            let cell = Paragraph::new(text)
                .style(style)
                .alignment(Alignment::Center);

            f.render_widget(cell, vertical_center[1]);
        }
    }
}
