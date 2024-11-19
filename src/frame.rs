use crate::{SCREEN_NUM_COLS, SCREEN_NUM_ROWS};

pub const X: char = '[';
pub const Y: char = ']';
pub const O: char = ' ';
pub const PIXEL_SIZE: usize = 2;
pub type Frame = [[char; SCREEN_NUM_ROWS]; SCREEN_NUM_COLS];

pub fn new_frame() -> Frame {
    [[0; SCREEN_NUM_ROWS]; SCREEN_NUM_COLS]
}
