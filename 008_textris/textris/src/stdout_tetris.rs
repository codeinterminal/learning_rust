use crate::tetris::{
    TetrisRender,
    Piece,
    PieceSet,
    PieceShape,
    TetrisGame,
    Board,
};
use std::io::{stdout, Stdout, Write};

use crossterm::{
    // execute,
    ExecutableCommand,
    QueueableCommand,
    style::{
        ResetColor,
        Print,
        SetColors,
        Colors,
        Color,
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

#[derive(Debug)]
struct Screen {
    width: u16,
    height: u16,
}

pub struct StdTetrisRender {
    screen: Screen,
}

impl StdTetrisRender {
    pub fn new() -> StdTetrisRender {
        let (term_width, term_height) = size().expect("has size");
        StdTetrisRender {
            screen: Screen{
                width: term_width,
                height: term_height,
            },
        }
    }

    pub fn init(&mut self) {
        stdout().execute(EnterAlternateScreen).expect("all ok");
        stdout().execute(ResetColor).expect("all ok");
        stdout().execute(cursor::Hide).unwrap();
        enable_raw_mode().unwrap();
    }

    pub fn shutdown(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(cursor::Show).unwrap();
        stdout().execute(LeaveAlternateScreen).expect("all ok");
    }

    fn draw_frame(&mut self, game: &TetrisGame) {
        let mut out = stdout();
        out.queue(Clear(ClearType::All)).unwrap();

        self.draw_board(&mut out, game);
        self.draw_piece(&mut out, &game.active_piece, &game.piece_set);

        out.flush();
    }

    fn draw_piece(&mut self, out: &mut Stdout, piece: &Piece, piece_set: &PieceSet) {
        let colors = vec![
            Color::Red,
            Color::Yellow,
            Color::Green,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
        ];

        out.queue(SetColors(Colors::new(
                Color::Red, colors[piece.definition_idx])));

        let ox : u16 = 4;
        let oy : u16 = 4;

        let x = piece.x + ox;
        let y = piece.y + oy;

        out.queue(cursor::MoveToRow(y)).unwrap();
        out.queue(cursor::MoveToColumn(x)).unwrap();

        let p : &PieceShape = &piece_set.definitions[piece.definition_idx].shapes[piece.shape_idx];

        let xx: u16 = (x as i16 + p.offset_x) as u16;
        let yy: u16 = (y as i16 + p.offset_y) as u16;
        for i in 0..p.height {
            for j in 0..p.width {
                let idx : usize = (p.width * i + j).into();
                let v : &str = &p.charmap[idx..idx+1];
                if v != " " {
                    out.queue(cursor::MoveToRow(yy + i)).unwrap();
                    out.queue(cursor::MoveToColumn(xx + j)).unwrap();
                    out.queue(Print(" ")).unwrap();
                }
            }
        }

        out.queue(ResetColor).unwrap();
    }

    fn draw_board(self: &mut Self, out: &mut Stdout, game: &TetrisGame) {
        let xx : u16 = 4;
        let yy : u16 = 4;

        out.queue(SetColors(Colors::new(
                Color::Red, Color::DarkGrey)));
        for y in 0..game.board.height {
            out.queue(cursor::MoveToRow(yy+y)).unwrap();

            out.queue(cursor::MoveToColumn(xx)).unwrap();
            out.queue(Print(" ")).unwrap();
            out.queue(cursor::MoveToColumn(
                    xx+2+game.board.width)).unwrap();
            out.queue(Print(" ")).unwrap();
        }
        out.queue(cursor::MoveToRow(
                yy+game.board.height)).unwrap();
        out.queue(cursor::MoveToColumn(xx)).unwrap();
        for x in 0..game.board.width+3 {
            out.queue(Print(" ")).unwrap();
        }
        out.queue(ResetColor).unwrap();
    }
}

impl TetrisRender for StdTetrisRender {
    fn render(self: &mut Self, game: &TetrisGame) {
        self.draw_frame(game);
    }
}
