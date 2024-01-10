use crate::{
    board::{coordinate_to_index, Board},
    piece::Piece,
    PieceColour, PieceType,
};

#[derive(Clone, Copy, Debug)]
pub struct King {
    colour: PieceColour,
    file: u8,
    rank: u8,
}

impl Piece for King {
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

    fn is_pseudo_legal(&self, new_file: u8, new_rank: u8, board: &Board) -> bool {
        if let Some(piece) = &board.squares[coordinate_to_index(new_file, new_rank)] {
            if piece.colour() == self.colour {
                return false;
            }
        }

        let file_diff = new_file.abs_diff(self.file);
        let rank_diff = new_rank.abs_diff(self.rank);

        if file_diff <= 1 && rank_diff <= 1 && file_diff + rank_diff != 0 {
            return true;
        }

        false
    }

    fn update_pos(&mut self, new_file: u8, new_rank: u8) {
        self.file = new_file;
        self.rank = new_rank;
    }

    fn colour(&self) -> PieceColour {
        self.colour
    }

    fn piece_type(&self) -> PieceType {
        PieceType::KING
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EMPTY_FEN;
    #[test]
    fn test1() {
        let k = King::new(true, 0, 0);
        let b = &Board::from_fen(EMPTY_FEN).unwrap();
        assert!(k.is_pseudo_legal(1, 1, b))
    }
    #[test]
    fn test2() {
        let k = King::new(false, 7, 7);
        let b = &Board::from_fen(EMPTY_FEN).unwrap();
        assert!(!k.is_pseudo_legal(7, 7, b))
    }
    #[test]
    fn test3() {
        let k = King::new(true, 2, 4);
        let b = &Board::from_fen(EMPTY_FEN).unwrap();
        assert!(k.is_pseudo_legal(3, 3, b))
    }
}
