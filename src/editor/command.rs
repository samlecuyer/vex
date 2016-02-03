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

enum TextObject {
	Char, Word, Sentence, Line, Paragraph,
}
enum Partial {
	Object(TextObject),
	Action(Action),
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

        match self.lookup_key(key) {
            Some(partial) => {
            	let cmd = self.build_cmd(partial);
            	return BuilderResult::Command(cmd);
            }
            None => {
            	return BuilderResult::Invalid;
            }
        }
      	BuilderResult::Invalid
    }

    fn build_cmd(&mut self, partial: Partial) -> Command {
        Command {
        	count: self.count.unwrap_or(1),
        	span: Span::Linewise,
        	motion: match partial {
        		Partial::Action(action) => match action {
        		    Action::Motion(motion) => motion,
        		    _ => panic!("{:?}", "oh no")
        		},
        		_ => {
        			panic!("{:?}", "oh no");
        		}
        	}
        }
    }

    fn lookup_key(&self, key: Key) -> Option<Partial> {
    	match key {
            Key::Ctrl('h') | Key::Char('h') | Key::Left  => { 
                Some(Partial::Action(Action::Motion(Motion::Goto(Column::Left(1), Line::Current))))
            }
            // Key::Ctrl('j') | Key::Char('j') | Key::Down  => { 
            //     Some(Command::goto(Span::Linewise, Column::Current, Line::Down(1)))
            // }
            // Key::Ctrl('p') | Key::Char('k') | Key::Up  => { 
            //     Some(Command::goto(Span::Linewise, Column::Current, Line::Up(1)))
            // }
            // Key::Char(' ') | Key::Char('l') | Key::Right  => { 
            //     Some(Command::goto(Span::Exclusive, Column::Right(1), Line::Current))
            // }
            // Key::Char('0')  => {
            //     Some(Command::goto(Span::Exclusive, Column::Specific(0), Line::Current))
            // }
            // Key::Char('^')  => {
            //     Some(Command::goto(Span::Exclusive, Column::Begin, Line::Current))
            // }
            // Key::Char('$')  => { 
            //     Some(Command::goto(Span::Exclusive, Column::End, Line::Current))
            // }
            // Key::Char('G')  => { 
            //     Some(Command::goto(Span::Linewise, Column::Current, Line::Last))
            // }
            // Key::Char('+') | Key::Char('m') => { 
            //     Some(Command::goto(Span::Exclusive, Column::Begin, Line::Down(1)))
            // }
            // Key::Char('-') => { 
            //     Some(Command::goto(Span::Exclusive, Column::Begin, Line::Up(1)))
            // }
            // Key::Ctrl('b') => {
            //     // what an awful way to do this.
            //     let lines = self.active().unwrap().window().1;
            //     let count = 1; // TODO: actually use [count]
            //     let relative = (count * (lines - 2));
            //     Some(Command::scroll(Line::Up(relative)))
            // }
            // Key::Ctrl('f') => {
            //     // what an awful way to do this.
            //     let lines = self.active().unwrap().window().1;
            //     let count = 1; // TODO: actually use [count]
            //     let relative = (count * (lines - 2));
            //     Some(Command::scroll(Line::Down(relative)))
            // }
            // Key::Ctrl('d') => {
            //     // what an awful way to do this.
            //     let lines = self.active().unwrap().window().1;
            //     let count = 1; // TODO: actually use [count]
            //     let relative = (count * (lines / 2));
            //     Some(Command::scroll(Line::Down(relative)))
            // }
            // Key::Ctrl('u') => {
            //     // what an awful way to do this.
            //     let lines = self.active().unwrap().window().1;
            //     let count = 1; // TODO: actually use [count]
            //     let relative = (count * (lines / 2));
            //     Some(Command::scroll(Line::Up(relative)))
            // }
            // Key::Ctrl('e') => {
            //     Some(Command::scroll(Line::Down(1)))
            // }
            // Key::Ctrl('y') => {
            //     Some(Command::scroll(Line::Up(1)))
            // }
            _ => None
        }
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

