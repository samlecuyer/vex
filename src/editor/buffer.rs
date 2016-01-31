extern crate unicode_segmentation;

use std::collections::VecDeque;
use std::iter::FromIterator;

use std::io::{Result};
use std::io::prelude::*;

use std::path::{Path, PathBuf};

use super::{Coord, Direction, Range, Yank};

use editor::command::{Command, Span, Motion, Line, Column, goto};
use std::cmp;

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
		lines.push_front("".to_owned());
		Buffer {
			name: PathBuf::from("untitled"),
			point: (0, 0),
			mark: None,
	    	window: (width, height),
	    	offset: 0,
	    	lines: lines,
		}
	}
	pub fn load_reader<B>(&mut self, reader: B) where B: BufRead {
	    let lines = reader.lines().filter_map(|l| l.ok());
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
    pub fn delete_line(&mut self, i: usize) {
        self.lines.remove(i);
        let numlines = self.lines.len() - 1;
        if self.point.1 > numlines {
        	self.point.1 = numlines;
        }
        if self.point.1 < self.offset {
        	self.offset = self.point.1;
        }
    }
    pub fn insert(&mut self, ch: char) {
        let (col, line)  = self.point;
        self.lines.get_mut(line).unwrap().insert(col, ch);
        self.point.0 += 1;
    }
    pub fn newline(&mut self) {
        let (col, line)  = self.point;
        let text = self.lines.get(line).unwrap().to_owned();
        let (_, b) = text.split_at(col);
        self.lines.get_mut(line).unwrap().truncate(col);
        self.lines.insert(line + 1, b.to_owned());
        self.point.1 += 1;
        self.point.0 = 0;
    }

    pub fn yank_del(&mut self, r: Range, mode: Yank) {
        
    }

    // Navigation
    pub fn left(&mut self, c: usize) {
		let curr = self.point.0;
		self.point.0 = curr.saturating_sub(c);
    }
    pub fn right(&mut self, c: usize) {
    	let line : &str = &self.lines.get(self.point.1).unwrap();
    	let graphemes = UnicodeSegmentation::graphemes(line, true);
    	let width = graphemes.count() - 1;
        let next = self.point.0 + c;
		self.point.0 = cmp::min(width, next);
    }
    // the rules are that offset can't be negative
    // point.1 can never be less than offset
    // point.1 can never be more than offset + heights

    pub fn begin(&mut self) {
    	let line = self.lines.get(self.point.1).unwrap();
		let idx = line.find(|c: char| !c.is_whitespace()).unwrap_or(0);
		self.point.0 = idx ;
    }
    pub fn end(&mut self) {
    	let line : &str = &self.lines.get(self.point.1).unwrap();
    	let graphemes = UnicodeSegmentation::graphemes(line, true);
    	let width = graphemes.count() - 1;
		self.point.0 = width;
    }

    pub fn scroll(&mut self, c: usize, dir: Direction) {
    	self.offset = match dir {
    	    Direction::Up => {
                self.offset.saturating_sub(c)
    	    },
    	    Direction::Down => {
    	    	let next = self.offset + c;
				let len = self.lines.len() - 1;
				cmp::min(len, next)
    	    },
    	};
    	if self.point.1 < self.offset {
    		self.point.1 = self.offset;
    	}
    	let end = self.last_line();
    	if self.point.1 > end {
    		self.point.1 = end;
    	}
		self.point.0 = 0;
    }

    pub fn page_back(&mut self, c: usize) {
		let cnt = (c * (self.window.1 - 2)) - 1;
		self.scroll(cnt, Direction::Up);
    }

    pub fn page_fwd(&mut self, c: usize) {
		let cnt = (c * (self.window.1 - 2)) - 1;
		self.scroll(cnt, Direction::Down);
    }

    pub fn prev(&mut self, c: usize) {
		let curr = self.point.1;
		self.point.1 = curr.saturating_sub(c);
    }
    pub fn next(&mut self, c: usize) {
        let height = self.window.1;
    	let len = self.lines.len() - 1;
        let next = self.point.1 + c;
		self.point.1 = cmp::min(len, next);
		if self.point.1 > self.offset + height - 1 {
			self.offset += 1;
		}	
    }
    pub fn resize(&mut self, w: usize, h: usize) {
        self.window = (w, h);
        self.window_to_point();
    }

    fn last_line(&self) -> usize {
    	let endline = self.lines.len() - self.offset;
    	self.offset + cmp::min(self.window.1 - 1, endline)
    }

    fn window_to_point(&mut self) {
        let l = self.point.1;
        let h = self.window.1 - 1;
        let offset = self.offset;
        if l < offset {
            // move the screen up to the point
            self.offset = l;
        } else if l > offset + h {
            self.offset = l - h; 
        }
    }
    fn point_to_window(&mut self) {
        let l = self.point.1;
        let (_, h) = self.window;
        let offset = self.offset;
        if l < offset {
            // move the point to the first line
            self.point.1 = offset;
            self.begin();
        } else if l >= offset + h {
            // move the point to the last line
            self.point.1 = self.last_line();
            self.begin();
        }
    }

    pub fn do_cmd(&mut self, count: usize, cmd: &Command) {
        let (c, l) = self.point;
        match cmd.motion {
            Motion::Goto(ref col, ref line) => {
                match *line {
                    Line::Current => { /* do nothing */ }
                    Line::Up(i) => {
                        let line = self.point.1.saturating_sub(i);
                        self.point.1 = line;
                    }
                    Line::Down(i) => {
                        let len = self.lines.len() - 1;
                        let line = self.point.1 + i;
                        self.point.1 = cmp::min(len, line);
                    }
                    Line::Specific(i) => {
                        let len = self.lines.len() - 1;
                        if i < len {
                            self.point.1 = i;
                        }
                    }
                    Line::Last => {
                        let len = self.lines.len() - 1;
                        self.point.1 = len;
                    }
                };
                match *col {
                    Column::Current => { /* do nothing */ }
                    Column::Specific(i) => {
                        let line = self.lines.get(l).unwrap();
                        if i < line.len() {
                            self.point.0 = i;
                        }
                    }
                    Column::Left(i) => {
                        let col = self.point.0.saturating_sub(i);
                        self.point.0 = col;
                    }
                    Column::Right(i) => {
                        let line = self.lines.get(l).unwrap();
                        let len = line.len();
                        let col = self.point.0 + i;
                        self.point.0 = cmp::min(len, col);
                    }
                    Column::Begin => {
                        let line = self.lines.get(self.point.1).unwrap();
                        let idx = line.find(|c: char| !c.is_whitespace()).unwrap_or(0);
                        self.point.0 = idx ;
                    }
                    Column::End => {
                        let line = self.lines.get(l).unwrap();
                        self.point.0 = line.len() - 1;
                    }
                }
                // println!("{:?} {:?}", col, line);
                self.window_to_point();
            }
            Motion::Scroll(ref line) => {
                match *line {
                    Line::Current => { /* do nothing */ }
                    Line::Down(i) => {
                        let len = self.lines.len() - 1;
                        let down = self.offset + i;
                        self.offset = cmp::min(len, down);
                    }
                    Line::Up(i) => {
                        let line = self.offset.saturating_sub(i);
                        self.offset = line;
                    }
                    _ => unreachable!()
                }
                self.point_to_window();
            }
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

#[test]
fn basic_column_nav() {
    let mut buf = Buffer::new_empty(80, 24);
    buf.lines.get_mut(0).unwrap().push_str(" hello world");

    let fwd   = goto(Span::Exclusive, Column::Right(1), Line::Current);
    let back  = goto(Span::Exclusive, Column::Left(1), Line::Current);
    let begin = goto(Span::Exclusive, Column::Begin, Line::Current);
    let end   = goto(Span::Exclusive, Column::End, Line::Current);
    let zero  = goto(Span::Exclusive, Column::Specific(0), Line::Current);

    buf.do_cmd(1, &fwd);
    assert_eq!(buf.point, (1, 0));

    buf.do_cmd(1, &end);
    assert_eq!(buf.point, (11, 0));

    buf.do_cmd(1, &back);
    assert_eq!(buf.point, (10, 0));

    for _ in 1..5 {
        buf.do_cmd(1, &fwd);
    }
    assert_eq!(buf.point, (12, 0));

    buf.do_cmd(1, &begin);
    assert_eq!(buf.point, (1, 0));

    buf.do_cmd(1, &zero);
    assert_eq!(buf.point, (0, 0));
}

#[test]
fn basic_line_nav() {
    let mut buf = Buffer::new_empty(80, 24);
    for _ in 1..10 {
        let line = String::from(" hello world this is a line");
        buf.lines.push_back(line);
    }
    // assert!(false, "should write tests for this");
    let down   = goto(Span::Linewise, Column::Current, Line::Down(1));
    let up     = goto(Span::Linewise, Column::Current, Line::Up(1));
    let first  = goto(Span::Linewise, Column::Current, Line::Specific(0));
    let fourth = goto(Span::Linewise, Column::Current, Line::Specific(3));
    let last   = goto(Span::Linewise, Column::Current, Line::Last);

    buf.do_cmd(1, &down);
    assert_eq!(buf.point, (0, 1));

    buf.do_cmd(1, &fourth);
    assert_eq!(buf.point, (0, 3));

    buf.do_cmd(1, &up);
    assert_eq!(buf.point, (0, 2));

    for _ in 1..10 {
        buf.do_cmd(1, &down);
    }
    assert_eq!(buf.point, (0, 9));

    buf.do_cmd(1, &first);
    assert_eq!(buf.point, (0, 0));

    buf.do_cmd(1, &last);
    assert_eq!(buf.point, (0, 9));
}

#[test]
fn basic_scroll_nav() {
    let mut buf = Buffer::new_empty(80, 24);
    for _ in 1..10 {
        buf.lines.get_mut(0).unwrap().push_str(" hello world this is a line");
    }
    assert!(false, "should write tests for this");
}
