// #![allow(dead_code, unused_variables, unused_mut)]

use std::result;

use game::Game;

mod board;
mod game;
mod piece;
mod render;

type UnitResult = result::Result<(), String>;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceColour {
    WHITE,
    BLACK,
}

fn main() -> UnitResult {
    let mut game = Game::new()?;

    game.run()
}
