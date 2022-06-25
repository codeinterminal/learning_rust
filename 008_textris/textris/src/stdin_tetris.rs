use std;
use std::io::{stdin, Read};

use crate::tetris::{
    TetrisInput,
    TetrisMove,
};

pub struct StdTetrisInput {
    stdin: std::io::Stdin,
    buf: [u8;1],
}

impl StdTetrisInput {
    pub fn new() -> StdTetrisInput {
        let s = stdin();
        return StdTetrisInput {
            stdin: s,
            buf: [0],
        }
    }
    // TODO: implement Drop to unlock stdin ?
}

impl TetrisInput for StdTetrisInput {
    fn input(self: &mut Self) -> TetrisMove {
        const QUIT : u8 = 113;
        const RIGHT : u8 = 108;
        const LEFT : u8 = 104;
        const DOWN : u8 = 106;
        const ROTCW : u8 = 'f' as u8;
        const ROTCCW : u8 = 'd' as u8;

        let mut reader = self.stdin.lock();
        let r = reader.read(&mut self.buf).expect("gimme a byte");
        if r > 0 {
            match self.buf[0] {
                QUIT => TetrisMove::Quit,
                RIGHT => TetrisMove::Right,
                LEFT => TetrisMove::Left,
                DOWN => TetrisMove::Fall,
                ROTCW => TetrisMove::RotCW,
                ROTCCW => TetrisMove::RotCCW,
                _ => TetrisMove::Nothing,
            }
        } else {
            TetrisMove::Nothing
        }
    }
}

