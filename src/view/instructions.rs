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

fn infos<'a>(frame: &mut Frame, area: Rect, state: &State) {
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
    let time = seconds_to_hr(state.start.elapsed());
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

fn filling<'a>(frame: &mut Frame, area: Rect, state: &State) {
    let nb_rows = 2;

    // Main frame

    let text = String::from("Filling values");
    let block = Block::new().borders(Borders::ALL).title(text);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);
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
    let clear_text = String::from("0 / Backspace / Delete → Clear");
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
    let nb_rows = 2;

    let text = String::from("Moving");
    let block = Block::new().borders(Borders::ALL).title(text);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);
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

fn app<'a>(frame: &mut Frame, area: Rect, state: &State) {
    // Main frame
    let nb_rows = 6;

    let text = String::from("App");
    let block = Block::new().borders(Borders::ALL).title(text);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, nb_rows); nb_rows as usize])
        .split(inner_block);

    // New game
    let new_grid_text = String::from("n → New grid");
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
    let reset_text = String::from("r → Reset grid");
    let reset_paragraph = Paragraph::new(reset_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(reset_paragraph, rows[1]);

    // Solve
    let solve_text = String::from("s → Solve grid");
    let solve_paragraph = Paragraph::new(solve_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(solve_paragraph, rows[2]);

    // Difficulty
    let difficulty_text = String::from("+ / - → Change difficulty");
    let difficulty_paragraph = Paragraph::new(difficulty_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(difficulty_paragraph, rows[3]);

    // Zen
    let zen_text = String::from("f → Toggle fullscreen");
    let zen_paragraph = Paragraph::new(zen_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(zen_paragraph, rows[4]);

    // Quit
    let quit_text = String::from("q / Esc → Quit app");
    let quit_paragraph = Paragraph::new(quit_text)
        .style(Style::default().fg(TEXT_FG))
        .alignment(Alignment::Left);
    frame.render_widget(quit_paragraph, rows[5]);
}
