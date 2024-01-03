pub mod pawn;

use crate::{board::Board, PieceColour, PieceType};

pub trait Piece {
    /// constructor
    fn new(is_white: bool, file: u8, rank: u8) -> Self
    where
        Self: Sized;

    /// checks if a given square can be moved to
    fn reachable(&self, new_file: u8, new_rank: u8, board: Board) -> bool;

    /// gets colour of piece
    fn colour(&self) -> PieceColour;

    /// gets type of piece
    fn piece_type(&self) -> PieceType;
}
