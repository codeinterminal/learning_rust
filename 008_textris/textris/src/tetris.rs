use std::fmt::{Display, Formatter, Error};
use rand::prelude::*;
use rand::rngs::SmallRng;

// TetrisMove represents a movement for the game
pub enum TetrisMove {
    Left,
    Right,
    Fall,
    RotCW,
    RotCCW,
    Quit,
    Nothing,
}

pub trait TetrisInput {
    fn input(&mut self) -> TetrisMove;
}

pub trait TetrisRender {
    fn render(&mut self, game: &TetrisGame);
}

#[derive(Debug)]
pub struct Board {
    pub width: u16,
    pub height: u16,

    pub debris: Vec<usize>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&*format!("B: {}x{}", self.width, self.height));
        Ok(())
    }
}

pub struct PieceShape {
    pub width: u16,
    pub height: u16,
    pub charmap: &'static str,
    pub offset_x: i16,
    pub offset_y: i16,
}

pub struct PieceDefinition {
    pub shapes: [PieceShape;4],
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub definition_idx: usize,
    pub shape_idx: usize,
    pub x: u16,
    pub y: u16,
}

pub struct PieceSet {
    pub definitions: Vec<PieceDefinition>,
}

pub struct TetrisGame {
    pub board:      Board,
    pub piece_set:  PieceSet,

    pub active_piece: Piece,
    pub next_piece: Piece,

    pub last_updated: i64,

    pub active_piece_down_at: i64,

    rnd : SmallRng,
}

pub const NO_DEBRIS: usize = 100;

impl TetrisGame {
    pub fn new() -> TetrisGame {
        let mut tg = TetrisGame {
            board: Board {
                width: 10,
                height:20,
                debris: vec![NO_DEBRIS; 10*20],
            },
            piece_set: PieceSet {
                definitions: vec![
                    PieceDefinition {
                        shapes: [
                            PieceShape{
                                width: 2,
                                height: 2,
                                charmap: "****",
                                offset_x: 0,
                                offset_y: 0,
                            },
                            PieceShape{
                                width: 2,
                                height: 2,
                                charmap: "****",
                                offset_x: 0,
                                offset_y: 0,
                            },
                            PieceShape{
                                width: 2,
                                height: 2,
                                charmap: "****",
                                offset_x: 0,
                                offset_y: 0,
                            },
                            PieceShape{
                                width: 2,
                                height: 2,
                                charmap: "****",
                                offset_x: 0,
                                offset_y: 0,
                            },
                        ],
                    },
                    PieceDefinition {
                        shapes: [
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: "**  **",
                                offset_x: -1,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: " **** ",
                                offset_x: 0,
                                offset_y: -1,
                            },
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: "**  **",
                                offset_x: -1,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: " **** ",
                                offset_x: 0,
                                offset_y: -1,
                            },
                        ]
                    },
                    PieceDefinition {
                        shapes: [
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: " **** ",
                                offset_x: -1,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: "* ** *",
                                offset_x: 0,
                                offset_y: -1,
                            },
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: " **** ",
                                offset_x: -1,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: "* ** *",
                                offset_x: 0,
                                offset_y: -1,
                            },
                        ]
                    },
                    PieceDefinition {
                        shapes: [
                            PieceShape {
                                width: 4,
                                height: 1,
                                charmap: "****",
                                offset_x: -1,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 1,
                                height: 4,
                                charmap: "****",
                                offset_x: 0,
                                offset_y: -1,
                            },
                            PieceShape {
                                width: 4,
                                height: 1,
                                charmap: "****",
                                offset_x: -1,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 1,
                                height: 4,
                                charmap: "****",
                                offset_x: 0,
                                offset_y: -1,
                            },
                        ]
                    },
                    PieceDefinition {
                        shapes: [
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: "*** * ",
                                offset_x: 0,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: " *** *",
                                offset_x: 0,
                                offset_y: -1,
                            },
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: " * ***",
                                offset_x: 0,
                                offset_y: -1,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: "* *** ",
                                offset_x: 0,
                                offset_y: -1,
                            },
                        ]
                    },
                    PieceDefinition {
                        shapes: [
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: "*  ***",
                                offset_x: 0,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: "*** * ",
                                offset_x: 0,
                                offset_y: -1,
                            },
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: "***  *",
                                offset_x: 0,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: " * ***",
                                offset_x: 0,
                                offset_y: -1,
                            },
                        ]
                    },
                    PieceDefinition {
                        shapes: [
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: "  ****",
                                offset_x: 0,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: "* * **",
                                offset_x: 0,
                                offset_y: -1,
                            },
                            PieceShape {
                                width: 3,
                                height: 2,
                                charmap: "****  ",
                                offset_x: 0,
                                offset_y: 0,
                            },
                            PieceShape {
                                width: 2,
                                height: 3,
                                charmap: "** * *",
                                offset_x: 0,
                                offset_y: -1,
                            },
                        ]
                    }
                ],
            },
            active_piece: Piece{
                definition_idx: 6,
                shape_idx: 0,
                x: 3,
                y: 3,
            },
            next_piece: Piece{
                definition_idx: 6,
                shape_idx: 0,
                x: 1,
                y: 1,
            },
            last_updated: 0,
            active_piece_down_at: 0,
            rnd: SmallRng::from_entropy(),
        };

        tg.active_piece.definition_idx = tg.rnd.gen_range(0..7);
        tg.next_piece.definition_idx = tg.rnd.gen_range(0..7);

        return tg;
    }

    pub fn update(self: &mut Self, elapsed_ms: i64) {
        // when initial time is 0, is the first time
        // we all update
        let update_every_x_ms = 1000;
        let mut dt = elapsed_ms - self.active_piece_down_at;

        // TODO: update_every_x_ms should be named something
        // related to the drop speed (or drop delay)
        while dt > update_every_x_ms {
            if self.will_collide(0, 1) {
                // leave the debris in the board
                let b = &mut self.board;

                let p : &PieceShape = &self.piece_set.definitions[
                    self.active_piece.definition_idx].shapes[
                    self.active_piece.shape_idx];

                for i in 0..p.height {
                    for j in 0..p.width {
                        let cidx : usize = (p.width * i + j).into();
                        let v : &str = &p.charmap[cidx..cidx+1];
                        if v != " " {
                            let idx = b.width as i16 *
                                (i as i16 + self.active_piece.y as i16 + p.offset_y)
                                + (j as i16 + self.active_piece.x as i16 + p.offset_x);

                            b.debris[idx as usize] = self.active_piece.definition_idx;
                        }
                    }
                }

                // TODO: check the debris to delete
                self.active_piece = self.next_piece;
                self.active_piece.x = 5;
                self.active_piece.y = 3;

                self.next_piece = Piece {
                    definition_idx: self.rnd.gen_range(0..7),
                    shape_idx: 0,
                    x: 1,
                    y: 1,
                }
            } else {
                self.active_piece.y += 1;
                self.active_piece_down_at += update_every_x_ms;
            }
            dt -= update_every_x_ms;
        }
        self.last_updated = elapsed_ms;
    }

    fn will_collide(self: &mut Self, xoff: i16, yoff: i16) -> bool {
        let mut x = (self.active_piece.x as i16) + xoff;
        let mut y = (self.active_piece.y as i16) + yoff;

        let pc = &self.active_piece;
        let p : &PieceShape = &self.piece_set
            .definitions[pc.definition_idx]
            .shapes[pc.shape_idx];

        let xx = x + p.offset_x;
        let yy = y + p.offset_y;

        for i in 0..p.height {
            for j in 0..p.width {
                let idx : usize = (p.width * i + j).into();
                let v : &str = &p.charmap[idx..idx+1];
                let px = xx + j as i16;
                let py = yy + i as i16;
                if v != " " {
                    // check with board limits:
                    if px < 0 {
                        return true;
                    }
                    if px >= self.board.width as i16 {
                        return true;
                    }

                    if py >= self.board.height as i16 {
                        return true;
                    }
                    // TODO: check debrise collision
                }
            }
        }
        return false
    }

    pub fn input(self: &mut Self, input: TetrisMove ) {
        match input {
            TetrisMove::Fall => {
                if !self.will_collide(0, 1) {
                    self.active_piece.y += 1;
                }
            },
            TetrisMove::Right => {
                if !self.will_collide(1, 0) {
                    self.active_piece.x += 1;
                }
            },
            TetrisMove::Left => {
                if !self.will_collide(-1, 0) {
                    self.active_piece.x -= 1;
                }
            },
            TetrisMove::RotCW => {
                if self.active_piece.shape_idx < 3 {
                    self.active_piece.shape_idx += 1;
                } else {
                    self.active_piece.shape_idx = 0;
                }
            },
            TetrisMove::RotCCW => {
                if self.active_piece.shape_idx > 0 {
                    self.active_piece.shape_idx -= 1;
                } else {
                    self.active_piece.shape_idx = 3;
                }
            },
            _ => {}
        }
    }
}
