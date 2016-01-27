extern crate unicode_segmentation;

use std::collections::VecDeque;
use std::iter::FromIterator;

use std::io::{Result};
use std::io::prelude::*;

use std::path::{Path, PathBuf};

use super::{Coord, Direction, imin, imax};

use self::unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Buffer {
	pub name: PathBuf,
	point: Coord,
	mark: Option<Coord>,
	window: (usize, usize),
	offset: usize,
	pub lines: VecDeque<String>,
}

impl Buffer {
	pub fn new(name: &Path, width: usize, height: usize) -> Buffer {
		Buffer {
			name: name.to_path_buf(),
			point: (0, 0),
			mark: None,
	    	window: (width, height),
	    	offset: 0,
	    	lines: VecDeque::new(),
		}
	}
	pub fn new_empty(width: usize, height: usize) -> Buffer {
		let mut lines = VecDeque::with_capacity(height);
		lines.push_front(" \n".to_owned());
		Buffer {
			name: PathBuf::from("[untitled]"),
			point: (0, 0),
			mark: None,
	    	window: (width, height),
	    	offset: 0,
	    	lines: lines,
		}
	}
	pub fn load_reader<B>(&mut self, reader: B) where B: BufRead {
		let lines_buf = Lines{buf: reader};
	    let lines = lines_buf.filter_map(|l| l.ok());
		self.lines = VecDeque::from_iter(lines);
	}
	pub fn name(&self) -> &str {
	    self.name.file_name().unwrap().to_str().unwrap()
	}
	pub fn point(&self) -> Coord {
	    self.point
	}
	pub fn window(&self) -> (usize, usize) {
	    self.window
	}
	pub fn offset(&self) -> usize {
	    self.offset
	}
    pub fn delete_line(&mut self, i: isize) {
        self.lines.remove(i as usize);
        let numlines = self.lines.len() as isize - 1;
        if self.point.1 > numlines {
        	self.point.1 = numlines;
        }
        if self.point.1 < self.offset as isize {
        	self.offset = self.point.1 as usize;
        }
    }
    pub fn insert(&mut self, ch: char) {
        let col  = self.point.0 as usize;
        let line = self.point.1 as usize;
        self.lines.get_mut(line).unwrap().insert(col, ch);
        self.point.0 += 1;
        if ch == '\n' {
        	let text = self.lines.get(line).unwrap().to_owned();
        	let (_, b) = text.split_at(col+1);
        	self.lines.get_mut(line).unwrap().truncate(col+1);
        	self.lines.insert(line + 1, b.to_owned());
        	self.point.1 += 1;
        	self.point.0 = 0;
        }
    }

    // Navigation
    pub fn left(&mut self, c: usize) {
		let next = self.point.0 - c as isize;
		self.point.0 = imax(0, next);
    }
    pub fn right(&mut self, c: usize) {
    	let line : &str = &self.lines.get(self.point.1 as usize).unwrap();
    	let graphemes = UnicodeSegmentation::graphemes(line, true);
    	let width = graphemes.count() - 1;
        let next = self.point.0 + c as isize;
		self.point.0 = imin(width as isize, next);
    }
    // the rules are that offset can't be negative
    // point.1 can never be less than offset
    // point.1 can never be more than offset + heights

    pub fn begin(&mut self) {
    	let line = self.lines.get(self.point.1 as usize).unwrap();
		let idx = line.find(|c: char| !c.is_whitespace()).unwrap_or(line.len() - 1);
		self.point.0 = idx as isize;
    }
    pub fn end(&mut self) {
    	let line : &str = &self.lines.get(self.point.1 as usize).unwrap();
    	let graphemes = UnicodeSegmentation::graphemes(line, true);
    	let width = graphemes.count() - 1;
		self.point.0 = width as isize;
    }

    pub fn scroll(&mut self, c: usize, dir: Direction) {
    	self.offset = match dir {
    	    Direction::Up => {
    	    	let next = self.offset as isize - c as isize;
				imax(0, next) as usize
    	    },
    	    Direction::Down => {
    	    	let next = self.offset as isize + c as isize;
				let len = self.lines.len() as isize - 1;
				imin(len, next) as usize
    	    },
    	};
    	if self.point.1 < self.offset as isize {
    		self.point.1 = self.offset as isize;
    	}
    	let end = self.last_line();
    	if self.point.1 > end {
    		self.point.1 = end;
    	}
		self.point.0 = 0;
    }

    pub fn page_back(&mut self, c: usize) {
		let cnt = (c as isize* (self.window.1 as isize - 2)) - 1;
		self.scroll(cnt as usize, Direction::Up);
    }

    pub fn page_fwd(&mut self, c: usize) {
		let cnt = (c as isize * (self.window.1 as isize - 2)) - 1;
		self.scroll(cnt as usize, Direction::Down);
    }

    pub fn prev(&mut self, c: usize) {
		let next = self.point.1 - c as isize;
		self.point.1 = imax(0, next);
		if self.point.1 < self.offset as isize {
			self.offset -= 1;
		}
    }
    pub fn next(&mut self, c: usize) {
        let next = self.point.1 + c as isize;
        let height = self.window.1 as isize;
    	let len = self.lines.len() as isize - 1;
		self.point.1 = imin(len, next);
		if self.point.1 > self.offset as isize + height - 1 {
			self.offset += 1;
		}	
    }
    pub fn resize(&mut self, w: usize, h: usize) {
        self.window = (w, h);
    }
    fn last_line(&self) -> isize {
    	let endline = self.lines.len() - self.offset;
    	self.offset as isize + imin(self.window.1 as isize - 1, endline as isize)
    }
}

struct Lines<B> {
	buf: B
}

impl<B: BufRead> Iterator for Lines<B> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e))
        }
    }
}

#[test]
fn new_buffer() {
	let buf = Buffer::new_empty(80, 24);
	assert_eq!(buf.point, (0, 0));
}

#[test]
fn basic_navigation() {
	let mut buf = Buffer::new_empty(80, 24);
	assert_eq!(buf.point, (0, 0));

	buf.next(1);
	assert_eq!(buf.point, (0, 0));
}

