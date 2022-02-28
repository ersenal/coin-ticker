use tui::layout::{Constraint, Direction, Layout, Rect};

pub struct Grid {
    cell_width: u16,
    cell_height: u16,
}

impl Grid {
    pub fn with_cell_size(cell_width: u16, cell_height: u16) -> Self {
        Self {
            cell_width,
            cell_height,
        }
    }

    pub fn split(&self, area: Rect) -> Vec<Rect> {
        let cols = area.width / self.cell_width;
        let rows = area.height / self.cell_height;

        let col = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                (0..cols)
                    .map(|_| Constraint::Ratio(1, cols as u32))
                    .collect::<Vec<Constraint>>(),
            );

        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                (0..rows)
                    .map(|_| Constraint::Ratio(1, rows as u32))
                    .collect::<Vec<Constraint>>(),
            )
            .split(area)
            .into_iter()
            .flat_map(|row| col.split(row))
            .collect()
    }
}
