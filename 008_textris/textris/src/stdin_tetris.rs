use std;

use tetris::{
    TetrisInput,
    TetrisMove,
}

pub mod stdin_tetris;



pub struct StdTetrisInput {
    stdin: std::io::Stdin,
    reader: &std::io::Read,
    buf: [u8:1],
}

impl StdTetrisInput {
    pub const fn new() -> StdTetrisInput {
        let s = stdin();
        let r = s.lock();
        return StdTetrisInput {
            stdin: s,
            reader: r,
    let mut buf : Vec<u8> = vec![0];
        }
    }
    // TODO: implement Drop to unlock stdin ?
}

impl TetrisInput for StdTetrisInput {
    // let up = 107;
    let down = 106;
    let right = 108;
    let left = 104;
    let quit = 113;

    pub fn input(self: &Self) -> Option<TetrisMove> {
        let r = reader.read(&mut buf).expect("gimme a byte");
        if r > 0 {
            match buf[0] {
                quit => Some(Quit), 
                right => Some(Right),
                left => Some(Left),
                down => Some(Fall),
                'f' => Some(RotCW),
                'd' => Some(RotCCW),
                _ => None,
            }
        }
    }
}

