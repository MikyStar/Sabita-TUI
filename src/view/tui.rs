use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event as CEvent, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

////////////////////////////////////////

pub fn run_tui() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?;
    crossterm::execute!(stdout, crossterm::cursor::Hide)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(crossterm::cursor::Show)?;
    stdout.execute(crossterm::terminal::LeaveAlternateScreen)?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.area();
            render_grid(size, f);
        })?;

        // poll for events with a timeout so the UI can react to resize
        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                CEvent::Key(key) => match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                },
                CEvent::Resize(_, _) => {
                    // will redraw on next loop iteration
                }
                _ => {}
            }
        }
    }
}

/// Render a 9x9 square grid where each cell is the same size. The grid's width
/// is computed to use 95% of the available terminal width (if possible). If the
/// terminal is too short vertically, the cell size is reduced so the whole grid
/// fits vertically.
fn render_grid(area: Rect, f: &mut ratatui::Frame) {
    let total_cols = area.width; // terminal width in characters
    let total_rows = area.height; // terminal height in characters

    // Target grid width = 95% of available width
    let target_grid_width = ((total_cols as f32) * 0.95).floor() as u16;

    // Each cell width (integer division). Guarantee at least 1.
    let mut cell_w = target_grid_width / 9;
    if cell_w == 0 {
        cell_w = 1;
    }

    // We want square cells, so height == width. But terminal may not have enough rows.
    // Calculate the maximum cell height that would fit vertically.
    let max_cell_h = total_rows / 9;

    // Final cell size is min(cell_w, max_cell_h)
    let mut cell_h = cell_w.min(max_cell_h);
    if cell_h == 0 {
        cell_h = 1;
    }

    // If reducing cell_h below cell_w (because of height constraints), we must also
    // recalc grid width so it doesn't overflow horizontally. This keeps the cells
    // square while ensuring the grid fits both dimensions.
    let grid_width = cell_w * 9;
    if grid_width > total_cols {
        // shrink cell_w to fit horizontally
        cell_w = total_cols / 9;
        if cell_w == 0 {
            cell_w = 1;
        }
        // recompute cell_h to keep squares but not exceed height
        cell_h = cell_w.min(max_cell_h);
        if cell_h == 0 {
            cell_h = 1;
        }
    }

    // Recompute final grid dimensions
    let final_grid_width = cell_w * 9;
    let final_grid_height = cell_h * 9;

    // Compute offsets to center the grid in the terminal
    let offset_x = if total_cols > final_grid_width {
        (total_cols - final_grid_width) / 2
    } else {
        0
    };
    let offset_y = if total_rows > final_grid_height {
        (total_rows - final_grid_height) / 2
    } else {
        0
    };

    // Draw a faint outer block that represents the grid area
    let outer = Rect::new(offset_x, offset_y, final_grid_width, final_grid_height);
    let outer_block = Block::default().borders(Borders::NONE).title(Span::styled(
        "9x9 grid",
        Style::default().add_modifier(Modifier::BOLD),
    ));
    f.render_widget(outer_block, outer);

    // Render each cell
    for r in 0..9u16 {
        for c in 0..9u16 {
            let x = offset_x + c * cell_w;
            let y = offset_y + r * cell_h;

            // Ensure we don't create an area that extends outside the terminal.
            let width = if x + cell_w > area.width {
                area.width - x
            } else {
                cell_w
            };
            let height = if y + cell_h > area.height {
                area.height - y
            } else {
                cell_h
            };

            // If width or height is zero, skip drawing this cell.
            if width == 0 || height == 0 {
                continue;
            }

            let cell_area = Rect::new(x, y, width, height);

            // Create a block with a thin border and an index label centered
            let title = format!("{}", r * 9 + c + 1); // 1..81 numbering
            let paragraph = Paragraph::new(Span::raw(title))
                .block(Block::default().borders(Borders::ALL))
                .alignment(ratatui::layout::Alignment::Center)
                .style(Style::default());

            f.render_widget(paragraph, cell_area);
        }
    }
}
