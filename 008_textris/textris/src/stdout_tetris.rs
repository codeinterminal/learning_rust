use crate::tetris::{Board, Piece, PieceSet, PieceShape, TetrisGame, TetrisRender, NO_DEBRIS};
use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor,
    style::{Color, Colors, Print, ResetColor, SetColors},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    Command,
    // execute,
    ExecutableCommand,
    QueueableCommand,
};

#[derive(Debug)]
struct Screen {
    width: u16,
    height: u16,
    offset_x: u16,
    offset_y: u16,
    zoom: u16,
}

const MIN_WIDTH_MARGIN: u16 = 10;
const MIN_HEIGHT_MARGIN: u16 = 2;

impl Screen {
    pub fn new(term_width: u16, term_height: u16, board_width: u16, board_height: u16) -> Screen {
        let width = board_width + MIN_WIDTH_MARGIN;
        let height = board_height + MIN_HEIGHT_MARGIN;

        let w_max_zoom = term_width / (width);
        let h_max_zoom = term_height / (height);
        let mut zoom = w_max_zoom;
        if zoom > h_max_zoom {
            zoom = h_max_zoom
        }
        // TODO: if zoom == 0, this should panic
        if zoom == 0 {
            zoom = 1
        }

        // TODO: use the zoom parameter when preparing commands
        let offset_x = (term_width - (width * zoom)) / 2;
        let offset_y = (term_height - (height * zoom)) / 2;

        Screen {
            width: width,
            height: height,
            offset_x: offset_x,
            offset_y: offset_y,
            zoom: zoom,
        }
    }

    fn stride(&self, len: usize) -> String {
        let mut s = String::with_capacity(len);
        for n in 0..len {
            s.push(' ');
        }
        s
    }

    fn colorline(&self, out: &mut Stdout, length: usize, color_idx: usize) {
        if length == 0 {
            return;
        }
        let palette = vec![
            Color::Red,
            Color::Yellow,
            Color::Green,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
            Color::DarkGrey,
            Color::Black,
        ];
        let mut idx = color_idx;
        if color_idx >= palette.len() {
            idx = palette.len() - 1;
        }
        let sc = SetColors(Colors::new(Color::Red, palette[idx]));
        out.queue(sc);
        out.queue(Print(self.stride(length)));
    }

    pub fn renderx(&self, out: &mut Stdout, game: &TetrisGame) {
        // move to the start of the first line that needs to
        // be drawn
        let zoom = self.zoom as usize;
        /*
        out.queue(cursor::MoveToRow(self.offset_y));
        out.queue(cursor::MoveToColumn(1 + self.offset_x));
        */

        // draw board lines
        for by in 0..game.board.height {
            // draw board left column
            for zy in 0..self.zoom {
                out.queue(cursor::MoveToRow(self.offset_y + by * self.zoom + zy));
                out.queue(cursor::MoveToColumn(1 + self.offset_x));
                self.colorline(out, zoom, 7);

                let line_start = (by * game.board.width) as usize;
                let mut prev = game.board.debris[line_start];
                let mut cur = prev;

                let mut stride_count = 0usize;
                let mut sl = 0usize;
                let mut sc = 0usize;
                let mut bx = 0;
                while bx < game.board.width {
                    (sl, sc) = self.cur_piece_stride(&game, bx, by);
                    if sl > 0 {
                        bx += sl as u16;
                        if sc == prev {
                            stride_count += sl;
                        } else {
                            self.colorline(out, stride_count, prev);
                            stride_count = sl;
                            prev = sc;
                        }
                    }

                    cur = game.board.debris[line_start + bx as usize];
                    if cur == prev {
                        stride_count += 1;
                    } else {
                        self.colorline(out, zoom * stride_count, prev);
                        stride_count = 1;
                        prev = cur;
                    }
                    bx += 1;
                }
                self.colorline(out, zoom * stride_count, prev);
                // draw board right column
                self.colorline(out, zoom, 7);

                // TODO: draw the caracters until the end of the screen to
                // avoid a move scape code
                // sl, sc = self.preview_box_stride(game, bx, by) {

                // draw preview box
            }
        }

        // TODO: draw bottom line
        //
        for i in 0..self.zoom {
            out.queue(cursor::MoveToRow(
                self.offset_y + i + self.zoom * game.board.height,
            ));
            out.queue(cursor::MoveToColumn(1 + self.offset_x));
            self.colorline(out, zoom * ((game.board.width + 2) as usize), 7);
        }
    }

    // returns the lenght and color of a stride, it returns 0, if at the current
    // point there is nothing to draw.
    fn cur_piece_stride(&self, game: &TetrisGame, x: u16, y: u16) -> (usize, usize) {
        (0, 0)
    }

    fn preview_box_stride(&self, game: &TetrisGame, x: u16, y: u16) -> (usize, usize) {
        (0, 0)
    }
}

pub struct StdTetrisRender {
    screen: Screen,
}

struct ColorLine {
    color: SetColors,
    length: usize,
}

impl StdTetrisRender {
    pub fn new() -> StdTetrisRender {
        let (term_width, term_height) = size().expect("has size");
        // TODO: check min size to see the board
        //
        // TODO: remove the hardcode size of the board
        let board_width = 10u16;
        let board_height = 20u16;

        if (board_width + 10) >= term_width || (board_height + 2) >= term_height {
            panic!("terminal too small");
        }

        StdTetrisRender {
            screen: Screen::new(term_width, term_height, board_width, board_height),
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
        /*
        out.queue(Clear(ClearType::All)).unwrap();
        self.draw_board(&mut out, game);
        self.draw_piece(&mut out, &game.active_piece, &game.piece_set);
        self.draw_next_piece(&mut out, &game.next_piece, &game.piece_set);
        */

        self.screen.renderx(&mut out, &game);
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
            Color::Red,
            colors[piece.definition_idx],
        )));

        // TODO: put the draw origin in some field in the struct
        let ox: u16 = 4 + 1;
        let oy: u16 = 4;

        let x = piece.x + ox;
        let y = piece.y + oy;

        out.queue(cursor::MoveToRow(y)).unwrap();
        out.queue(cursor::MoveToColumn(x)).unwrap();

        let p: &PieceShape = &piece_set.definitions[piece.definition_idx].shapes[piece.shape_idx];

        let xx: u16 = (x as i16 + p.offset_x) as u16;
        let yy: u16 = (y as i16 + p.offset_y) as u16;
        for i in 0..p.height {
            for j in 0..p.width {
                let idx: usize = (p.width * i + j).into();
                let v: &str = &p.charmap[idx..idx + 1];
                if v != " " {
                    out.queue(cursor::MoveToRow(yy + i)).unwrap();
                    out.queue(cursor::MoveToColumn(xx + j)).unwrap();
                    out.queue(Print(" ")).unwrap();
                }
            }
        }

        out.queue(ResetColor).unwrap();
    }

    fn draw_next_piece(&mut self, out: &mut Stdout, piece: &Piece, piece_set: &PieceSet) {
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
            Color::Red,
            colors[piece.definition_idx],
        )));

        let ox: u16 = 20;
        let oy: u16 = 4;

        let x = piece.x + ox;
        let y = piece.y + oy;

        out.queue(cursor::MoveToRow(y)).unwrap();
        out.queue(cursor::MoveToColumn(x)).unwrap();

        let p: &PieceShape = &piece_set.definitions[piece.definition_idx].shapes[piece.shape_idx];

        let xx: u16 = (x as i16 + p.offset_x) as u16;
        let yy: u16 = (y as i16 + p.offset_y) as u16;
        for i in 0..p.height {
            for j in 0..p.width {
                let idx: usize = (p.width * i + j).into();
                let v: &str = &p.charmap[idx..idx + 1];
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
        let xx: u16 = 4;
        let yy: u16 = 4;

        out.queue(SetColors(Colors::new(Color::Red, Color::DarkGrey)));
        for y in 0..game.board.height {
            out.queue(cursor::MoveToRow(yy + y)).unwrap();

            out.queue(cursor::MoveToColumn(xx)).unwrap();
            out.queue(Print(" ")).unwrap();
            out.queue(cursor::MoveToColumn(xx + 1 + game.board.width))
                .unwrap();
            out.queue(Print(" ")).unwrap();
        }
        out.queue(cursor::MoveToRow(yy + game.board.height))
            .unwrap();
        out.queue(cursor::MoveToColumn(xx)).unwrap();
        for x in 0..game.board.width + 2 {
            out.queue(Print(" ")).unwrap();
        }

        // draw the debris
        let colors = vec![
            Color::Red,
            Color::Yellow,
            Color::Green,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
        ];
        for dy in 0..game.board.height {
            for dx in 0..game.board.width {
                let idx = (dy * game.board.width + dx) as usize;
                let def_idx = game.board.debris[idx];
                if def_idx != NO_DEBRIS {
                    out.queue(cursor::MoveToRow(yy + dy)).unwrap();
                    out.queue(cursor::MoveToColumn(xx + dx + 1)).unwrap();
                    out.queue(SetColors(Colors::new(Color::Red, colors[def_idx])));
                    out.queue(Print(" ")).unwrap();
                }
            }
        }
        out.queue(ResetColor).unwrap();
    }
}

impl TetrisRender for StdTetrisRender {
    fn render(self: &mut Self, game: &TetrisGame) {
        self.draw_frame(game);
    }
}
