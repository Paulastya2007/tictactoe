use crate::config::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    X,
    O,
}

#[derive(Clone, Copy)]
pub struct Board {
    pub cells: [[CellState; 3]; 3],
    pub cell_size: f32,
    pub x: f32,
    pub y: f32,
}

impl Board {
    pub fn new() -> Self {
        let cell_size = 150.0;
        let board_total_size = cell_size * 3.0;

        Self {
            cells: [[CellState::Empty; 3]; 3],
            cell_size,
            x: VIRTUAL_WIDTH / 2.0 - board_total_size / 2.0,
            y: VIRTUAL_HEIGHT / 2.0 - board_total_size / 2.0 + 30.0, // Slightly offset down for title
        }
    }

    pub fn draw(&self) {
        // Draw grid lines
        let color = DARK_GREY;
        let thickness = 4.0;

        for i in 1..3 {
            let offset = i as f32 * self.cell_size;
            // Vertical lines
            draw_line(
                self.x + offset,
                self.y,
                self.x + offset,
                self.y + self.cell_size * 3.0,
                thickness,
                color,
            );
            // Horizontal lines
            draw_line(
                self.x,
                self.y + offset,
                self.x + self.cell_size * 3.0,
                self.y + offset,
                thickness,
                color,
            );
        }

        // Draw cells
        for row in 0..3 {
            for col in 0..3 {
                let cell_x = self.x + col as f32 * self.cell_size;
                let cell_y = self.y + row as f32 * self.cell_size;
                match self.cells[row][col] {
                    CellState::X => {
                        let font = crate::config::get_inter_font();
                        let font_size = 120;
                        let text = "X";
                        let text_dim = measure_text(text, font, font_size, 1.0);

                        draw_text_ex(
                            text,
                            cell_x + self.cell_size / 2.0 - text_dim.width / 2.0,
                            cell_y + self.cell_size / 2.0 + text_dim.height / 2.0 - 5.0,
                            TextParams {
                                font,
                                font_size,
                                color: PRIMARY_BLUE,
                                ..Default::default()
                            },
                        );
                    }
                    CellState::O => {
                        let font = crate::config::get_inter_font();
                        let font_size = 120;
                        let text = "O";
                        let text_dim = measure_text(text, font, font_size, 1.0);

                        draw_text_ex(
                            text,
                            cell_x + self.cell_size / 2.0 - text_dim.width / 2.0,
                            cell_y + self.cell_size / 2.0 + text_dim.height / 2.0 - 5.0,
                            TextParams {
                                font,
                                font_size,
                                color: SUCCESS_GREEN,
                                ..Default::default()
                            },
                        );
                    }
                    CellState::Empty => {}
                }
            }
        }
    }

    pub fn get_cell_at(&self, mouse: Vec2) -> Option<(usize, usize)> {
        let relative_x = mouse.x - self.x;
        let relative_y = mouse.y - self.y;

        if relative_x >= 0.0
            && relative_x < self.cell_size * 3.0
            && relative_y >= 0.0
            && relative_y < self.cell_size * 3.0
        {
            let col = (relative_x / self.cell_size) as usize;
            let row = (relative_y / self.cell_size) as usize;
            return Some((row, col));
        }

        None
    }

    pub fn check_winner(&self) -> Option<CellState> {
        // Rows
        for row in 0..3 {
            if self.cells[row][0] != CellState::Empty
                && self.cells[row][0] == self.cells[row][1]
                && self.cells[row][1] == self.cells[row][2]
            {
                return Some(self.cells[row][0]);
            }
        }

        // Columns
        for col in 0..3 {
            if self.cells[0][col] != CellState::Empty
                && self.cells[0][col] == self.cells[1][col]
                && self.cells[1][col] == self.cells[2][col]
            {
                return Some(self.cells[0][col]);
            }
        }

        // Diagonals
        if self.cells[0][0] != CellState::Empty
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][2]
        {
            return Some(self.cells[0][0]);
        }
        if self.cells[0][2] != CellState::Empty
            && self.cells[0][2] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][0]
        {
            return Some(self.cells[0][2]);
        }

        None
    }

    pub fn is_full(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.cells[row][col] == CellState::Empty {
                    return false;
                }
            }
        }
        true
    }

    pub fn reset(&mut self) {
        self.cells = [[CellState::Empty; 3]; 3];
    }
}
