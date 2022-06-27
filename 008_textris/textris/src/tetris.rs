use std::fmt::{Display, Formatter, Error};

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
    pub last_updated: i64,

    pub active_piece_down_at: i64,
}

impl TetrisGame {
    pub fn new() -> TetrisGame {
        return TetrisGame {
            board: Board {
                width: 10,
                height:20,
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
                    }
                ],
            },
            active_piece: Piece{
                definition_idx: 1,
                shape_idx: 0,
                x: 3,
                y: 3,
            },
            last_updated: 0,
            active_piece_down_at: 0,
        }
    }

    pub fn move_piece(self: &mut Self, movement: TetrisMove) {
        match movement {
            TetrisMove::Fall => {
                self.active_piece.y += 1;
            },
            TetrisMove::Right => {
                self.active_piece.x += 1;
            },
            TetrisMove::Left => {
                self.active_piece.x -= 1;
            },
            _ => {}
        }
    }

    pub fn update(self: &mut Self, elapsed_ms: i64) {
        // when initial time is 0, is the first time
        // we all update
        let update_every_x_ms = 1000;
        let mut dt = elapsed_ms - self.active_piece_down_at;
        while dt > update_every_x_ms {
            self.active_piece.y += 1;
            self.active_piece_down_at += update_every_x_ms;
            dt -= update_every_x_ms;
        }
        self.last_updated = elapsed_ms;
    }

    pub fn input(self: &mut Self, input: TetrisMove ) {

    }
}
