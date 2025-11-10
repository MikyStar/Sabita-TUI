use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    core::{state::State, time::seconds_to_hr},
    view::grid::{TEXT_TO_FILL_GOOD_FG, TEXT_TO_FILL_WRONG_FG},
};

////////////////////////////////////////

const TEXT_FG: Color = Color::DarkGray;

pub const FILLING: &str = "Filling values";
pub const CHANGE_VALUE: &str = "1-9 → Change";
pub const CLEAR_VALUE: &str = "0 / Backspace / Delete → Clear";

pub const MOVING: &str = "Moving";
pub const MOVE: &str = "Arrow keys / hjkl → Move";
pub const CYCLE: &str = "Tab / Shift+Tab → Cycle";

pub const APP: &str = "App";
pub const NEW: &str = "n → New grid";
pub const RESET: &str = "r → Reset grid";
pub const SOLVE: &str = "s → Solve grid";
pub const CHANGE_DIFFICULTY: &str = "+ / - → Change difficulty";
pub const FULLSCREEN: &str = "f → Toggle fullscreen";
pub const ESCAPE: &str = "q / Esc → Quit app";

////////////////////////////////////////

pub fn render_instructions(frame: &mut Frame, area: Rect, state: &State) {
    // Layout

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(area);
    let [top, bottom] = *rows else {
        panic!("Expected 2 rows");
    };

    // Render

    infos(frame, top, state);
    controls(frame, bottom, state);
}

////////////////////

fn infos(frame: &mut Frame, area: Rect, state: &State) {
    // Main frame
    let nb_cols = 3;

    let text = String::from("Infos");
    let block = Block::new().borders(Borders::ALL).title(text);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);
    let rows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(1, nb_cols); nb_cols as usize])
        .split(inner_block);

    // Timer
    let time = if let Some(end) = state.solved_at {
        let diff = end - state.start;
        seconds_to_hr(diff)
    } else {
        seconds_to_hr(state.start.elapsed())
    };
    let timer_text = format!("Elapsed → {time}");
    let timer_paragraph = Paragraph::new(timer_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Center);
    frame.render_widget(timer_paragraph, rows[0]);

    // Difficulty
    let difficulty_text = format!("Difficulty → {}", state.difficulty);
    let difficulty_paragraph = Paragraph::new(difficulty_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Center);
    frame.render_widget(difficulty_paragraph, rows[1]);

    // Streak
    let streak_text = format!("Streak → {}", state.streak);
    let streak_paragraph = Paragraph::new(streak_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Center);
    frame.render_widget(streak_paragraph, rows[2]);
}

pub fn controls(frame: &mut Frame, area: Rect, state: &State) {
    let nb_cols = 2;

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(1, nb_cols); nb_cols as usize])
        .split(area);

    let [left, right] = *cols else {
        panic!("Expected 2 columns");
    };

    render_left(frame, left, state);
    app(frame, right, state);
}

fn render_left(frame: &mut Frame, area: Rect, state: &State) {
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

fn filling(frame: &mut Frame, area: Rect, state: &State) {
    let nb_rows = 2;

    // Main frame

    let text = String::from(FILLING);
    let block = Block::new().borders(Borders::ALL).title(text);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, nb_rows); nb_rows as usize])
        .split(inner_block);

    // Change
    let change_text = String::from(CHANGE_VALUE);
    let change_paragraph = Paragraph::new(change_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(change_paragraph, rows[0]);

    // Clear
    let clear_text = String::from(CLEAR_VALUE);
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

fn moving(frame: &mut Frame, area: Rect) {
    // Main frame
    let nb_rows = 2;

    let text = String::from(MOVING);
    let block = Block::new().borders(Borders::ALL).title(text);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, nb_rows); nb_rows as usize])
        .split(inner_block);

    // Directions
    let directions_text = String::from(MOVE);
    let directions_paragraph = Paragraph::new(directions_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(directions_paragraph, rows[0]);

    // Cycle
    let cycle_text = String::from(CYCLE);
    let cycle_paragraph = Paragraph::new(cycle_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(cycle_paragraph, rows[1]);
}

fn app(frame: &mut Frame, area: Rect, state: &State) {
    // Main frame
    let nb_rows = 6;

    let text = String::from(APP);
    let block = Block::new().borders(Borders::ALL).title(text);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, nb_rows); nb_rows as usize])
        .split(inner_block);

    // New game
    let new_grid_text = String::from(NEW);
    let new_grid_style = if state.is_solved == Some(true) {
        Style::default().fg(TEXT_TO_FILL_GOOD_FG)
    } else {
        Style::default().fg(TEXT_FG)
    };
    let new_grid_paragraph = Paragraph::new(new_grid_text)
        .style(new_grid_style)
        .alignment(Alignment::Left);
    frame.render_widget(new_grid_paragraph, rows[0]);

    // Reset
    let reset_text = String::from(RESET);
    let reset_paragraph = Paragraph::new(reset_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(reset_paragraph, rows[1]);

    // Solve
    let solve_text = String::from(SOLVE);
    let solve_paragraph = Paragraph::new(solve_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(solve_paragraph, rows[2]);

    // Difficulty
    let difficulty_text = String::from(CHANGE_DIFFICULTY);
    let difficulty_paragraph = Paragraph::new(difficulty_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(difficulty_paragraph, rows[3]);

    // Zen
    let zen_text = String::from(FULLSCREEN);
    let zen_paragraph = Paragraph::new(zen_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(zen_paragraph, rows[4]);

    // Quit
    let quit_text = String::from(ESCAPE);
    let quit_paragraph = Paragraph::new(quit_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(quit_paragraph, rows[5]);
}
