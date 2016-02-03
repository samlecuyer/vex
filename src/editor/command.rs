use editor::keyboard::{Key};

// A motion can vary a column in the following ways:
// * not varying it
// * changing it relatively
// * going to a specific column
// * going to a conceptual column (eol, first-non-blank)
#[derive(Debug)] 
pub enum Column {
    Current,
    Left(usize), Right(usize),
    Specific(usize),
    // Begin is the first-non-blank. 0 should use specific
    Begin, End
}

enum SkipThing {
	Word, Sentence, Whitespace,
}

#[derive(Debug)] 
pub enum Line {
    Current,
    Up(usize), Down(usize),
    Specific(usize),
    Last
}

#[derive(Debug)]
pub enum Span {
    Inclusive,
    Exclusive,
    Linewise,
}

#[derive(Debug)] 
pub enum Motion {
    Goto(Column,Line),
    Scroll(Line),
}

pub enum Operator {
	Delete,
	Insert,
}

pub enum Action {
	Operator(Operator),
	Motion(Motion),
}

#[derive(Debug)]
pub struct Command {
	pub count: usize,
    pub span: Span,
    pub motion: Motion,
}

impl Command {
    pub fn goto(s: Span, c: Column, l: Line) -> Command {
	    Command{count: 1, span: s, motion: Motion::Goto(c, l) }
	}

	pub fn scroll(l: Line) -> Command {
	    Command{count: 1, span: Span::Linewise, motion: Motion::Scroll(l) }
	}
}

#[derive(Debug)]
enum BuilderResult {
    Invalid,
    Pending,
    Command(Command),
}

struct Builder {
	// TODO; what goes here
	count: Option<usize>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
        	count: None
        }
    }
    pub fn handle_key(&mut self, key: Key) -> BuilderResult {
        if let Key::Char(c) = key {
        	if c.is_digit(10) {
        		let c = c.to_digit(10).unwrap() as usize;
        		match self.count {
        			None => {
        				self.count = Some(c);
        			}
        			Some(n) => {
        				self.count = Some(n * 10 + c);
        			}
        		};
        		return BuilderResult::Pending;
        	}
        }
        
      	BuilderResult::Invalid
    }
}

#[test]
fn builder_enter_count() {
    let mut builder = Builder::new();
    let cmd = builder.handle_key(Key::Char('1'));

    match cmd {
        BuilderResult::Pending => assert!(true),
        _ => assert!(false, "should be pending"),
    }

    let cmd = builder.handle_key(Key::Char('2'));
    match cmd {
        BuilderResult::Pending => assert!(true),
        _ => assert!(false, "should be pending"),
    }
    assert_eq!(builder.count, Some(12));
}

