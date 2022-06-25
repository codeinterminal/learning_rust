use std::io::{stdout, stdin, Read};
use std::{thread, time};
use std::fmt::{Display, Formatter, Error};
use std::sync::mpsc;

use textris::tetris::{
    TetrisGame,
    TetrisInput,
    TetrisRender,
    TetrisMove,
};

use textris::stdout_tetris::{
    StdTetrisRender,
};

use textris::stdin_tetris::{
    StdTetrisInput,
};

fn main() {
    let tetris_game = TetrisGame::new();
    let tetris_render = StdTetrisRender::new();
    let tetris_input = StdTetrisInput::new();

    tetris_render.init();

    let (mut tx, mut rx) = mpsc::channel();
    thread::spawn(move || {
        game_thread(&mut rx, &mut tetris_game, &mut tetris_render);
        tetris_render.shutdown()
    });

    process_input(&mut tx, &mut tetris_input);

    // tetris_render.shutdown();

    // dirty hack to make sure that the thread has finished
    // TODO: check how to to wait for the thread to finish
    let sleep_ms = time::Duration::from_millis(300);
    thread::sleep(sleep_ms);
}

fn process_input(tx: &mut mpsc::Sender<TetrisMove>,
                 input: &mut impl TetrisInput) {
    let nextInput = TetrisMove::Nothing;
    while nextInput != TetrisMove::Quit {
        nextInput = input.input();
        _ = tx.send(nextInput);
    }
}

fn game_thread(rx: &mut mpsc::Receiver<TetrisMove>,
    tetris_game: &mut TetrisGame,
    renderer: &mut dyn TetrisRender
    ) {

    let mut i = 0;

    // TODO: get the time
    let initial_time = time::Instant::now();

    let sleep_ms = time::Duration::from_millis(16);

    // first render without any input
    tetris_game.update(0);
    renderer.render(tetris_game);
    thread::sleep(sleep_ms);

    let mut action = TetrisMove::Nothing;
    while action != TetrisMove::Quit {
        if let Ok(action) = rx.try_recv() {
            tetris_game.input(action);
        }
        let elapsed_ms = time::Instant::now()
            .duration_since(initial_time).as_millis();
        let cur_time : i64 = elapsed_ms.try_into().unwrap();
        tetris_game.update(cur_time);
        renderer.render(tetris_game);
        thread::sleep(sleep_ms);
    }
}
