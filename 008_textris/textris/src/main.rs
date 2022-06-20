use std::io::{stdout, stdin, Read};
use std::{thread, time};
use std::fmt::{Display, Formatter, Error};
use std::sync::mpsc;

use tetris::{
    TetrisGame,
    TetrisInput,
    TetrisRender,
};

use stdout_tetris::{
    StdTetrisRender,
};

use stdin_tetris::{
    StdTetrisInput,
};

fn main() {
    let tetris_game = TetrisGame::new();
    let tetris_render = StdTetrisRender::new();
    let tetris_input = StdTetrisInput::new();

    tetris_render.init();

    let (mut tx, mut rx) = mpsc::channel();
    thread::spawn(move || {
        game_thread(&mut rx, tetris_game, tetris_render);
    });

    process_input(&mut tx);

    // tetris_render.shutdown();

    // dirty hack to make sure that the thread has finished
    // TODO: check how to to wait for the thread to finish
    let sleep_ms = time::Duration::from_millis(300);
    thread::sleep(sleep_ms);
}

fn process_input(tx: &mut mpsc::Sender<TetrisMove>,
                 input: &(impl TetrisInput)) {
    let nextInput = tetris::TetrisMove;
    while nextInput != tetris::TetrisMove::Quit {
        if let(tm) = input.input() {
            _ = tx.send(tm);
            nextInput = tm;
        }
    }
}

fn game_thread(rx: &mut mpsc::Receiver<TetrisMove>,
    tetris_game: &mut tetris::TetrisGame,
    renderer: &mut tetris::TetrisRender
    ) {

    let mut i = 0;

    // TODO: get the time
    let initial_time = time::Time();
    let mut cur_time = initial_time;
    let sleep_ms = time::Duration::from_millis(16);

    let mut action : Option<tetris::TetrisMove> = Option(None)
    while action != Some(Quit) {
        if let Ok(c) = rx.try_recv() {
            tetris_game.Input(c);
            action = Some(c);
        }
        tetris_game.Update(cur_time - initial_time);
        tetris_game.Render();
        thread::sleep(sleep_ms);
        cur_time = time.Time();
    }

    tetris_render.shutdown();
}
