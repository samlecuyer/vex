extern crate gapbuffer;

use self::gapbuffer::{GapBuffer, Items};

use std::iter::{FromIterator, IntoIterator};

/// As described in
/// http://pubs.opengroup.org/onlinepubs/9699919799/utilities/ex.html

enum BufferMode {
    Character,
    Line,
}

struct Buffer {
	top: usize,
	cursor: usize,
	mode: BufferMode,
    buf: GapBuffer<u8>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
        	top: 0,
        	cursor: 0,
        	mode: BufferMode::Line,
        	buf: GapBuffer::new()
        }
    }
    pub fn len(&self) -> usize {
    	self.buf.len()
    }
}

impl FromIterator<u8> for Buffer {
	fn from_iter<T>(iterator: T) -> Self where T: IntoIterator<Item=u8> {
		let mut buf = GapBuffer::from_iter(iterator);
		Buffer {
			top: 0,
			cursor: 0,
			mode: BufferMode::Line,
        	buf: buf
		}
	}
}




#[test]
fn buffer_from_string() {
    let contents = String::from("hello world.\n\nthis is a new line");
    let buffer = Buffer::from_iter(contents.bytes());

    assert_eq!(contents.len(), buffer.len());
}