use crate::{board::Board, piece::Piece, PieceColour, PieceType};

#[derive(Clone, Copy)]
pub struct Pawn {
    colour: PieceColour,
    _file: u8,
    _rank: u8,
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
            _file: file,
            _rank: rank,
        }
    }

    fn reachable(&self, _new_file: u8, _new_rank: u8, _board: Board) -> bool {
        false
    }

    fn colour(&self) -> PieceColour {
        self.colour
    }

    fn piece_type(&self) -> crate::PieceType {
        PieceType::PAWN
    }
}
