use crate::board::{Board, CellState};
use crate::config::*;
use crate::state::GameState;
use crate::utils::*;
use macroquad::prelude::*;

static mut BOARD: Option<Board> = None;
static mut CURRENT_TURN: CellState = CellState::X;
static mut GAME_OVER: bool = false;
static mut WINNER: Option<CellState> = None;

#[allow(static_mut_refs)]
pub fn update(mode: GameState, scale: &ScreenScale) -> Option<GameState> {
    let board = unsafe {
        if BOARD.is_none() {
            BOARD = Some(Board::new());
        }
        BOARD.as_mut().unwrap()
    };

    let mouse = mouse_to_virtual(scale);
    let font = crate::config::get_font();

    // ---- Draw UI ----
    draw_text_ex(
        match mode {
            GameState::PvP => "Player vs Player",
            GameState::PvAI => "Player vs AI",
            _ => "",
        },
        20.0,
        40.0,
        TextParams {
            font,
            font_size: 24,
            color: TEXT_COLOR,
            ..Default::default()
        },
    );

    draw_text_ex(
        "Press ESC to return to Menu | R to Reset",
        20.0,
        70.0,
        TextParams {
            font,
            font_size: 18,
            color: MUTED_GREY,
            ..Default::default()
        },
    );

    // Turn indicator or Game Over message
    unsafe {
        if GAME_OVER {
            let msg = match WINNER {
                Some(CellState::X) => "PLAYER X WINS!",
                Some(CellState::O) => "PLAYER O WINS!",
                _ => "IT'S A DRAW!",
            };
            let color = match WINNER {
                Some(CellState::X) => PRIMARY_BLUE,
                Some(CellState::O) => SUCCESS_GREEN,
                _ => MUTED_GREY,
            };

            let dim = measure_text(msg, font, 40, 1.0);
            draw_text_ex(
                msg,
                VIRTUAL_WIDTH / 2.0 - dim.width / 2.0,
                80.0,
                TextParams {
                    font,
                    font_size: 40,
                    color,
                    ..Default::default()
                },
            );
        } else {
            let turn_text = match CURRENT_TURN {
                CellState::X => "Turn: X",
                CellState::O => "Turn: O",
                _ => "",
            };
            draw_text_ex(
                turn_text,
                VIRTUAL_WIDTH - 120.0,
                40.0,
                TextParams {
                    font,
                    font_size: 24,
                    color: DARK_GREY,
                    ..Default::default()
                },
            );
        }
    }

    // ---- Draw Board ----
    board.draw();

    // ---- Input Handling ----
    unsafe {
        if !GAME_OVER {
            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some((row, col)) = board.get_cell_at(mouse) {
                    if board.cells[row][col] == CellState::Empty {
                        board.cells[row][col] = CURRENT_TURN;

                        // Check winner/draw
                        if let Some(winner) = board.check_winner() {
                            WINNER = Some(winner);
                            GAME_OVER = true;
                        } else if board.is_full() {
                            GAME_OVER = true;
                        } else {
                            // Switch turn
                            CURRENT_TURN = if CURRENT_TURN == CellState::X {
                                CellState::O
                            } else {
                                CellState::X
                            };
                        }
                    }
                }
            }
        }
    }

    // Reset game
    if is_key_pressed(KeyCode::R) {
        board.reset();
        unsafe {
            GAME_OVER = false;
            WINNER = None;
            CURRENT_TURN = CellState::X;
        }
    }

    if is_key_pressed(KeyCode::Escape) {
        // Reset game on exit too
        board.reset();
        unsafe {
            GAME_OVER = false;
            WINNER = None;
            CURRENT_TURN = CellState::X;
        }
        return Some(GameState::Menu);
    }

    None
}
