use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

////////////////////////////////////////

/// Create a Paragraph that centers `title` horizontally and vertically inside `area`.
/// `has_borders` should be true if you render the Paragraph with `Block::default().borders(Borders::ALL)`.
pub fn make_cell<'a>(title: &'a str, area: Rect, has_borders: bool) -> Paragraph<'a> {
    // Count content lines (handle multi-line titles)
    let content_lines = title.lines().count().max(1);

    // Compute inner height available for content (subtract border rows if any)
    let inner_height = {
        let h = area.height as i32;
        let subtract = if has_borders { 2 } else { 0 }; // top + bottom borders
                                                        // floor at 0 to avoid underflow
        (h - subtract).max(0) as usize
    };

    // Compute top padding inside the inner area to vertically center the content.
    // Use floor so extra row (when difference is odd) will be below the content.
    let pad_top = if inner_height > content_lines {
        (inner_height - content_lines) / 2
    } else {
        0
    };

    // Build lines: pad_top empty lines, then each content line
    let mut lines: Vec<Line<'a>> = Vec::with_capacity(pad_top + content_lines);
    for _ in 0..pad_top {
        lines.push(Line::from(""));
    }
    for ln in title.lines() {
        lines.push(Line::from(ln));
    }

    let text = Text::from(lines);

    Paragraph::new(text)
        .block(Block::default().borders(if has_borders {
            Borders::ALL
        } else {
            Borders::NONE
        }))
        .alignment(Alignment::Center) // horizontal centering
        .style(Style::default())
}
