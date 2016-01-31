pub use editor::state::State;
pub use editor::command::{Command, Span, Motion, Line, Column};

// column, line
type Coord = (usize, usize);
type Range = (Coord, Coord);

#[test]
fn compare_coord() {
	let a = (0, 5);
	let b = (10, 6);
	assert!(b > a);

	let a = (0, 6);
	let b = (10, 6);
	assert!(b > a);

	let a = (10, 6);
	let b = (10, 6);
	assert!(b == a);

	let a = (25, 6);
	let b = (10, 6);
	assert!(b < a);
}

enum Yank {
	YankOnly, YankDel
}

enum Direction {
    Up, Down, //Left, Right,
}

mod command;
mod state;
mod buffer;
