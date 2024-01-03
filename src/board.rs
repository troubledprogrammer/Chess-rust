use crate::piece::pawn::Pawn;
use crate::piece::Piece;
use crate::{PieceColour, UnitResult, STARTING_FEN};

use arr_macro::arr;

pub type Square = Option<Box<dyn Piece>>;

pub struct Board {
    pub squares: [Square; 64],
    pub turn: PieceColour,
}

impl Board {
    pub fn new() -> Result<Self, String> {
        Board::from_fen(Some("8/pppppppp/pp4pp/PpPp3P/8/8/PPPPPPPP/8 b KQkq - 0 1"))
    }

    fn from_fen(fen: Option<&str>) -> Result<Self, String> {
        let mut f: Vec<String> = match fen {
            Some(s) => s,
            None => STARTING_FEN,
        }
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
                        let file = (ind % 8) as u8;
                        let rank = (ind / 8) as u8;
                        squares[ind] = match chr.to_lowercase().to_string().as_str() {
                            "p" => Some(Box::new(Pawn::new(is_white, file, rank))),
                            _ => {
                                return Err(format!(
                                    "Invalid fen: cannot parse char '{}' as a piece",
                                    chr
                                ))
                            }
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

    pub fn update(&mut self, command: String) -> UnitResult {
        println!(
            "Updated with instruction: {}",
            &command[..command.len() - 1]
        );

        Ok(())
    }
}
