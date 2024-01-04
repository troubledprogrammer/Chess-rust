use crate::{
    board::{coordinate_to_index, Board},
    piece::Piece,
    PieceColour, PieceType,
};

#[derive(Clone, Copy)]
pub struct Pawn {
    colour: PieceColour,
    file: u8,
    rank: u8,
}

impl Piece for Pawn {
    fn new(is_white: bool, file: u8, rank: u8) -> Self
    where
        Self: Sized,
    {
        Self {
            colour: if is_white {
                PieceColour::WHITE
            } else {
                PieceColour::BLACK
            },
            file,
            rank,
        }
    }

    fn is_pseudo_legal(&self, new_file: u8, new_rank: u8, board: Board) -> bool {
        let direction: i32 = match self.colour {
            PieceColour::WHITE => 1,
            PieceColour::BLACK => -1,
        };

        // bounds check
        // files stored with 0 indexing
        if new_file >= 8 {
            return false;
        }

        // move forward 1
        if new_file == self.file && new_rank.abs_diff(self.rank) == 1 {
            let next_rank = (self.rank as i32 + direction) as u8; // can't be out of bounds as pawn can't be on rank 1 or 8
            if let Some(_) = board.squares[coordinate_to_index(new_file, next_rank as u8)] {
                return false;
            } else {
                return true;
            }
        };

        // capture
        if new_file.abs_diff(self.file) == 1 && new_rank.abs_diff(self.rank) == 1 {
            if let Some(piece) = &board.squares[coordinate_to_index(new_file, new_rank)] {
                return piece.colour() == self.colour;
            }
        };

        false
    }

    fn update_pos(&mut self, new_file: u8, new_rank: u8) {
        self.file = new_file;
        self.rank = new_rank;
    }

    fn colour(&self) -> PieceColour {
        self.colour
    }

    fn piece_type(&self) -> crate::PieceType {
        PieceType::PAWN
    }
}
