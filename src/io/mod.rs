use crate::{board::Board, UnitResult};

pub mod console;
pub mod window;

#[derive(Debug)]
pub enum Command {
    None,
    Move {
        start_file: u8,
        start_rank: u8,
        end_file: u8,
        end_rank: u8,
    },
    Quit,
}

impl From<Vec<&str>> for Command {
    fn from(value: Vec<&str>) -> Self {
        if value.len() == 4 {
            Self::Move {
                start_file: 1,
                start_rank: 1,
                end_file: 1,
                end_rank: 1,
            }
        } else if value.len() == 1 && value[0] == "quit" {
            Self::Quit
        } else {
            Self::None
        }
    }
}

pub trait IO {
    fn get_command(&mut self, board: &Board) -> Result<Command, String>;
    fn render(&mut self, board: &Board) -> UnitResult;
}
