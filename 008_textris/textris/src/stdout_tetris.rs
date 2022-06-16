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

    fn draw_frame(frame: u32) {
        let col : u16 = frame.try_into().unwrap();
        stdout().execute(Clear(ClearType::All)).unwrap();
        stdout().execute(cursor::MoveToRow(1)).expect("move it");
        stdout().execute( cursor::MoveToColumn(1 + col)).unwrap();
        stdout().execute(Print("*")).unwrap();
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
    }
}

impl TetrisRender for StdTetrisRender {

}
