use crate::board::{Board, CellState};
use crate::config::*;
use crate::state::GameState;
use crate::utils::*;
use macroquad::prelude::*;

static mut BOARD: Option<Board> = None;
static mut CURRENT_TURN: CellState = CellState::X;
static mut GAME_OVER: bool = false;
static mut WINNER: Option<CellState> = None;
static mut PLAYER_SYMBOL: CellState = CellState::X;
static mut AI_TIMER: f32 = 0.0;

pub fn set_player_symbol(symbol: CellState) {
    unsafe {
        PLAYER_SYMBOL = symbol;
    }
}

pub fn is_game_over() -> bool {
    unsafe { GAME_OVER }
}

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
    let theme = crate::theme::get_current_theme();

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
            color: theme.text,
            ..Default::default()
        },
    );

    draw_text_ex(
        "ESC: Menu | R: Reset | T: Theme",
        20.0,
        70.0,
        TextParams {
            font,
            font_size: 18,
            color: theme.muted,
            ..Default::default()
        },
    );

    unsafe {
        if GAME_OVER {
            let msg = match WINNER {
                Some(CellState::X) => "PLAYER X WINS!",
                Some(CellState::O) => "PLAYER O WINS!",
                _ => "IT'S A DRAW!",
            };
            let color = match WINNER {
                Some(CellState::X) => theme.primary,
                Some(CellState::O) => theme.secondary,
                _ => theme.muted,
            };

            let dim = measure_text(msg, font, 40, 1.0);
            draw_text_ex(
                msg,
                VIRTUAL_WIDTH / 2.0 - dim.width / 2.0,
                110.0,
                TextParams {
                    font,
                    font_size: 40,
                    color,
                    ..Default::default()
                },
            );

            // RESTART MESSAGE
            let restart_msg = "PRESS R TO RESTART";
            let r_dim = measure_text(restart_msg, font, 24, 1.0);
            draw_text_ex(
                restart_msg,
                VIRTUAL_WIDTH / 2.0 - r_dim.width / 2.0,
                150.0,
                TextParams {
                    font,
                    font_size: 24,
                    color: theme.text,
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
                    color: theme.text,
                    ..Default::default()
                },
            );
        }
    }

    // ---- Draw Board ----
    board.draw();

    // ---- Gameplay Logic ----
    unsafe {
        if !GAME_OVER {
            let is_ai_mode = mode == GameState::PvAI;
            let is_ai_turn = is_ai_mode && CURRENT_TURN != PLAYER_SYMBOL;

            if is_ai_turn {
                AI_TIMER += get_frame_time();
                if AI_TIMER >= 0.6 {
                    // Artificial delay for feel
                    if let Some((row, col)) = crate::ai::find_best_move(board, CURRENT_TURN) {
                        apply_move(board, row, col);
                    }
                    AI_TIMER = 0.0;
                }
            } else {
                if is_mouse_button_pressed(MouseButton::Left) {
                    if let Some((row, col)) = board.get_cell_at(mouse) {
                        if board.cells[row][col].state == CellState::Empty {
                            apply_move(board, row, col);
                        }
                    }
                }
            }
        }
    }

    // Theme switching
    if is_key_pressed(KeyCode::T) {
        crate::theme::cycle_theme();
    }

    // Reset game
    if is_key_pressed(KeyCode::R) {
        board.reset();
        unsafe {
            GAME_OVER = false;
            WINNER = None;
            CURRENT_TURN = CellState::X;
            AI_TIMER = 0.0;
        }
    }

    if is_key_pressed(KeyCode::Escape) {
        board.reset();
        unsafe {
            GAME_OVER = false;
            WINNER = None;
            CURRENT_TURN = CellState::X;
            AI_TIMER = 0.0;
        }
        return Some(GameState::Menu);
    }

    None
}

unsafe fn apply_move(board: &mut Board, row: usize, col: usize) {
    unsafe {
        board.set_cell(row, col, CURRENT_TURN);
    }
    crate::config::play_move();

    // Spawn move particles
    let center = board.get_cell_center(row, col);
    let color = unsafe {
        if CURRENT_TURN == CellState::X {
            crate::theme::get_current_theme().primary
        } else {
            crate::theme::get_current_theme().secondary
        }
    };
    crate::particles::spawn_move(center, color);

    // Check winner/draw
    if let Some(winner) = board.check_winner() {
        unsafe {
            WINNER = Some(winner);
            GAME_OVER = true;
        }
        crate::config::play_win();

        // Spawn win particles for all winning cells
        if let Some(winning_coords) = board.winning_cells {
            for coords in winning_coords {
                let win_center = board.get_cell_center(coords.0, coords.1);
                crate::particles::spawn_win(win_center, color);
            }
        }
    } else if board.is_full() {
        unsafe {
            GAME_OVER = true;
        }
    } else {
        // Switch turn
        unsafe {
            CURRENT_TURN = if CURRENT_TURN == CellState::X {
                CellState::O
            } else {
                CellState::X
            };
        }
    }
}
