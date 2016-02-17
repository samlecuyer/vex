extern crate rustbox;

use self::rustbox::{Color, RustBox, Event};

pub use keyboard::Key;

pub enum EditorEvent {
    KeyEvent(Key),
    Resize(usize, usize),
    Unsupported
}

pub	trait Driver {
    fn poll_event(&self) -> EditorEvent;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

impl Driver for rustbox::RustBox {
    fn poll_event(&self) -> EditorEvent {
        match self.poll_event(false) {
        	Ok(Event::KeyEvent(k)) => EditorEvent::KeyEvent(match k {
        		rustbox::Key::Tab => Key::Tab,
        		rustbox::Key::Enter => Key::Enter,
        		rustbox::Key::Esc => Key::Esc,	
				rustbox::Key::Backspace => Key::Backspace,
				rustbox::Key::Right => Key::Right,
				rustbox::Key::Left => Key::Left,
				rustbox::Key::Up => Key::Up,
				rustbox::Key::Down => Key::Down,
				rustbox::Key::Delete => Key::Delete,
				rustbox::Key::Insert => Key::Insert,

				rustbox::Key::Home => Key::Home,
				rustbox::Key::End => Key::End,
				rustbox::Key::PageUp => Key::PageUp,
				rustbox::Key::PageDown => Key::PageDown,

				rustbox::Key::Char(c) => Key::Char(c),
				rustbox::Key::Ctrl(c) => Key::Ctrl(c),
				rustbox::Key::F(i)  => Key::F(i),
				rustbox::Key::Unknown(u) => Key::Unknown(u),
        	}),
        	Ok(Event::ResizeEvent(x, y)) => EditorEvent::Resize(x as usize, y as usize),
        	_ => EditorEvent::Unsupported
        }
    }
    fn height(&self) -> usize {
        self.height()
    }
    fn width(&self) -> usize {
        self.width()
    }
}