#[derive(Debug, PartialEq)]
pub enum JsonInputErr {
    IoErr,
    InvalidUtf8,
    InvalidJson,
    InvalidEscape(u8),
    UnexpectedEndOfInput,
}
