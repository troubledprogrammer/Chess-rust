use crate::board::Board;
// use crate::io::console::ConsoleRenderer;
use crate::io::window::WindowRenderer;
use crate::io::{Command, IO};
use crate::UnitResult;

pub struct Game {
    pub board: Board,
    pub io: Box<dyn IO>,
    is_running: bool,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            board: Board::new()?,
            io: Box::new(WindowRenderer::new()?),
            is_running: true,
        })
    }

    pub fn run(&mut self) -> UnitResult {
        while self.is_running {
            self.render()?;
            self.update()?;
        }

        Ok(())
    }

    fn render(&mut self) -> UnitResult {
        self.io.render(&self.board)
    }

    fn update(&mut self) -> UnitResult {
        match self.io.get_command()? {
            Command::None => Ok(()),
            Command::Quit => {
                self.is_running = false;
                Ok(())
            }
            cmd => self.board.update(cmd),
        }
    }
}
