// extern crate vex;

use buffer::Buffer;
use driver::{Driver, EditorEvent, Key};

use std::path::Path;
use std::fs::File;

pub trait Render {
    fn render(&self, editor: &Editor);
}

pub struct Editor {
	height: usize,
	width: usize,
	pub bufs: Vec<Buffer>,
}

impl Editor {
    pub fn new(w: usize, h: usize) -> Editor {
    	Editor {
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

    pub fn edit<T>(&mut self, driver: &T) where T: Driver + Render {
    	loop {
    	    driver.render(&self);
    		match driver.poll_event() {
    			EditorEvent::KeyEvent(Key::Ctrl('c')) => {
    				// gotta have some way to exit
	    	    	return;
	    	    }
	    	    EditorEvent::KeyEvent(key) => {

	    	    }
	    	    EditorEvent::Resize(w, h) => {
	    	    	self.width = w;
	    	    	self.height = h;
	    	    }
	    	    EditorEvent::Unsupported => {

	    	    },
	    	}
    	}
    }
}

#[test]
fn new_editor() {
	let editor = Editor::new(80, 24);
}
