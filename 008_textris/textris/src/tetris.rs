pub mod tetris;

// TetrisMove represents a movement for the game
pub enum TetrisMove {
    Left,
    Right,
    Fall,
    RotCW,
    RotCCW,
    Quit
}

pub trait TetrisInput {
    fn Input() -> TetrisMove;
}

pub trait TetrisRender {
    fn Render(game: &TetrisGame);
}

#[derive(Debug)]
pub struct Board {
    width: u16,
    height: u16,
}

//
//
//   *          ****
//  +*++
//   *
//   *
//
//  | 
//  | 
//  | 
//  | 
//
//  | ####
//  |    #
//  | ####
//  | ####
//
//
//  Square:
// -------------
//  12
//  34
//
// 41
// 32
//
//  Stair left
// ------------
//  12
//   34
//
//    1
//   32
//   4
//
//          -- double reflex
//               21
//              43
//
//              43
//               21
//
//          -- double reflex
//               34
//              12
//
//              43
//               21
//
//  Stair right
// --------------
//   12
//  34
//
//   3
//   41
//    2
//
pub struct PieceShape {
    width: u16,
    height: u16,
    charmap: &'static str,
    offset_x: i16,
    offset_y: i16,
}

struct PieceDefinition {
    shapes: [PieceShape;4],
}

struct Piece {
    definition_idx: usize,
    shape_idx: usize,
    x: u16,
    y: u16,
}

struct TetrisGame {
    board:      TetrisBoard,
    piece_set:  PieceSet,

    active_piece: mut Piece,
}

impl TetrisGame {
    pub const fn new() -> TetrisGame {
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
            }
            active_piece: Piece{
                definition_idx: 1,
                shape_idx: 0,
                x: 3,
                y: 3,
            },
        }
    }

    pub fn move_piece(self: &mut Self, movement: TetrisMove) {
        // TODO: use the enum definitions
        match movement {
            if b == up {
                *y -= 1;
            } else if b == down {
                *y += 1;
            } else if b == right {
                *x += 1;
            } else if b == left {
                *x -= 1;
            }
        }
    }

    pub fn update(self: &mut Self, elapsed_ms: i64) {
        // when initial time is 0, is the first time 
        // we all update
        //
        // TODO: animate piece
    }

    pub fn input(self: &mut Self, input: TetrisMove ) {

    }
}
