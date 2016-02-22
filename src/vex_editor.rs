// extern crate vex;

use buffer::Buffer;
use driver::{Driver, EditorEvent, Key};

use std::path::Path;
use std::fs::File;

pub trait Render {
    fn render(&self, editor: &Editor);
}

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    // Command,
    // Ex,
}

pub struct Editor {
    mode: Mode,
    height: usize,
    width: usize,
    pub top: usize,
    pub bufs: Vec<Buffer>,
}

impl Editor {
    pub fn new(w: usize, h: usize) -> Editor {
        Editor {
            mode: Mode::Normal,
            top: 0,
            width: w,
            height: h,
            bufs: Vec::new()
        }
    }

    pub fn open(&mut self, filename: &Path) {
        let mut buf = match File::open(filename) {
            Ok(file) => Buffer::from(file),
            Err(_) => Buffer::new()
        };
        self.bufs.push(buf);
    }

    pub fn open_empty(&mut self) {
        let mut buf = Buffer::new();
        self.bufs.push(buf);
    }

    pub fn edit<T>(&mut self, driver: &T) where T: Driver + Render {
        while self.is_editing() {
            driver.render(&self);
            match driver.poll_event() {
                EditorEvent::KeyEvent(key) => {
                    self.handle_key(key);
                }
                EditorEvent::Resize(w, h) => {
                    self.resize(w, h);
                }
                EditorEvent::Unsupported => {
                    // http://10x.engineer/
                },
            }
        }
    }

    fn handle_key(&mut self, key: Key) {
        let cmd = match self.mode {
            Mode::Insert => InsertMode::handle_key(key),
            Mode::Normal => NormalMode::handle_key(key),
        };
        match cmd {
            MultiOption::Some(cmd) => {
                self.do_cmd(cmd);
            }
            MultiOption::Pending => {
                // do nothing for now
            }
            MultiOption::None => {
                // maybe signal an error to the user
            }
        }
    }

    fn do_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Quit => {
                self.bufs.remove(0);
            }
            Command::Modal(mode) => {
                self.mode = mode;
            }
            Command::Insert(ch) => {
                self.bufs.get_mut(0).unwrap().insert(ch);
            }
            _ => {}
        }
    }

    fn is_editing(&self) -> bool {
        self.bufs.len() > 0
    }

    fn resize(&mut self, w: usize, h: usize) {
        self.width = w;
        self.height = h;
    }
}

#[test]
fn new_editor() {
    let editor = Editor::new(80, 24);

    assert_eq!(editor.width, 80);
    assert_eq!(editor.height, 24);
}

enum MultiOption<T> {
    None,
    Pending,
    Some(T),
}
enum Command {
    Insert(char),
    Modal(Mode),
    MoveCursor,
    Quit
}

trait HandleKey {
    fn handle_key(key: Key) -> MultiOption<Command>;
}

#[derive(Debug)]
struct InsertMode;
impl HandleKey for InsertMode {
    fn handle_key(key: Key) -> MultiOption<Command> {
        match key {
            Key::Char(c) => MultiOption::Some(Command::Insert(c)),
            Key::Enter =>   MultiOption::Some(Command::Insert('\n')),
            Key::Esc => MultiOption::Some(Command::Modal(Mode::Normal)),
            _ => MultiOption::None
        }
    }
}

#[test]
fn insert_mode_keys() {
    match InsertMode::handle_key(Key::Esc) {
        MultiOption::Some(Command::Modal(Mode::Normal)) => assert!(true, "should enter normal mode"),
        _ => assert!(false, "should enter normal mode"),
    }
}

#[derive(Debug)]
struct NormalMode;
impl HandleKey for NormalMode {
    fn handle_key(key: Key) -> MultiOption<Command> {
        match key {
            Key::Ctrl('c') => MultiOption::Some(Command::Quit),
            Key::Char('i') => MultiOption::Some(Command::Modal(Mode::Insert)),
            Key::Char('h') => MultiOption::Some(Command::MoveCursor),
            Key::Char('j') => MultiOption::Some(Command::MoveCursor),
            Key::Char('k') => MultiOption::Some(Command::MoveCursor),
            Key::Char('l') => MultiOption::Some(Command::MoveCursor),
            _ => MultiOption::None
        }
    }
}

#[test]
fn normal_mode_keys() {
    match NormalMode::handle_key(Key::Char('i')) {
        MultiOption::Some(Command::Modal(Mode::Insert)) => assert!(true, "should enter insert mode"),
        _ => assert!(false, "should enter insert mode"),
    }
}

