use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;

struct App {
    grid: [[Option<u8>; 9]; 9],
    cursor_row: usize,
    cursor_col: usize,
}

impl App {
    fn new() -> App {
        App {
            grid: [[None; 9]; 9],
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    fn move_cursor(&mut self, dr: i32, dc: i32) {
        let new_row = (self.cursor_row as i32 + dr).clamp(0, 8) as usize;
        let new_col = (self.cursor_col as i32 + dc).clamp(0, 8) as usize;
        self.cursor_row = new_row;
        self.cursor_col = new_col;
    }

    fn set_number(&mut self, num: u8) {
        if num >= 1 && num <= 9 {
            self.grid[self.cursor_row][self.cursor_col] = Some(num);
        }
    }

    fn clear_cell(&mut self) {
        self.grid[self.cursor_row][self.cursor_col] = None;
    }
}

pub fn claude() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Up | KeyCode::Char('k') => app.move_cursor(-1, 0),
                KeyCode::Down | KeyCode::Char('j') => app.move_cursor(1, 0),
                KeyCode::Left | KeyCode::Char('h') => app.move_cursor(0, -1),
                KeyCode::Right | KeyCode::Char('l') => app.move_cursor(0, 1),
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    if let Some(d) = c.to_digit(10) {
                        app.set_number(d as u8);
                    }
                }
                KeyCode::Backspace | KeyCode::Delete | KeyCode::Char('0') => app.clear_cell(),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
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
    render_grid(f, app, grid_area);

    // Instructions
    let instructions =
        Paragraph::new("Arrow keys/hjkl: Move | 1-9: Enter number | 0/Backspace: Clear | q: Quit")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}

fn render_grid(f: &mut Frame, app: &App, area: Rect) {
    // Create 9 rows
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3); 9])
        .split(area);

    for row in 0..9 {
        // Create 9 columns
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(6); 9])
            .split(rows[row]);

        for col in 0..9 {
            let is_selected = app.cursor_row == row && app.cursor_col == col;
            let is_thick_right = col % 3 == 2 && col < 8;
            let is_thick_bottom = row % 3 == 2 && row < 8;

            let mut borders = Borders::ALL;
            let mut border_style = Style::default().fg(Color::DarkGray);

            // Thicker borders for 3x3 boxes
            if is_thick_right || is_thick_bottom {
                border_style = Style::default().fg(Color::White);
            }

            let cell_value = app.grid[row][col];
            let text = if let Some(num) = cell_value {
                num.to_string()
            } else {
                " ".to_string()
            };

            let mut style = Style::default().fg(Color::White);
            if is_selected {
                style = style.bg(Color::Blue).add_modifier(Modifier::BOLD);
            }

            let cell = Paragraph::new(text)
                .style(style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(borders).border_style(border_style));

            f.render_widget(cell, cols[col]);
        }
    }
}

fn center_rect(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(vertical[1])[1]
}
