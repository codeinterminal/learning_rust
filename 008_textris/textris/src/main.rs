use std::io::{stdout, stdin, Read};
use std::thread;

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

    // ---
    enter_screen();

    // TODO: launch this as a background thread
    // draw_thread();

    process_input();

    leave_screen();
}

fn process_input() {
    let mut s = stdin();
    let mut reader = s.lock();

    let mut buf : Vec<u8> = vec![0];
    let mut r = reader.read(&mut buf).expect("gimme a byte");

    stdout().execute(Print(format!("byte: {}   {}",
        buf[0], r))).unwrap();
    thread::sleep_ms(3000);
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

/*
fn draw_thread() {
    for i in 0..10 {
        draw_frame(i);
        thread::sleep_ms(1000);
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
*/
