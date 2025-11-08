use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{core::state::State, view::grid::TEXT_TO_FILL_WRONG_FG};

////////////////////////////////////////

const TEXT_FG: Color = Color::DarkGray;

////////////////////////////////////////

pub fn render_instructions(frame: &mut Frame, state: &State, area: Rect) {
    let size = frame.area();

    // Layout

    let nb_cols = 2;
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(1, nb_cols); nb_cols as usize])
        .split(area);

    let [left, right] = *cols else {
        panic!("Expected 2 columns");
    };

    // Render

    render_left(frame, left, state);
    moving(frame, right);
}

////////////////////

fn render_left<'a>(frame: &mut Frame, area: Rect, state: &State) {
    // Layout

    let nb_rows = 2;
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, nb_rows); nb_rows as usize])
        .split(area);
    let [top, bottom] = *rows else {
        panic!("Expected 2 rows");
    };

    // Render

    filling(frame, top, state);
    moving(frame, bottom);
}

////////////////////

fn filling<'a>(frame: &mut Frame, area: Rect, state: &State) {
    // TODO use state to put clear cell in red if wrong

    // Main frame

    let text = String::from("Filling values");
    let block = Block::new().borders(Borders::ALL).title(text);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);
    let nb_rows = 2;
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, nb_rows); nb_rows as usize])
        .split(inner_block);

    // Change
    let change_text = String::from("1-9 → Change");
    let change_paragraph = Paragraph::new(change_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(change_paragraph, rows[0]);

    // Clear
    let clear_text = String::from("0 / Backspace → Clear");
    let clear_style = if state.is_solved == Some(false) {
        Style::default().fg(TEXT_TO_FILL_WRONG_FG)
    } else {
        Style::default().fg(TEXT_FG)
    };
    let clear_paragraph = Paragraph::new(clear_text)
        .style(clear_style)
        .alignment(Alignment::Left);
    frame.render_widget(clear_paragraph, rows[1]);
}

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

// fn app<'a>(state: State) {
//     // use state to put new game in green if done
// }
