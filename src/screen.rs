

pub mod figure;

use figure::square::Square;
use figure::position::Direction;
use crate::screen::figure::Figure;

const SCREEN_HEIGHT: u32 = 29;
const SCREEN_WIDTH: u32 = 10;

pub enum Key {None, Up, Down, Left, Right}

pub struct Screen {
    height: u32,
    width: u32,
}

impl Screen {

    pub fn new() -> Self {
        Self {
            height : SCREEN_HEIGHT,
            width : SCREEN_WIDTH,
        }
    }

    pub fn run(&self) {

        let mut square = Square::new();
        let mut dibujado = false;

        loop {

            let key_pressed = Key::None; //HOW TO CATCH THIS IN RUST????

            //CREAL THE CONSOLE (TODO)
            
            let position = square.getPosition();

            match key_pressed {
                Key::Up => position.move_pos(Direction::Up),
                Key::Down => position.move_pos(Direction::Down),
                Key::Left => position.move_pos(Direction::Left),
                Key::Right => position.move_pos(Direction::Right),
                Key::None => {},
            };

            let curr_y = position.get_y();
            let curr_x = position.get_x();

            for y in 0..curr_y {
                println!();
            }

            let bitfield = square.getBitfield();

            for line in bitfield {

                let mut sprint = "          ";

                let mut curr_aux_x = curr_x;

                for field in line {
                    
                    if field { 
                        sprint[curr_aux_x as usize] = 'X'
                    }
                    curr_aux_x += 1;

                }

                println!("{}", sprint);
            }
        }
    }

    pub fn ScreenHeight() -> u32 {
        SCREEN_HEIGHT
    }

    pub fn ScreenWidth() -> u32 {
        SCREEN_WIDTH
    }

    pub fn screenWidth(&self) -> u32 {
        self.width
    }
}
