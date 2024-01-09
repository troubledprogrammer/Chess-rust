use crate::{
    board::{coordinate_to_index, Board},
    piece::Piece,
    PieceColour, PieceType,
};

#[derive(Clone, Copy, Debug)]
pub struct Knight {
    colour: PieceColour,
    file: u8,
    rank: u8,
}

impl Piece for Knight {
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

        let diff = (new_file.abs_diff(self.file), new_rank.abs_diff(self.rank));

        if diff == (2, 1) || diff == (1, 2) {
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

    fn piece_type(&self) -> crate::PieceType {
        PieceType::KNIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EMPTY_FEN;
    #[test]
    fn test1() {
        let k = Knight::new(true, 0, 0);
        let b = &Board::from_fen(EMPTY_FEN).unwrap();
        assert!(k.is_pseudo_legal(2, 1, b))
    }
    #[test]
    fn test2() {
        let k = Knight::new(false, 7, 7);
        let b = &Board::from_fen(EMPTY_FEN).unwrap();
        assert!(!k.is_pseudo_legal(5, 5, b))
    }
    #[test]
    fn test3() {
        let k = Knight::new(true, 2, 4);
        let b = &Board::from_fen(EMPTY_FEN).unwrap();
        assert!(k.is_pseudo_legal(3, 2, b))
    }
}
