use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    core::state::State,
    view::{grid::render_grid, utils::center_rect},
};

////////////////////////////////////////

pub fn main_page(f: &mut Frame, state: &State) {
    let size = f.area();

    // Create main layout with padding
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(27),
            Constraint::Length(3),
        ])
        .split(size);

    // Title
    let title = Paragraph::new("Sudoku")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Calculate grid area (centered and square-ish)
    let grid_area = center_rect(chunks[1], 60, 27);

    // Render the grid
    render_grid(f, state, grid_area);

    // Instructions
    let instructions = Paragraph::new(
        "Arrow keys/hjkl: Move | 1-9: Enter number | 0/Backspace: Clear | q/Esc: Close",
    )
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}
