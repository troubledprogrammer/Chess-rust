pub mod pawn;

use std::fmt::Debug;

use crate::{board::Board, PieceColour, PieceType};

pub trait Piece {
    /// constructor
    fn new(is_white: bool, file: u8, rank: u8) -> Self
    where
        Self: Sized;

    /// checks if a given square can be moved to
    fn is_pseudo_legal(&self, new_file: u8, new_rank: u8, board: Board) -> bool;

    /// updates the piece's position
    fn update_pos(&mut self, new_file: u8, new_rank: u8);

    /// gets colour of piece
    fn colour(&self) -> PieceColour;

    /// gets type of piece
    fn piece_type(&self) -> PieceType;
}

impl Debug for dyn Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Piece{{{:?}, {:?}}}", self.colour(), self.piece_type())
    }
}
