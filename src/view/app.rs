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

    let constraints = if state.is_fullscreen {
        [
            Constraint::Percentage(0),
            Constraint::Percentage(100),
            Constraint::Percentage(0),
        ]
    } else {
        [
            Constraint::Percentage(5),
            Constraint::Percentage(75),
            Constraint::Percentage(20),
        ]
    };

    // Create main layout with padding
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(constraints)
        .split(size);
    let [top, center, bottom] = *chunks else {
        panic!("Expected 3 rows");
    };
    let grid_dimension = center.width;
    let grid_area = center_rect(center, grid_dimension, grid_dimension);

    if state.is_fullscreen {
        render_grid(frame, state, grid_area);
    } else {
        let title = title();
        frame.render_widget(title, top);

        render_grid(frame, state, grid_area);

        render_instructions(frame, bottom, state);
    }
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
