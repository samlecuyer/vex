extern crate rustbox;

use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::fs::File;
use std::error::Error;
use std::cmp;

use editor::buffer::Buffer;
use super::{Direction};
use super::{Command, Span, Motion, Line, Column};
use editor::command::{goto, scroll};

use self::rustbox::{Color, RustBox, Key, Event};

enum Mode {
    // we treat `:` as different from normal
    Colon,
    Normal,
    Insert,
}

pub struct State {
    mode: Mode,
    height: usize,
    width: usize,
    buf_idx: usize,
    buffers: Vec<Buffer>,
    count: String,
    status: String,
    colon: String,
}

impl State {
    pub fn new(w: usize, h: usize) -> State {
        State{
            mode: Mode::Normal,
            width: w,
            height: h,
            buf_idx: 0,
            buffers: Vec::new(),
            count: String::new(),
            status: String::new(),
            colon: String::new(),
        }
    }
    
    pub fn open(&mut self, filename: &Path) {
        // assume buffers are 100% width and miss 1 line on top and bottom
        let mut buffer = Buffer::new(filename, self.width, self.height-2);
        match File::open(filename) {
            Ok(f) => {
                let reader = BufReader::new(f);
                buffer.load_reader(reader);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        self.buffers.push(buffer);
    }

    fn save_buffer(&self, buffer: &Buffer) {
        match File::create(buffer.name.as_path()) {
            Ok(file) => {
                let mut writer = LineWriter::new(file);
                for line in buffer.lines.iter() {
                    writer.write(line.as_ref()).unwrap();
                    writer.write("\n".as_ref()).unwrap();
                }
                // status.push_str(&"file written");
            }
            Err(_) => {
                // status.push_str(e.description());
            }
        }
    }

    fn active(&self) -> Option<&Buffer> {
        self.buffers.get(self.buf_idx)
    }

    fn active_mut(&mut self) -> Option<&mut Buffer> {
        self.buffers.get_mut(self.buf_idx)
    }

    fn resize(&mut self, w: usize, h: usize) {
        self.width = w;
        self.height = h;
        for buffer in self.buffers.iter_mut() {
            buffer.resize(w, h-2);
        }
    }

    pub fn get_motion(&self, key: Key) -> Option<Command> {
        match key {
            Key::Ctrl('h') | Key::Char('h') | Key::Left  => { 
                Some(goto(Span::Exclusive, Column::Left(1), Line::Current))
            }
            Key::Ctrl('j') | Key::Char('j') | Key::Down  => { 
                Some(goto(Span::Linewise, Column::Current, Line::Down(1)))
            }
            Key::Ctrl('p') | Key::Char('k') | Key::Up  => { 
                Some(goto(Span::Linewise, Column::Current, Line::Up(1)))
            }
            Key::Char(' ') | Key::Char('l') | Key::Right  => { 
                Some(goto(Span::Exclusive, Column::Right(1), Line::Current))
            }
            Key::Char('0')  => {
                Some(goto(Span::Exclusive, Column::Specific(0), Line::Current))
            }
            Key::Char('^')  => {
                Some(goto(Span::Exclusive, Column::Begin, Line::Current))
            }
            Key::Char('$')  => { 
                Some(goto(Span::Exclusive, Column::End, Line::Current))
            }
            Key::Char('G')  => { 
                Some(goto(Span::Linewise, Column::Current, Line::Last))
            }
            Key::Char('+') | Key::Char('m') => { 
                Some(goto(Span::Exclusive, Column::Begin, Line::Down(1)))
            }
            Key::Char('-') => { 
                Some(goto(Span::Exclusive, Column::Begin, Line::Up(1)))
            }
            Key::Ctrl('b') => {
                // what an awful way to do this.
                let lines = self.active().unwrap().window().1;
                let count = 1; // TODO: actually use [count]
                let relative = (count * (lines - 2));
                Some(scroll(Line::Up(relative)))
            }
            Key::Ctrl('f') => {
                // what an awful way to do this.
                let lines = self.active().unwrap().window().1;
                let count = 1; // TODO: actually use [count]
                let relative = (count * (lines - 2));
                Some(scroll(Line::Down(relative)))
            }
            Key::Ctrl('d') => {
                // what an awful way to do this.
                let lines = self.active().unwrap().window().1;
                let count = 1; // TODO: actually use [count]
                let relative = (count * (lines / 2));
                Some(scroll(Line::Down(relative)))
            }
            Key::Ctrl('u') => {
                // what an awful way to do this.
                let lines = self.active().unwrap().window().1;
                let count = 1; // TODO: actually use [count]
                let relative = (count * (lines / 2));
                Some(scroll(Line::Up(relative)))
            }
            Key::Ctrl('e') => {
                Some(scroll(Line::Down(1)))
            }
            Key::Ctrl('y') => {
                Some(scroll(Line::Up(1)))
            }
            _ => None
        }
    }

    fn do_cmd_key(&mut self, key: Key) {
        match key {
            Key::Char(':') => {
                self.mode = Mode::Colon;
            }
            Key::Char('i') => {
                self.mode = Mode::Insert;
            }
            Key::Char('a') => {
                self.active_mut().unwrap().right(1);
                self.mode = Mode::Insert;
            }
            Key::Char('O') => {
                self.mode = Mode::Insert;
                let mut active = self.active_mut().unwrap();
                active.end();
                active.newline();
            }
            Key::Char('o') => {
                self.mode = Mode::Insert;
                let mut active = self.active_mut().unwrap();
                active.end();
                active.newline();
            }
            _ => match self.get_motion(key) {
                Some(cmd) => {
                    self.active_mut().unwrap().do_cmd(1, &cmd);
                }
                None => {}
            }
        }
    }

    fn do_colon_key(&mut self, key: Key) {
        match key {
            Key::Char(c) => {
                self.colon.push(c);
            }
            Key::Esc => {
                self.colon.clear();
                self.mode = Mode::Normal;
            }
            Key::Enter => {
                // TODO: actually parse the line
                if self.colon == "q" {
                    self.buffers.remove(self.buf_idx);
                    if self.buf_idx >= self.buffers.len() {
                        self.buf_idx = 0;
                    }
                }
                self.colon.clear();
                self.mode = Mode::Normal;
            }
            _ => {}
        }
    }

    fn do_insert_key(&mut self, key: Key) {
        match key {
            Key::Esc => {
                self.mode = Mode::Normal;
            }
            Key::Enter => {
                self.active_mut().unwrap().newline();
            }
            Key::Char(c) => {
                self.active_mut().unwrap().insert(c);
            }
            _ => {}
        }
    }

    pub fn edit(&mut self, rustbox: &RustBox) {
        if self.buffers.is_empty() {
            self.buffers.push(Buffer::new_empty(self.width, self.height-2))
        }
        while self.buffers.len() > 0 {
            rustbox.draw(&self);
            match rustbox.poll_event(true) {
                Ok(Event::KeyEvent(key)) => {
                    match self.mode {
                        Mode::Normal => {
                            self.do_cmd_key(key)
                        }
                        Mode::Colon => {
                            self.do_colon_key(key)
                        }
                        Mode::Insert => {
                            self.status = format!("{:?}", key);
                            self.do_insert_key(key)
                        }
                    }
                },
                Ok(Event::ResizeEvent(w, h)) => {
                    self.resize(w as usize, h as usize);
                },
                Err(e) => panic!("{}", e),
                other => {
                    self.status = format!("{:?}", other);
                }
            }
        }
    }

    fn get_count(&mut self) -> usize {
        let cnt = match self.count.parse::<usize>() {
            Ok(c) => c,
            Err(_) =>   1
        };
        self.count.clear();
        cnt
    }
}

pub trait VexDisplay {
    fn draw(&self, state: &State);
}

impl VexDisplay for rustbox::RustBox {
    fn draw(&self, state: &State) {
        self.clear();
        self.present();

        let active = state.active().unwrap();
        let (x, y) = active.point();
        let (w, h) = active.window();

        let offset = active.offset();
        let other = String::from("~");

        for i in 0..h {
            let line = active.lines.get(i+offset).unwrap_or(&other);
            // let num = format!("{:2}", i+offset);
            let text = line.replace("\t", "    ");
            // self.print(0, i + 1, rustbox::RB_BOLD, Color::Default, Color::Default, &num);
            let formatted = format!("{: <1$}", text, w);
            self.print(0, i + 1, rustbox::RB_NORMAL, Color::Default, Color::Default, &formatted);
        }

        let mut idx = 0;
        for (i, buffer) in state.buffers.iter().enumerate() {
            let name = buffer.name();
            let (fg, bg) = if i == state.buf_idx {
                (Color::White, Color::Red)
            } else {
                (Color::Default, Color::Default)
            };
            self.print(idx, 0, rustbox::RB_NORMAL, fg, bg, &name);
            idx += 1 + name.len();
        }
    
        match state.mode {
            Mode::Colon => {
                let last_line = self.height() - 1;
                self.print(0, last_line, rustbox::RB_BOLD, Color::Default, Color::Default, ":");
                self.print(1, last_line, rustbox::RB_NORMAL, Color::Default, Color::Default, &state.colon);
                self.set_cursor(state.colon.len() as isize + 1, last_line as isize);
            }
            _ =>  {
                let len = active.lines.len();
                let status_line = format!("{} {}L {}", active.name(), len, state.status);
                self.print(0, self.height() - 1, rustbox::RB_NORMAL, Color::Default, Color::Default, &status_line);

                let x_ = active.lines.get(y).unwrap().len();
                let x__ = cmp::min(x_, x);
                self.set_cursor(x__ as isize, (y - offset) as isize + 1);
            }
        }

        self.present();
    }
}
