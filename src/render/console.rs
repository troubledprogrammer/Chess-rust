use crate::{board::Board, board::Square, PieceColour, PieceType, UnitResult};

/// Renders the came through a text cli
pub struct ConsoleRenderer {}

impl ConsoleRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, board: &Board) -> UnitResult {
        println!("    a   b   c   d   e   f   g   h  ");
        println!("  +---+---+---+---+---+---+---+---+");
        for rank in 0..8 {
            print!("{} ", 8 - rank);
            for file in 0..8 {
                print!("| {} ", self.piece_to_char(&board.squares[rank * 8 + file]));
            }
            println!("|");
            println!("  +---+---+---+---+---+---+---+---+");
        }

        println!("{:?}'s turn", &board.turn);

        Ok(())
    }

    fn piece_to_char(&self, piece_option: &Square) -> char {
        match piece_option {
            Some(piece) => match (piece.colour(), piece.piece_type()) {
                (PieceColour::WHITE, PieceType::KING) => 'K',
                (PieceColour::WHITE, PieceType::QUEEN) => 'Q',
                (PieceColour::WHITE, PieceType::ROOK) => 'R',
                (PieceColour::WHITE, PieceType::BISHOP) => 'B',
                (PieceColour::WHITE, PieceType::KNIGHT) => 'N',
                (PieceColour::WHITE, PieceType::PAWN) => 'P',
                (PieceColour::BLACK, PieceType::KING) => 'k',
                (PieceColour::BLACK, PieceType::QUEEN) => 'q',
                (PieceColour::BLACK, PieceType::ROOK) => 'r',
                (PieceColour::BLACK, PieceType::BISHOP) => 'b',
                (PieceColour::BLACK, PieceType::KNIGHT) => 'n',
                (PieceColour::BLACK, PieceType::PAWN) => 'p',
            },
            None => ' ',
        }
    }
}
