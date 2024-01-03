use crate::board::Board;
use crate::render::console::ConsoleRenderer;
use crate::UnitResult;

use std::io::stdin;

pub struct Game {
    pub board: Board,
    pub renderer: ConsoleRenderer,
    pub is_running: bool,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            board: Board::new()?,
            renderer: ConsoleRenderer::new(),
            is_running: true,
        })
    }

    pub fn run(&mut self) -> UnitResult {
        self.render()?;

        while self.is_running {
            self.update()?;
            self.render()?;
        }

        Ok(())
    }

    fn render(&self) -> UnitResult {
        self.renderer.render(&self.board)
    }

    fn update(&mut self) -> UnitResult {
        let mut buffer = String::new();

        stdin()
            .read_line(&mut buffer)
            .map_err(|err| err.to_string())?;

        self.board.update(buffer)
    }
}
