

use crate::screen::*;

pub enum Direction {Right,Left,Up,Down}

pub struct Position {
    pos_x : u32,
    pos_y : u32,
}

impl Position {

    pub fn new() -> Self {
        Self{
            pos_x : 0,
            pos_y : 0,
        }
    }

    pub fn get_x(&self) -> u32 {
        self.pos_x
    }

    pub fn get_y(&self) -> u32 {
        self.pos_y
    }

    pub fn move_pos(&mut self, direction : Direction) {
        match direction {
            Direction::Right => {
                if self.pos_x < (Screen::ScreenWidth() - 1) {
                    self.pos_x += 1;
                }
            }
            Direction::Left => {
                if self.pos_x > 0 {
                    self.pos_x -= 1;
                }
            }
            Direction::Up => {
                if self.pos_y > 0 {
                    self.pos_y -= 1;
                }
            }
            Direction::Down => {
                if self.pos_y < (Screen::ScreenHeight() - 1) {
                    self.pos_y += 1;
                }
            }
        };
    }
}
