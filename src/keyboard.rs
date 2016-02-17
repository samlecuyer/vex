
// lifted verbatim from
// https://github.com/gchp/rustbox/blob/master/src/keyboard.rs

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Tab,
    Enter,
    Esc,
    Backspace,
    Right,
    Left,
    Up,
    Down,
    Delete,
    Insert,

    Home,
    End,
    PageUp,
    PageDown,

    Char(char),
    Ctrl(char),
    F(u32),
    Unknown(u16),
}
