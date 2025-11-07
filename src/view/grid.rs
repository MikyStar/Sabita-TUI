use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};
use sabita::core::constants::LENGTH_DIMENSION;

use crate::core::state::{State, LENGTH_USIZE};

////////////////////////////////////////

const DARK_CELL_BG: Color = Color::Black;
const LIGHT_CELL_BG: Color = Color::Rgb(15, 15, 15);
const SELECTED_CELL_BG: Color = Color::Rgb(30, 30, 30);

const TEXT_TO_FILL_FG: Color = Color::Yellow;
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
            if cell_value_fillable != 0 {
                text = cell_value_fillable.to_string();
            }

            if col == 3 || col == 6 {
                col_should_black = !col_should_black;
            }

            // Text style
            let mut style = Style::default().fg(TEXT_STATIC_FG);
            if cell_value_original == 0 {
                style = style.fg(TEXT_TO_FILL_FG)
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

            let cell = Paragraph::new(text)
                .style(style)
                .alignment(Alignment::Center);

            f.render_widget(cell, cols[col]);
        }
    }
}
