#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use std::{ops::Not, result};

use game::Game;
use io::{console::ConsoleRenderer, window::WindowRenderer};

mod board;
mod game;
mod io;
mod piece;

type UnitResult = result::Result<(), String>;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const EMPTY_FEN: &str = "8/8/8/8/8/8/8/8 w - - 0 1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
}

impl PieceType {
    const PIECES: [Self; 6] = [
        Self::KING,
        Self::QUEEN,
        Self::ROOK,
        Self::BISHOP,
        Self::KNIGHT,
        Self::PAWN,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceColour {
    WHITE,
    BLACK,
}

impl PieceColour {
    const COLOURS: [Self; 2] = [Self::WHITE, Self::BLACK];
}

impl Not for PieceColour {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            PieceColour::WHITE => PieceColour::BLACK,
            PieceColour::BLACK => PieceColour::WHITE,
        }
    }
}

fn main() -> UnitResult {
    let renderer = WindowRenderer::new()?;
    let mut game = Game::new(renderer)?;

    game.run()
}
