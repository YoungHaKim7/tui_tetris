use rand::{rngs::OsRng, RngCore};

use crate::frame::{O, X, Y};

pub enum BlockType {
    LeftS,
    RightS,
}

pub struct Block {
    frames: Vec<Vec<Vec<char>>>,
    x: usize,
    y: usize,
    current_frame: i32,
}

impl Block {
    pub fn new(frames: Vec<Vec<Vec<char>>>) -> Self {
        Self {
            frames,
            x: 0,
            y: 0,
            current_frame: 0,
        }
    }
}

pub fn build_block(block_type: BlockType) -> Block {
    match block_type {
        BlockType::RightS => Block::new(vec![
            vec![vec![O, O, X, Y, X, Y], vec![X, Y, X, Y, O, O]],
            vec![vec![X, Y, O, O], vec![X, Y, X, Y], vec![O, O, X, Y]],
            vec![vec![O, O, X, Y, X, Y], vec![X, Y, X, Y, O, O]],
            vec![vec![X, Y, O, O], vec![X, Y, X, Y], vec![O, O, X, Y]],
        ]),
        BlockType::LeftS => Block::new(vec![
            // X '[', Y ']',  O' ',
            // [ ] [ ]   ,     [ ] [ ]
            //        [ ] , [ ], [ ]
            vec![vec![X, Y, X, Y, O, O], vec![O, O, X, Y, X, Y]],
            vec![vec![O, O, X, Y], vec![X, Y, X, Y], vec![X, Y, O, O]],
            vec![vec![X, Y, X, Y, O, O], vec![O, O, X, Y, X, Y]],
            vec![vec![O, O, X, Y], vec![X, Y, X, Y], vec![X, Y, O, O]],
        ]),
    }
}

pub fn randomize_block() -> Block {
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    let random = OsRng.next_u32() % 2;

    match random {
        0 => build_block(BlockType::LeftS),
        1 => build_block(BlockType::RightS),
        _ => unreachable!("Trying to generate an unkown block"),
    }
}
