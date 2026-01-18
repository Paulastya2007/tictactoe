use crate::board::{Board, CellState};

pub fn find_best_move(board: &Board, ai_symbol: CellState) -> Option<(usize, usize)> {
    let mut best_score = f32::NEG_INFINITY;
    let mut best_move = None;

    for row in 0..3 {
        for col in 0..3 {
            if board.cells[row][col] == CellState::Empty {
                let mut temp_board = *board;
                temp_board.cells[row][col] = ai_symbol;
                let score = minimax(&temp_board, 0, false, ai_symbol);
                if score > best_score {
                    best_score = score;
                    best_move = Some((row, col));
                }
            }
        }
    }

    best_move
}

fn minimax(board: &Board, depth: i32, is_maximizing: bool, ai_symbol: CellState) -> f32 {
    let player_symbol = if ai_symbol == CellState::X {
        CellState::O
    } else {
        CellState::X
    };

    if let Some(winner) = board.check_winner() {
        if winner == ai_symbol {
            return 10.0 - depth as f32;
        } else if winner == player_symbol {
            return -10.0 + depth as f32;
        }
    }

    if board.is_full() {
        return 0.0;
    }

    if is_maximizing {
        let mut best_score = f32::NEG_INFINITY;
        for row in 0..3 {
            for col in 0..3 {
                if board.cells[row][col] == CellState::Empty {
                    let mut temp_board = *board;
                    temp_board.cells[row][col] = ai_symbol;
                    let score = minimax(&temp_board, depth + 1, false, ai_symbol);
                    if score > best_score {
                        best_score = score;
                    }
                }
            }
        }
        best_score
    } else {
        let mut best_score = f32::INFINITY;
        for row in 0..3 {
            for col in 0..3 {
                if board.cells[row][col] == CellState::Empty {
                    let mut temp_board = *board;
                    temp_board.cells[row][col] = player_symbol;
                    let score = minimax(&temp_board, depth + 1, true, ai_symbol);
                    if score < best_score {
                        best_score = score;
                    }
                }
            }
        }
        best_score
    }
}
