use crate::board::Board;
use crate::io::{Command, IO};
use crate::{UnitResult, STARTING_FEN};

pub struct Game<T: IO> {
    pub board: Board,
    pub io: T,
    is_running: bool,
}

impl<T: IO> Game<T> {
    pub fn new(io: T) -> Result<Self, String> {
        Ok(Self {
            board: Board::from_fen(STARTING_FEN)?,
            io: io,
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
        match self.io.get_command(&self.board)? {
            Command::None => Ok(()),
            Command::Quit => {
                self.is_running = false;
                Ok(())
            }
            cmd => self.board.update(cmd),
        }
    }
}
