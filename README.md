# tui_tetris

# final project
- https://github.com/YoungHaKim7/rusty_game_tetris

# `BlockType::T`
```rs
pub const X: char = '[';
pub const Y: char = ']';
pub const O: char = ' ';
pub const PIXEL_SIZE: usize = 2;


BlockType::T => Block::new(vec![
    vec![vec![O, O, X, Y, O, O], vec![X, Y, X, Y, X, Y]],
    vec![vec![X, Y, O, O], vec![X, Y, X, Y], vec![X, Y, O, O]],
    vec![vec![X, Y, X, Y, X, Y], vec![O, O, X, Y, O, O]],
    vec![vec![O, O, X, Y], vec![X, Y, X, Y], vec![O, O, X, Y]],
]),

// T 모양
//         'ㅗ' 모양 
//         BlockType::T => Block::new(vec![
//             vec![vec![O, O, X, Y, O, O],
            //      vec![X, Y, X, Y, X, Y]],

//         'ㅏ' 모양 
//             vec![vec![X, Y, O, O],
                //  vec![X, Y, X, Y],
                //  vec![X, Y, O, O]],

//         'ㅜ' 모양 
//             vec![vec![X, Y, X, Y, X, Y],
//                  vec![O, O, X, Y, O, O]],

//         'ㅓ' 모양 
//             vec![vec![O, O, X, Y],
//                  vec![X, Y, X, Y],
//                  vec![O, O, X, Y]],

```
