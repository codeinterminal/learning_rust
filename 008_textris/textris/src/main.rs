use std::io::{stdout, stdin, Read};
use std::{thread, time};
use std::fmt::{Display, Formatter, Error};
use std::sync::mpsc;

use tetris::{TetrisGame};

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

    let tetris_game = TetrisGame::new();
    let tetris_render = StdTetrisRender::new();
    let tetris_input = StdTetrisInput::new();


    println!("Screen: {:?} , {}", screen, board);

    tetris_render.init();
    enter_screen();

    let (mut tx, mut rx) = mpsc::channel();
    thread::spawn(move || {
        game_thread(&mut rx, &piece_set);
    });
    process_input(&mut tx);

    tetris_render.shutdown();
}

fn process_input(tx: &mut mpsc::Sender<TetrisMove>,
                 input: impl &TetrisInput) {
    let nextInput = tetris::TetrisMove;
    while !buf[0] != 113 {
        let r = reader.read(&mut buf).expect("gimme a byte");
        if r > 0 {
            _ = tx.send(buf[0]);
        }
    }
}

fn game_thread(rx: &mut mpsc::Receiver<TetrisMove>,
    tetris_game: &tetris::TetrisGame,
    renderer: &tetris::TetrisRender
    ) {

    let mut i = 0;

    // TODO: get the time
    let initial_time = time::Time();
    let mut cur_time = initial_time;
    let sleep_ms = time::Duration::from_millis(16);
    loop {
        if let Ok(c) = rx.try_recv() {
            tetris_game.Input(c);
        }
        tetris_game.Update(cur_time - initial_time);
        tetris_game.Render()
        thread::sleep(sleep_ms);
        cur_time = time.Time();
    }
}
