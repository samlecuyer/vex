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
    	match key {
    	    Key::Ctrl('c') => {
				self.bufs.remove(0);
		    }
            Key::Esc => {
                self.mode = Mode::Normal;
            }
            Key::Char('i') if self.mode == Mode::Normal => {
                self.mode = Mode::Insert;
            }
            Key::Char(c) if self.mode == Mode::Insert => {
                self.bufs.get_mut(0).unwrap().insert(c);
            }
            Key::Enter if self.mode == Mode::Insert => {
                self.bufs.get_mut(0).unwrap().insert('\n');
            }
		    Key::Char('j') => {
		    	self.top += 1;
		    }
		    Key::Char('k') => {
		    	self.top = self.top.saturating_sub(1);
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
