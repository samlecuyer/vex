pub use editor::state::State;

// column, line
type Coord = (isize, isize);

// enum Yank {
// 	YankOnly, YankDel
// }

enum Direction {
    Up,
    Down,
}

mod state;
mod buffer;

// fn umin(a: usize, b: usize) -> usize {
// 	if a < b { a } else { b }
// }

// fn umax(a: usize, b: usize) -> usize {
// 	if a > b { a } else { b }
// }

fn imin(a: isize, b: isize) -> isize {
	if a < b { a } else { b }
}

fn imax(a: isize, b: isize) -> isize {
	if a > b { a } else { b }
}