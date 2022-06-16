use tetris::{TetrisRender};

pub mod stdout_tetris;

#[derive(Debug)]
struct Screen {
    width: u16,
    height: u16,
}

struct StdTetrisRender {
    screen: Screen,
}

impl StdTetrisRender {
    pub const fn new() -> StdTetrisRender {
        let (term_width, term_height) = size().expect("has size");
        StdTetrisRender {
            screen: Screen{
                width: term_width,
                height: term_height,
            },
        }
    }

    pub fn init() {
        stdout().execute(EnterAlternateScreen).expect("all ok");
        stdout().execute(ResetColor).expect("all ok");
        stdout().execute(cursor::Hide).unwrap();
        enable_raw_mode().unwrap();
    }

    pub fn shutdown() {
        disable_raw_mode().unwrap();
        stdout().execute(cursor::Show).unwrap();
        stdout().execute(LeaveAlternateScreen).expect("all ok");
    }
}

impl TetrisRender for StdTetrisRender {

}
