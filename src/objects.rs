
#[derive(Debug)]
pub enum TextObject {
    Char,
    Word,
    Line,
    Sentence,
    Paragraph,
}

#[derive(Debug)]
pub enum Relative {
    Before,
    Begin,
    Current,
    End,
    After,
}
