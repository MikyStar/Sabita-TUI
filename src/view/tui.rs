use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event as CEvent, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use super::grid::render_grid;

////////////////////////////////////////

pub fn run_tui() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(crossterm::terminal::EnterAlternateScreen)?;
    crossterm::execute!(stdout, crossterm::cursor::Hide)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let res = make_app(&mut terminal);

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

fn make_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
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
