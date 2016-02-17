extern crate gapbuffer;

use std::io::{Result, Read, BufReader, BufRead};
use self::gapbuffer::{GapBuffer, Items};

use std::cmp::{Ordering, min};
use std::iter::{FromIterator, IntoIterator};
use std::convert::From;

/// As described in
/// http://pubs.opengroup.org/onlinepubs/9699919799/utilities/ex.html

enum BufferMode {
    Character,
    Line,
}

pub struct Buffer {
    cursor: usize,
    mode: BufferMode,
    buf: GapBuffer<u8>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            cursor: 0,
            mode: BufferMode::Line,
            buf: GapBuffer::new()
        }
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn iter(&self) -> Items<u8> {
        self.buf.iter()
    }

    pub fn as_read<'a>(&'a self) -> Bytes {
        Bytes {idx: 0, buf: &self.buf}
    }

    pub fn as_read_from<'a>(&'a self, from: usize) -> Bytes {
        Bytes {idx: from, buf: &self.buf}
    }

    pub fn insert(&mut self, c: char) {
        // TODO: use encode_utf8 when that's stable
        for b in  c.to_string().as_bytes()  {
            self.buf.insert(self.cursor, *b);
            self.cursor += 1;
        }
    }
}

// impl<P: AsRef<Path>> From<P> for Buffer {
//     fn from(path: P) -> Buffer {
//         match File::open(path) {
//             Ok(file) => Buffer::from(file),
//             Err(_) => Buffer::new()
//         }
//     }
// }

impl<R: Read> From<R> for Buffer {
    fn from(mut reader: R) -> Buffer {
        let mut b = reader.bytes().flat_map(|c| c);
        Buffer::from_iter(b)
    }
}

impl FromIterator<u8> for Buffer {
    fn from_iter<T>(iterator: T) -> Self where T: IntoIterator<Item=u8> {
        let mut buf = GapBuffer::from_iter(iterator);
        Buffer {
            cursor: 0,
            mode: BufferMode::Line,
            buf: buf
        }
    }
}

// TODO: support BufRead rather than use a BufReader
pub struct Bytes<'a> {
    idx: usize,
    buf: &'a GapBuffer<u8>,
}

impl<'a> Read for Bytes<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let expected = buf.len();
        let size = self.buf.len();
        let upper = min(size, expected + self.idx);
        let mut nread = 0;
        for i in self.idx..upper {
            buf[i - self.idx] = *self.buf.get(i).unwrap();
            nread += 1;
        }
        self.idx += nread;
        Ok(nread)
    }
}

#[test]
fn buffer_from_string_iter() {
    let input = String::from("hello world.\n\nthis is a new line");
    let buffer = Buffer::from_iter(input.bytes());

    assert_eq!(32, buffer.len());

    let contents : Vec<u8> = buffer.iter().map(|&c| c).collect();
    let output = String::from_utf8(contents).unwrap();

    assert_eq!(Ordering::Equal, String::cmp(&input, &output));
}

#[test]
fn buffer_as_reader() {
    let input = String::from("hello world.\n\nthis is a new line");
    let buffer = Buffer::from_iter(input.bytes());

    let r = buffer.as_read();
    let br = BufReader::new(r);

    let strings : Vec<String> = br.lines().flat_map(|c| c).collect();
    for line in strings {
        println!("{:?}", line);
    }

    let r = buffer.as_read_from(14);
    let br = BufReader::new(r);

    let strings : Vec<String> = br.lines().flat_map(|c| c).collect();
    assert_eq!(strings.get(0).unwrap(), "this is a new line");
}

#[test]
fn buffer_insert_chars() {
    let input = String::from("hello world.\n\nthis is a new line");
    let mut buffer = Buffer::from_iter(input.bytes());

    assert_eq!(32, buffer.len());
    buffer.insert('東');
    buffer.insert(' ');
    assert_eq!(36, buffer.len());

    let r = buffer.as_read();
    let br = BufReader::new(r);

    let strings : Vec<String> = br.lines().flat_map(|c| c).collect();

    assert_eq!(strings.get(0).unwrap(), "東 hello world.");
}

