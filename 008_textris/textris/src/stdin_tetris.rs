use std;
// use std::io::{stdin, Read};

use crate::tetris::{
    TetrisInput,
    TetrisMove,
};

pub struct StdTetrisInput {
    stdin: std::io::Stdin,
    reader: &std::io::StdinLock,
    buf: [u8,1],
}

impl StdTetrisInput {
    pub const fn new() -> StdTetrisInput {
        let s = stdin();
        let r = s.lock();
        return StdTetrisInput {
            stdin: s,
            reader: r,
            // buf : Vec<u8> = vec![0];
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

    pub fn input(self: &mut Self) -> Option<TetrisMove> {
        let r = self.reader.read(&mut buf).expect("gimme a byte");
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

