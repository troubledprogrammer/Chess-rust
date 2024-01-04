use crate::board::Board;
use crate::io::console::ConsoleRenderer;
use crate::io::Command;
use crate::UnitResult;

pub struct Game {
    pub board: Board,
    pub io: ConsoleRenderer,
    is_running: bool,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            board: Board::new()?,
            io: ConsoleRenderer::new(),
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

    fn render(&self) -> UnitResult {
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
