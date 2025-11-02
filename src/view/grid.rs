use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders},
};

use super::cell::make_cell;

////////////////////////////////////////

/// Render a 9x9 square grid where each cell is the same size. The grid's width
/// is computed to use 95% of the available terminal width (if possible). If the
/// terminal is too short vertically, the cell size is reduced so the whole grid
/// fits vertically.
pub fn render_grid(area: Rect, frame: &mut ratatui::Frame) {
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
    frame.render_widget(outer_block, outer);

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

            let title = format!("{}", r * 9 + c + 1); // 1..81 numbering

            let cell = make_cell(&title, cell_area, true);

            frame.render_widget(cell, cell_area);
        }
    }
}
