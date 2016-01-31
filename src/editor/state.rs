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
    Command,
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
            mode: Mode::Command,
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

    pub fn get_motion(&self, key: Key) -> Command {
        match self.mode {
            Mode::Command => {
                match key {
                    // navigation
                    // TODO: [count]
                    Key::Ctrl('h') | Key::Char('h') | Key::Left  => { 
                        goto(Span::Exclusive, Column::Left(1), Line::Current)
                    }
                    Key::Ctrl('j') | Key::Char('j') | Key::Down  => { 
                        goto(Span::Linewise, Column::Current, Line::Down(1))
                    }
                    Key::Ctrl('p') | Key::Char('k') | Key::Up  => { 
                        goto(Span::Linewise, Column::Current, Line::Up(1))
                    }
                    Key::Char(' ') | Key::Char('l') | Key::Right  => { 
                        goto(Span::Exclusive, Column::Right(1), Line::Current)
                    }
                    Key::Char('0')  => {
                        goto(Span::Exclusive, Column::Specific(0), Line::Current)
                    }
                    Key::Char('^')  => {
                        goto(Span::Exclusive, Column::Begin, Line::Current)
                    }
                    Key::Char('$')  => { 
                        goto(Span::Exclusive, Column::End, Line::Current)
                    }
                    Key::Char('G')  => { 
                        goto(Span::Linewise, Column::Current, Line::Last)
                    }
                    Key::Char('+') | Key::Char('m') => { 
                        goto(Span::Exclusive, Column::Begin, Line::Down(1))
                    }
                    Key::Char('-') => { 
                        goto(Span::Exclusive, Column::Begin, Line::Up(1))
                    }
                    Key::Ctrl('b') => {
                        // what an awful way to do this.
                        let lines = self.active().unwrap().window().1;
                        let count = 1; // TODO: actually use [count]
                        let relative = (count * (lines - 2)) - 1;
                        scroll(Line::Up(relative))
                    }
                    Key::Ctrl('f') => {
                        // what an awful way to do this.
                        let lines = self.active().unwrap().window().1;
                        let count = 1; // TODO: actually use [count]
                        let relative = (count * (lines - 2)) - 1;
                        scroll(Line::Down(relative))
                    }
                    Key::Ctrl('d') => {
                        // what an awful way to do this.
                        let lines = self.active().unwrap().window().1;
                        let count = 1; // TODO: actually use [count]
                        let relative = (count * (lines / 2)) - 1;
                        scroll(Line::Down(relative))
                    }
                    Key::Ctrl('u') => {
                        // what an awful way to do this.
                        let lines = self.active().unwrap().window().1;
                        let count = 1; // TODO: actually use [count]
                        let relative = (count * (lines / 2)) - 1;
                        scroll(Line::Up(relative))
                    }
                    Key::Ctrl('e') => {
                        scroll(Line::Down(1))
                    }
                    Key::Ctrl('y') => {
                        scroll(Line::Up(1))
                    }
                    _ => panic!("unimplemented")
                }
            }
            Mode::Insert => {
                unreachable!()
            }
        }
    }

    pub fn handle_key(&mut self, key: Key) {
        match self.mode {
            Mode::Command => {
                match key {
                    // TODO: we should have a separate `ex` parsing function
                    Key::Enter if !self.colon.is_empty() => {
                        match self.colon.as_ref() {
                            ":w" | ":write" => {
                                let buffer = self.active().unwrap();
                                self.save_buffer(buffer);
                            }
                            ":q" | ":quit" => {
                                // TODO: there should be a callback
                                // so the editor can clean up
                                self.buffers.remove(self.buf_idx);
                                if self.buf_idx >= self.buffers.len() {
                                    self.buf_idx = 0;
                                }
                            }
                            _ => { }
                        }
                        self.colon.clear();
                    }
                    Key::Char(c) if !self.colon.is_empty() => {
                        self.colon.push(c);
                    }
                    Key::Char(':') => {
                        self.colon.push(':');
                    }
                    Key::Char('0') if self.count.is_empty() => {
                        
                    }
                    Key::Char(c) if c.is_digit(10) => {
                        self.count.push(c);
                    }
                    Key::Ctrl('b') => { 
                        let cnt = self.get_count();
                        self.active_mut().unwrap().page_back(cnt);
                    }
                    Key::Ctrl('d') => {
                        let cnt = self.get_count();
                        self.active_mut().unwrap().scroll(cnt, Direction::Down);
                    }
                    Key::Ctrl('e') => { 
                        // let cnt = self.get_count();
                        self.active_mut().unwrap().scroll(1, Direction::Down);
                    }
                    Key::Ctrl('f') => {
                        let cnt = self.get_count();
                        self.active_mut().unwrap().page_fwd(cnt); 
                    }
                    Key::Ctrl('m') | Key::Char('+') => { 
                        // let cnt = self.get_count();
                        let active = self.active_mut().unwrap();
                        active.next(1);
                        active.begin();
                    }
                    Key::Ctrl('n') => {
                        let len = self.buffers.len();
                        self.buf_idx = (self.buf_idx + 1) % len;
                    }
                    Key::Ctrl('u') => {
                        let cnt = self.get_count();
                        self.active_mut().unwrap().scroll(cnt, Direction::Up); 
                    }
                    Key::Ctrl('y') => { 
                        // let cnt = self.get_count();
                        self.active_mut().unwrap().scroll(1, Direction::Up);
                    }
                    Key::Esc => {
                        self.colon.clear(); 
                        self.status.clear();
                    }
                    Key::Char('^') => {
                        self.active_mut().unwrap().begin(); 
                    }
                    Key::Char('$') => {
                        let cnt = self.get_count();
                        self.active_mut().unwrap().next(cnt - 1);
                        self.active_mut().unwrap().end(); 
                    }
                    Key::Char('i') => {
                        self.mode = Mode::Insert;
                    }
                    Key::Char('d') => {
                        let mut buf = self.active_mut().unwrap();
                        let (_, l) = buf.point();
                        buf.delete_line(l);
                    }
                    Key::Char('D') => {
                        
                    }
                    Key::Ctrl('h') | Key::Char('h') | Key::Left  => { self.active_mut().unwrap().left(1); }
                    Key::Ctrl('j') | Key::Char('j') | Key::Down  => { self.active_mut().unwrap().next(1); }
                    Key::Ctrl('p') | Key::Char('k') | Key::Up => { self.active_mut().unwrap().prev(1); }
                    Key::Char(' ') | Key::Char('l') | Key::Right => { self.active_mut().unwrap().right(1); }
                    _ => { }
                };
            },
            Mode::Insert => {
                match key {
                    Key::Esc => {
                        self.mode = Mode::Command;
                    }
                    Key::Enter => {
                        let mut buf = self.active_mut().unwrap();
                        buf.newline();
                    }
                    Key::Char(k) => {
                        let mut buf = self.active_mut().unwrap();
                        buf.insert(k);
                    }
                    Key::Ctrl('h') | Key::Left  => { self.active_mut().unwrap().left(1); }
                    Key::Down  => { self.active_mut().unwrap().next(1); }
                    Key::Up    => { self.active_mut().unwrap().prev(1); }
                    Key::Right => { self.active_mut().unwrap().right(1); }
                    _ => { }
                };
            }
        }
    }

    pub fn edit(&mut self, rustbox: &RustBox) {
        if self.buffers.is_empty() {
            self.buffers.push(Buffer::new_empty(self.width, self.height-2))
        }
        while self.buffers.len() > 0 {
            rustbox.draw(&self);
            match rustbox.poll_event(false) {
                Ok(Event::KeyEvent(key)) => {
                    // self.handle_key(key)
                    let cmd = self.get_motion(key);
                    self.active_mut().unwrap().do_cmd(1, &cmd);
                },
                Ok(Event::ResizeEvent(w, h)) => {
                    self.resize(w as usize, h as usize);
                },
                Err(e) => panic!("{}", e),
                _ => { }
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
    
        if state.colon.is_empty() {
            let len = active.lines.len();
            let status_line = format!("{} {}L", active.name(), len);
            self.print(0, self.height() - 1, rustbox::RB_NORMAL, Color::Default, Color::Default, &status_line);
        } else {
            self.print(0, self.height() - 1, rustbox::RB_NORMAL, Color::Default, Color::Default, &state.colon);
        }
        let x_ = active.lines.get(y).unwrap().len();
        let x__ = cmp::min(x_.saturating_sub(1), x);
        self.set_cursor(x__ as isize, (y - offset) as isize + 1);

        self.present();
    }
}
