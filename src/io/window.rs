use std::collections::HashMap;

use raylib::prelude::*;

use crate::board::{index_to_coordinate, Board};
use crate::io::{Command, IO};
use crate::{PieceColour, PieceType, UnitResult};

pub struct WindowRenderer {
    rl: RaylibHandle,
    thread: RaylibThread,
    board_texture: Texture2D,
    piece_textures: HashMap<(PieceColour, PieceType), Texture2D>,
}

impl WindowRenderer {
    pub fn new() -> Result<Self, String> {
        set_trace_log(TraceLogLevel::LOG_ERROR);

        let (mut rl, thread) = raylib::init().size(800, 800).title("Chess").build();

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

        Ok(Self {
            rl,
            thread,
            board_texture,
            piece_textures,
        })
    }
}

impl IO for WindowRenderer {
    fn get_command(&mut self) -> Result<Command, String> {
        if self.rl.window_should_close() {
            return Ok(Command::Quit);
        }
        Ok(Command::None)
    }

    fn render(&mut self, board: &Board) -> UnitResult {
        let mut d = self.rl.begin_drawing(&self.thread);

        // draw board
        d.draw_texture(&self.board_texture, 0, 0, Color::WHITE);

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
