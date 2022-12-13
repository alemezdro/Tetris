


use crate::figure::Figure;
use crate::figure::position::Position;

const SQUARE_HEIGHT : usize = 2;
const SQUARE_WIDTH : usize = 2;

pub struct Square {
    height : u32,
    width : u32,
    position : Position,
    bit_field : [[bool;SQUARE_WIDTH];SQUARE_HEIGHT]
}

impl Square {

    pub fn new() -> Self {
        Self {
            height : SQUARE_HEIGHT as u32,
            width : SQUARE_WIDTH as u32,
            position : Position::new(),
            bit_field : [[true, true], [true, true]],
        }
    }

    pub fn getBitfield(&self) -> [[bool;SQUARE_WIDTH];SQUARE_HEIGHT] {
        self.bit_field
    }
}

impl Figure for Square {

    fn getPosition(&mut self) -> Position {
        self.position
    }

    fn getHeight(&self) -> u32 {
        self.height
    }

    fn getWidth(&self) -> u32 {
        self.width
    }
}
