
pub mod square;
pub mod position;

use position::Position;

pub trait Figure {
    fn getPosition(&mut self) -> Position;
    fn getHeight(&self) -> u32;
    fn getWidth(&self) -> u32;
    //(TODO falta get bitfield)
}

