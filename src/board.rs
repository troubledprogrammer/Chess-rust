use crate::io::Command;
use crate::piece::{Bishop, King, Knight, Pawn, Piece, Queen, Rook};
use crate::{PieceColour, UnitResult};

use arr_macro::arr;

pub type Square = Option<Box<dyn Piece>>;

/// converts a (file, rank) pair to an index
pub fn coordinate_to_index(file: u8, rank: u8) -> usize {
    (file + rank * 8) as usize
}

/// converts an index to a (file, rank) pair
pub fn index_to_coordinate(index: usize) -> (u8, u8) {
    ((index % 8) as u8, (index / 8) as u8)
}

pub struct Board {
    pub squares: [Square; 64],
    pub turn: PieceColour,
}

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut f: Vec<_> = fen
            .chars()
            .rev()
            .collect::<String>()
            .split(" ")
            .map(|s| s.to_owned())
            .collect();

        // position
        let mut squares: [Square; 64] = arr![None; 64];
        let mut ind: usize = 0;
        let mut pos = f.pop().ok_or("Invalid fen: No position")?;

        loop {
            match pos.pop() {
                Some(chr) => {
                    if chr.is_numeric() {
                        ind += chr.to_digit(10).unwrap() as usize;
                    } else if chr.is_alphabetic() {
                        let is_white = !chr.is_lowercase();
                        let (file, rank) = index_to_coordinate(ind);
                        squares[ind] = match chr.to_lowercase().to_string().as_str() {
                            "p" => Some(Box::new(Pawn::new(is_white, file, rank))),
                            "n" => Some(Box::new(Knight::new(is_white, file, rank))),
                            "b" => Some(Box::new(Bishop::new(is_white, file, rank))),
                            "r" => Some(Box::new(Rook::new(is_white, file, rank))),
                            "q" => Some(Box::new(Queen::new(is_white, file, rank))),
                            "k" => Some(Box::new(King::new(is_white, file, rank))),
                            _ => None,
                            // _ => {
                            //     return Err(format!(
                            //         "Invalid fen: cannot parse char '{}' as a piece",
                            //         chr
                            //     ))
                            // }
                        };
                        ind += 1;
                    }
                }
                None => break,
            };
        }
        // end position

        // turn
        let t = f.pop().ok_or("Invalid fen: No turn")?;
        let turn = match t.as_str() {
            "w" => PieceColour::WHITE,
            "b" => PieceColour::BLACK,
            _ => return Err(format!("Invalid fen: '{}' is not a valid turn", t)),
        };
        // end turn

        // TODO: castling

        // TODO: en passant

        // TODO: halfmove clock (50 move rule)

        // TODO: fullmove clock

        Ok(Self { squares, turn })
    }

    pub fn update(&mut self, command: Command) -> UnitResult {
        match command {
            Command::Move {
                start_file,
                start_rank,
                end_file,
                end_rank,
            } => self.make_move(start_file, start_rank, end_file, end_rank),
            _ => (),
        }
        Ok(())
    }

    fn make_move(&mut self, start_file: u8, start_rank: u8, end_file: u8, end_rank: u8) {
        if let Some(mut piece) = self.squares[coordinate_to_index(start_file, start_rank)].take() {
            if piece.is_pseudo_legal(end_file, end_rank, self) {
                piece.update_pos(end_file, end_rank);
                self.turn = !self.turn;
                self.squares[coordinate_to_index(end_file, end_rank)] = Some(piece);
                println!("{:?}'s turn", self.turn);
            } else {
                println!("Cannot make move: invalid move");
                self.squares[coordinate_to_index(start_file, start_rank)] = Some(piece);
            }
        } else {
            println!("Cannot make move: piece doesn't exist");
        }
    }
}
