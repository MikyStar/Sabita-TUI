use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use sabita::core::constants::PKG_NAME as SABITA_PKG_NAME;

use crate::{
    core::state::State,
    view::{grid::render_grid, instructions::render_instructions, utils::center_rect},
};

////////////////////////////////////////

pub fn render_app(frame: &mut Frame, state: &State) {
    let size = frame.area();

    // Create main layout with padding
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Ratio(2, 35),
            Constraint::Ratio(28, 35),
            Constraint::Ratio(5, 35),
        ])
        .split(size);

    let title = title();
    frame.render_widget(title, chunks[0]);

    // Calculate grid area (centered and square-ish)
    let grid_area = center_rect(chunks[1], 60, 27);
    render_grid(frame, state, grid_area);

    render_instructions(frame, state, chunks[2]);
}

////////////////////

fn title<'a>() -> Paragraph<'a> {
    let text = String::from(SABITA_PKG_NAME).to_uppercase();

    Paragraph::new(text)
        .style(
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL))
}
