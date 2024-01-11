use std::collections::HashMap;

use raylib::prelude::*;

use crate::board::{coordinate_to_index, index_to_coordinate, Board};
use crate::io::{Command, IO};
use crate::{PieceColour, PieceType, UnitResult};

pub struct WindowRenderer {
    rl: RaylibHandle,
    thread: RaylibThread,
    board_texture: Texture2D,
    piece_textures: HashMap<(PieceColour, PieceType), Texture2D>,
    selected_square: Option<(u8, u8)>,
}

impl WindowRenderer {
    pub fn new() -> Result<Self, String> {
        set_trace_log(TraceLogLevel::LOG_ERROR);

        let (mut rl, thread) = init().size(800, 800).title("Chess").build();

        // board
        let mut board_img = Image::load_image("assets/board/wood/board.png")
            .map_err(|_| "Couldn't load board texture")?;
        board_img.resize(800, 800);
        let board_texture = rl
            .load_texture_from_image(&thread, &board_img)
            .map_err(|_| "Couldn't create image from board texture")?;

        // pieces
        let mut piece_textures: HashMap<(PieceColour, PieceType), Texture2D> = HashMap::new();
        for piece_type in PieceType::PIECES {
            for colour in PieceColour::COLOURS {
                let path = match (colour, piece_type) {
                    (PieceColour::WHITE, PieceType::KING) => "wk",
                    (PieceColour::WHITE, PieceType::QUEEN) => "wq",
                    (PieceColour::WHITE, PieceType::ROOK) => "wr",
                    (PieceColour::WHITE, PieceType::BISHOP) => "wb",
                    (PieceColour::WHITE, PieceType::KNIGHT) => "wn",
                    (PieceColour::WHITE, PieceType::PAWN) => "wp",
                    (PieceColour::BLACK, PieceType::KING) => "bk",
                    (PieceColour::BLACK, PieceType::QUEEN) => "bq",
                    (PieceColour::BLACK, PieceType::ROOK) => "br",
                    (PieceColour::BLACK, PieceType::BISHOP) => "bb",
                    (PieceColour::BLACK, PieceType::KNIGHT) => "bn",
                    (PieceColour::BLACK, PieceType::PAWN) => "bp",
                };

                let mut img =
                    Image::load_image(format!("assets/pieces/default/{path}.png").as_str())
                        .map_err(|_| format!("Couldn't load texture {path}.png"))?;
                img.resize(100, 100);
                let tex = rl
                    .load_texture_from_image(&thread, &img)
                    .map_err(|_| "Couldn't create texture from img")?;
                piece_textures.insert((colour, piece_type), tex);
            }
        }

        // selected squares

        Ok(Self {
            rl,
            thread,
            board_texture,
            piece_textures,
            selected_square: None,
        })
    }
}

impl IO for WindowRenderer {
    fn get_command(&mut self, board: &Board) -> Result<Command, String> {
        if self.rl.window_should_close() {
            return Ok(Command::Quit);
        }

        if self
            .rl
            .is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
        {
            let mouse_pos = self.rl.get_mouse_position();
            let file = (mouse_pos.x / 100.0) as u8;
            let rank = (mouse_pos.y / 100.0) as u8;
            if let Some(piece) = &board.squares[coordinate_to_index(file, rank)] {
                if piece.colour() == board.turn {
                    self.selected_square = Some((file, rank));
                    return Ok(Command::None);
                }
            }
            if let Some((selected_file, selected_rank)) = self.selected_square {
                self.selected_square = None;
                return Ok(Command::Move {
                    start_file: selected_file,
                    start_rank: selected_rank,
                    end_file: file,
                    end_rank: rank,
                    promotion_piece: Some(PieceType::QUEEN), // TODO
                });
            } else {
                self.selected_square = None;
            }
        }

        Ok(Command::None)
    }

    fn render(&mut self, board: &Board) -> UnitResult {
        let mut d = self.rl.begin_drawing(&self.thread);

        // draw board
        d.draw_texture(&self.board_texture, 0, 0, Color::WHITE);

        // draw selected
        if let Some((file, rank)) = self.selected_square {
            d.draw_rectangle(
                100 * file as i32,
                100 * rank as i32,
                100,
                100,
                Color::new(245, 140, 40, 180), // translusent orange
            )
        }

        // draw pieces
        for square in board.squares.iter().enumerate() {
            if let (ind, Some(piece)) = square {
                let (file, rank) = index_to_coordinate(ind);
                let tex = self
                    .piece_textures
                    .get(&(piece.colour(), piece.piece_type()));
                if let Some(t) = tex {
                    d.draw_texture(&t, 100 * file as i32, 100 * rank as i32, Color::WHITE)
                } else {
                    return Err(format!("Texture for {:?} does not exist", piece));
                }
            }
        }

        Ok(())
    }
}
