use crate::config::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    X,
    O,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub state: CellState,
    pub anim_timer: f32, // 0.0 to 1.0 for scale/alpha animation
}

#[derive(Clone, Copy)]
pub struct Board {
    pub cells: [[Cell; 3]; 3],
    pub cell_size: f32,
    pub x: f32,
    pub y: f32,
    pub winning_cells: Option<[(usize, usize); 3]>,
}

impl Board {
    pub fn new() -> Self {
        let cell_size = 150.0;
        let board_total_size = cell_size * 3.0;

        Self {
            cells: [[Cell {
                state: CellState::Empty,
                anim_timer: 0.0,
            }; 3]; 3],
            cell_size,
            x: VIRTUAL_WIDTH / 2.0 - board_total_size / 2.0,
            y: VIRTUAL_HEIGHT / 2.0 - board_total_size / 2.0 + 30.0,
            winning_cells: None,
        }
    }

    pub fn draw(&mut self) {
        let dt = get_frame_time();
        let time = get_time() as f32;

        // Draw grid lines
        let color = DARK_GREY;
        let thickness = 4.0;

        for i in 1..3 {
            let offset = i as f32 * self.cell_size;
            draw_line(
                self.x + offset,
                self.y,
                self.x + offset,
                self.y + self.cell_size * 3.0,
                thickness,
                color,
            );
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
                let cell = &mut self.cells[row][col];
                if cell.state != CellState::Empty {
                    // Update animation
                    if cell.anim_timer < 1.0 {
                        cell.anim_timer = (cell.anim_timer + dt * 5.0).min(1.0);
                    }

                    let cell_x = self.x + col as f32 * self.cell_size;
                    let cell_y = self.y + row as f32 * self.cell_size;

                    let font = crate::config::get_inter_font();

                    // Base entry animation
                    let t = cell.anim_timer;
                    let mut scale = if t < 1.0 {
                        let overshoot = 0.4;
                        let s =
                            1.0 + overshoot * (1.0 - t) * (t * std::f32::consts::PI * 2.0).cos();
                        s * t
                    } else {
                        1.0
                    };

                    // Victory pulse
                    if let Some(wins) = self.winning_cells {
                        if wins.contains(&(row, col)) {
                            scale *= 1.0 + (time * 8.0).sin() * 0.1;
                        }
                    }

                    let font_size = (120.0 * scale) as u16;
                    let (text, color) = match cell.state {
                        CellState::X => ("X", PRIMARY_BLUE),
                        _ => ("O", SUCCESS_GREEN),
                    };

                    let text_dim = measure_text(text, font, font_size, 1.0);
                    draw_text_ex(
                        text,
                        cell_x + self.cell_size / 2.0 - text_dim.width / 2.0,
                        cell_y + self.cell_size / 2.0 + text_dim.height / 2.0 - 5.0,
                        TextParams {
                            font,
                            font_size,
                            color,
                            ..Default::default()
                        },
                    );
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

    pub fn set_cell(&mut self, row: usize, col: usize, state: CellState) {
        if self.cells[row][col].state == CellState::Empty {
            self.cells[row][col].state = state;
            self.cells[row][col].anim_timer = 0.0;
        }
    }

    pub fn get_cell_center(&self, row: usize, col: usize) -> Vec2 {
        vec2(
            self.x + col as f32 * self.cell_size + self.cell_size / 2.0,
            self.y + row as f32 * self.cell_size + self.cell_size / 2.0,
        )
    }

    pub fn check_winner(&mut self) -> Option<CellState> {
        if let Some((state, cells)) = self.check_winner_pure() {
            self.winning_cells = Some(cells);
            return Some(state);
        }
        None
    }

    pub fn check_winner_pure(&self) -> Option<(CellState, [(usize, usize); 3])> {
        // Rows
        for row in 0..3 {
            if self.cells[row][0].state != CellState::Empty
                && self.cells[row][0].state == self.cells[row][1].state
                && self.cells[row][1].state == self.cells[row][2].state
            {
                return Some((self.cells[row][0].state, [(row, 0), (row, 1), (row, 2)]));
            }
        }

        // Columns
        for col in 0..3 {
            if self.cells[0][col].state != CellState::Empty
                && self.cells[0][col].state == self.cells[1][col].state
                && self.cells[1][col].state == self.cells[2][col].state
            {
                return Some((self.cells[0][col].state, [(0, col), (1, col), (2, col)]));
            }
        }

        // Diagonals
        if self.cells[0][0].state != CellState::Empty
            && self.cells[0][0].state == self.cells[1][1].state
            && self.cells[1][1].state == self.cells[2][2].state
        {
            return Some((self.cells[0][0].state, [(0, 0), (1, 1), (2, 2)]));
        }
        if self.cells[0][2].state != CellState::Empty
            && self.cells[0][2].state == self.cells[1][1].state
            && self.cells[1][1].state == self.cells[2][0].state
        {
            return Some((self.cells[0][2].state, [(0, 2), (1, 1), (2, 0)]));
        }

        None
    }

    pub fn is_full(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.cells[row][col].state == CellState::Empty {
                    return false;
                }
            }
        }
        true
    }

    pub fn reset(&mut self) {
        self.cells = [[Cell {
            state: CellState::Empty,
            anim_timer: 0.0,
        }; 3]; 3];
        self.winning_cells = None;
    }
}
