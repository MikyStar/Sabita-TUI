use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::core::state::State;

////////////////////////////////////////

#[derive(Clone)]
pub struct ConfirmationDialogData<'a> {
    pub title: &'a str,
    pub description: &'a str,

    pub callbacks: ConfirmationDialogCallbacks,
}

#[derive(Clone)]
pub struct ConfirmationDialogCallbacks {
    pub on_confirm: Rc<dyn Fn(&mut State)>,
    pub on_cancel: Rc<dyn Fn(&mut State)>,
}

////////////////////////////////////////

pub fn render_confirmation_dialog(frame: &mut Frame, area: Rect, state: &mut State) {
    let ConfirmationDialogData {
        title, description, ..
    } = state.confirmation_dialog_data.clone().unwrap();

    // Layout

    let block = Block::new().borders(Borders::ALL).title(title);
    frame.render_widget(&block, area);

    let inner_block = block.inner(area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .margin(2)
        .split(inner_block);
    let [top, bottom] = *rows else {
        panic!("Expected 2 rows");
    };

    // Render
    let desc_paragraph = Paragraph::new(description)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);
    frame.render_widget(desc_paragraph, top);

    render_buttons(frame, bottom);
}

pub fn get_popup_area(area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(20)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    area
}

////////////////////

fn render_buttons(frame: &mut Frame, area: Rect) {
    let nb_rows = 2;

    // Main frame

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(1, nb_rows); nb_rows as usize])
        .split(area);

    // Confirm
    let confirm_text = String::from("Yes (y)");
    let confirm_paragraph = Paragraph::new(confirm_text)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center);
    frame.render_widget(confirm_paragraph, cols[0]);

    // Cancel
    let cancel_text = String::from("No (n)");
    let cancel_paragraph = Paragraph::new(cancel_text)
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center);
    frame.render_widget(cancel_paragraph, cols[1]);
}
