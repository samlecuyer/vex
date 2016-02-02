
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

// TODO; what goes here
struct Builder;

#[derive(Debug)] 
pub enum Line {
    Current,
    Up(usize), Down(usize),
    Specific(usize),
    Last
}

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

impl Builder {
    pub fn new() -> Builder {
        Builder
    }
}

