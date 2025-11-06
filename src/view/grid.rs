use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};
use sabita::core::constants::LENGTH_DIMENSION;

use crate::core::state::{State, LENGTH_USIZE};

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

            let cell_value = state.grid[row][col];
            let text = if let Some(num) = cell_value {
                num.to_string()
            } else {
                " ".to_string()
            };

            if col == 3 || col == 6 {
                col_should_black = !col_should_black;
            }

            let mut style = Style::default().fg(Color::White);
            if is_selected {
                style = style.bg(Color::Blue).add_modifier(Modifier::BOLD);
            } else {
                if !col_should_black {
                    style = style.bg(Color::DarkGray);
                } else {
                    style = style.bg(Color::Black);
                }
            }

            let cell = Paragraph::new(text)
                .style(style)
                .alignment(Alignment::Center);

            f.render_widget(cell, cols[col]);
        }
    }
}
