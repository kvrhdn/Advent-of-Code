use crate::grid::{Dir, Pos};
use crate::intcode::Computer;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use StatusOutput::{AtOxygenSystem, HitAWall, MovedForward};

#[derive(Eq, FromPrimitive, PartialEq)]
pub enum StatusOutput {
    HitAWall = 0,
    MovedForward = 1,
    AtOxygenSystem = 2,
}

pub struct RepairDroid {
    computer: Computer,
    pos: Pos,
    curr_dir: Dir,
}

impl RepairDroid {
    pub fn new_from_input(input: &str) -> Result<Self, &'static str> {
        Ok(Self {
            computer: Computer::new_from_input(input)?,
            pos: Pos::at(0, 0),
            curr_dir: Dir::Left,
        })
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }

    pub fn get_curr_dir(&self) -> Dir {
        self.curr_dir
    }

    pub fn try_move(&mut self, dir: Dir) -> Result<StatusOutput, &'static str> {
        let movement_command = match dir {
            Dir::Up => 1,    // north
            Dir::Down => 2,  // south
            Dir::Left => 4,  // west
            Dir::Right => 3, // east
        };
        self.computer.put_input(movement_command);

        self.computer.run()?;

        let output = self.computer.get_output().ok_or("expected an output")?;
        let output = FromPrimitive::from_i64(output).ok_or("unsupported output code")?;

        match output {
            MovedForward | AtOxygenSystem => {
                self.pos = self.pos.step(dir);
                self.curr_dir = dir;
            }
            HitAWall => {}
        }

        Ok(output)
    }
}
