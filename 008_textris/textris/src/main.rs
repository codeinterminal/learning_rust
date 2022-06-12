use std::io::{stdout, stdin, Read};
use std::thread;
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
//   **.   .*
//   .**   **
//         *.
//
// #[Derive(debug)]
// struct Piece {
//     width: i32,
//     height: i32,
//     mask: [u8],
// }


// ? can we do a random over an enum ?
fn main() {
    let (term_width, term_height) = size().expect("has size");
    println!("Screen size: {term_width} , {term_height}");

    enter_screen();

    let (mut tx, mut rx) = mpsc::channel();
    thread::spawn(move || {
        draw_thread(&mut rx);
    });
    process_input(&mut tx);
    leave_screen();
}

fn process_input(tx: &mut mpsc::Sender<u8>) {
    let mut s = stdin();
    let mut reader = s.lock();

    let mut buf : Vec<u8> = vec![0];
    _ = reader.read(&mut buf).expect("gimme a byte");

    while buf[0] != 113 {
        // stdout().execute(Print(format!("byte: {}   {} \n",
        //    buf[0], r))).unwrap();
        let r = reader.read(&mut buf).expect("gimme a byte");
        if r > 0 {
            tx.send(buf[0]);
        }
    }
}

fn enter_screen() {
    stdout().execute(EnterAlternateScreen).expect("all ok");
    stdout().execute(ResetColor).expect("all ok");
    stdout().execute(cursor::Hide).unwrap();
    enable_raw_mode();
}

fn leave_screen() {
    disable_raw_mode();
    stdout().execute(cursor::Show).unwrap();
    stdout().execute(LeaveAlternateScreen).expect("all ok");
}

fn draw_thread(rx: &mut mpsc::Receiver<u8>) {
    let mut i = 0;

    let mut x: u16 = 30;
    let mut y: u16 = 15;

    loop {
        draw_frame(i);
        if let Ok(c) = rx.try_recv() {
            move_piece(c, &mut x, &mut y);
        }
        draw_piece(x, y);

        thread::sleep_ms(16);
        i = (i + 1) % 20;
    }
}

fn draw_frame(frame: u32) {
    stdout().execute(Clear(ClearType::All)).unwrap();
    stdout().execute(cursor::MoveToRow(1)).expect("move it");
    stdout().execute(cursor::MoveToColumn(1)).unwrap();
    for j in 0..frame {
        stdout().execute(Print(" ")).unwrap();
    }
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

fn draw_piece(x: u16, y: u16) {
    stdout().execute(cursor::MoveToRow(y)).expect("move it");
    stdout().execute(cursor::MoveToColumn(x)).unwrap();

    stdout().execute(Print("#")).unwrap();
}
