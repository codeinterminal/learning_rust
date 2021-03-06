use std::io::{stdout, stdin, Read};
use std::{thread, time};
use std::fmt::{Display, Formatter, Error};
use std::sync::mpsc;


use crossterm::{
    // execute,
    ExecutableCommand,
    style::{
        ResetColor,
        Print,
    },
    terminal::{
        size,
        enable_raw_mode,
        disable_raw_mode,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    cursor,
};

// Example:
//
// width: 3, hei
//   **     *
//    **   **
//         *

#[derive(Debug)]
struct Screen {
    width: u16,
    height: u16,
}

#[derive(Debug)]
struct Board {
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
struct PieceShape {
    width: u16,
    height: u16,
    charmap: &'static str,
    offset_x: i16,
    offset_y: i16,
}

struct PieceDefinition {
    // shapes: Vec<PieceShape>,
    //
    //
    //
    //
    //
    // TODO: check if we can use this
    //
    //
    shapes: [PieceShape;4],
}

struct Piece {
    definition_idx: usize,
    shape_idx: usize,
    x: u16,
    y: u16,
}

struct PieceSet {
    definitions: Vec<PieceDefinition>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&*format!("B: {}x{}", self.width, self.height));
        Ok(())
    }
}

// ? can we do a random over an enum ?
fn main() {
    let (term_width, term_height) = size().expect("has size");

    let screen = Screen{
        width: term_width,
        height: term_height,
    };

    let board = Board {
        width: 10,
        height:20,
    };

    let piece_set = PieceSet {
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
    };


    println!("Screen: {:?} , {}", screen, board);

    enter_screen();

    let (mut tx, mut rx) = mpsc::channel();
    thread::spawn(move || {
        draw_thread(&mut rx, &piece_set);
    });
    process_input(&mut tx);
    leave_screen();
}

fn process_input(tx: &mut mpsc::Sender<u8>) {
    let s = stdin();
    let mut reader = s.lock();
    let mut buf : Vec<u8> = vec![0];
    while buf[0] != 113 {
        let r = reader.read(&mut buf).expect("gimme a byte");
        if r > 0 {
            _ = tx.send(buf[0]);
        }
    }
}

fn enter_screen() {
    stdout().execute(EnterAlternateScreen).expect("all ok");
    stdout().execute(ResetColor).expect("all ok");
    stdout().execute(cursor::Hide).unwrap();
    enable_raw_mode().unwrap();
}

fn leave_screen() {
    disable_raw_mode().unwrap();
    stdout().execute(cursor::Show).unwrap();
    stdout().execute(LeaveAlternateScreen).expect("all ok");
}

fn draw_thread(rx: &mut mpsc::Receiver<u8>,
    piece_set: &PieceSet) {

    let mut i = 0;

    let mut x: u16 = 30;
    let mut y: u16 = 15;

    let test_piece = Piece{
        definition_idx: 1,
        shape_idx: 0,
        x: 3,
        y: 3,
    };

    let sleep_ms = time::Duration::from_millis(16);
    loop {
        draw_frame(i);
        if let Ok(c) = rx.try_recv() {
            move_piece(c, &mut x, &mut y);
        }
        // draw_piece(x, y, &test_piece, &piece_set);
        draw_piece(4, 4, &test_piece, &piece_set);
        thread::sleep(sleep_ms);
        i = (i + 1) % 20;
    }
}

fn draw_frame(frame: u32) {
    let col : u16 = frame.try_into().unwrap();
    stdout().execute(Clear(ClearType::All)).unwrap();
    stdout().execute(cursor::MoveToRow(1)).expect("move it");
    stdout().execute( cursor::MoveToColumn(1 + col)).unwrap();
    stdout().execute(Print("*")).unwrap();
}

fn move_piece(b: u8, x: &mut u16, y: &mut u16) {
    // move and display the piece
    let up = 107;
    let down = 106;
    let right = 108;
    let left = 104;

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

fn draw_piece(x: u16, y: u16, piece: &Piece, piece_set: &PieceSet) {
    stdout().execute(cursor::MoveToRow(y)).expect("move it");
    stdout().execute(cursor::MoveToColumn(x)).unwrap();

    let p : &PieceShape = &piece_set.definitions[piece.definition_idx].shapes[piece.shape_idx];

    let xx: u16 = (x as i16 + p.offset_x) as u16;
    let yy: u16 = (y as i16 + p.offset_y) as u16;
    for i in 0..p.height {
        for j in 0..p.width {
            let idx : usize = (p.width * i + j).into();
            let v : &str = &p.charmap[idx..idx+1];
            if v != " " {
                stdout().execute(cursor::MoveToRow(yy + i)).unwrap();
                stdout().execute(cursor::MoveToColumn(xx + j)).unwrap();
                stdout().execute(Print(v)).unwrap();
            }
        }
    }

    // stdout().execute(Print(p.charmap)).unwrap();
}


