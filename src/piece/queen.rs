use crate::{
    board::{coordinate_to_index, Board},
    piece::Piece,
    PieceColour, PieceType,
};

#[derive(Clone, Copy, Debug)]
pub struct Queen {
    colour: PieceColour,
    file: u8,
    rank: u8,
}

impl Piece for Queen {
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

        if file_diff != 0 && rank_diff != file_diff && rank_diff != 0
            || rank_diff != 0 && rank_diff != file_diff && file_diff != 0
        {
            return false;
        }

        let file_dir = if file_diff == 0 {
            0
        } else {
            match new_file > self.file {
                true => 1,
                false => -1,
            }
        };
        let rank_dir = if rank_diff == 0 {
            0
        } else {
            match new_rank > self.rank {
                true => 1,
                false => -1,
            }
        };

        let mut checking_file = new_file as i32 - file_dir;
        let mut checking_rank = new_rank as i32 - rank_dir;

        while checking_file != self.file as i32 {
            if let Some(_) =
                board.squares[coordinate_to_index(checking_file as u8, checking_rank as u8)]
            {
                return false;
            }

            checking_file -= file_dir;
            checking_rank -= rank_dir;
        }

        true
    }

    fn update_pos(&mut self, new_file: u8, new_rank: u8) {
        self.file = new_file;
        self.rank = new_rank;
    }

    fn colour(&self) -> PieceColour {
        self.colour
    }

    fn piece_type(&self) -> PieceType {
        PieceType::QUEEN
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EMPTY_FEN;
    #[test]
    fn test1() {
        let k = Queen::new(true, 0, 0);
        let b = &Board::from_fen(EMPTY_FEN).unwrap();
        assert!(k.is_pseudo_legal(2, 0, b))
    }
    #[test]
    fn test2() {
        let k = Queen::new(false, 7, 7);
        let b = &Board::from_fen(EMPTY_FEN).unwrap();
        assert!(!k.is_pseudo_legal(5, 4, b))
    }
    #[test]
    fn test3() {
        let k = Queen::new(true, 2, 4);
        let b = &Board::from_fen(EMPTY_FEN).unwrap();
        assert!(k.is_pseudo_legal(3, 3, b))
    }
}
