use rusty_time::Timer;

use crate::block::{randomize_block, Block};

const BOARD_NUM_COLS: usize = 20;
const BOARD_NUM_ROWS: usize = 20;

pub enum Direction {
    Down,
    Left,
    Right,
}

pub struct Board {
    grid: Vec<Vec<bool>>,
    current_block: Block,
    next_block: Block,
    move_timer: Timer,
    top_x: usize,
    top_y: usize,
    current_speed: u64,
    debug_mode: bool,
}

impl Board {
    pub fn new(debug_mode: bool) -> Self {
        Self {
            grid: vec![vec![false; BOARD_NUM_COLS]; BOARD_NUM_ROWS],
            current_block: randomize_block(),
        }
    }
}
