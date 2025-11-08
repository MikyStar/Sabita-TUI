use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::core::state::State;

////////////////////////////////////////

const TEXT_FG: Color = Color::DarkGray;

////////////////////////////////////////

pub fn render_instructions(frame: &mut Frame, state: &State, area: Rect) {
    let size = frame.area();

    let nb_cols = 2;

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(1, nb_cols); nb_cols as usize])
        .split(area);

    moving(frame, cols[0]);
    moving(frame, cols[1]);
}

////////////////////

fn moving<'a>(frame: &mut Frame, area: Rect) {
    // Main frame

    let text = String::from("Moving");
    let block = Block::new().borders(Borders::ALL).title(text);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);
    let nb_rows = 2;
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, nb_rows); nb_rows as usize])
        .split(inner_block);

    // Directions
    let directions_text = String::from("Arrow keys / hjkl → Move");
    let directions_paragraph = Paragraph::new(directions_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(directions_paragraph, rows[0]);

    // Cycle
    let cycle_text = String::from("Tab / Shift+Tab → Cycle");
    let cycle_paragraph = Paragraph::new(cycle_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(cycle_paragraph, rows[1]);
}
//
// fn filling<'a>(state: State) -> Rect<'a> {
//     // use state to put clear cell in red if wrong
// }
//
// fn app<'a>(state: State) -> Rect<'a> {
//     // use state to put new game in green if done
// }
