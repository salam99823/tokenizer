use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// An enumeration of possible errors that can occur during tokenization.
///
/// # Examples
///
/// ```
/// use tokenizer_py::{tokenize, Token, TokenizeError};
///
///
/// assert_eq!(
///     Err(TokenizeError::Number("Invalid decimal literal".to_owned(), (1, 3))), 
///     tokenize("1_.1")
/// );
/// ```
#[derive(PartialEq, Eq)]
pub enum TokenizeError {
    EscapeSeq(String, (usize, usize)),
    String(String, (usize, usize)),
    Number(String, (usize, usize)),
    Operator(String, (usize, usize)),
    Char(String, (usize, usize)),
    Indent(String, (usize, usize)),
    EndOfFile(String, (usize, usize)),
}

impl Debug for TokenizeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizeError::EscapeSeq(desc, pos) => {
                write!(f, "Escape Sequence Error: {} at pos {:?}", desc, pos)
            }
            TokenizeError::String(desc, pos) => {
                write!(f, "String Error: {} at pos {:?}", desc, pos)
            }
            TokenizeError::Number(desc, pos) => {
                write!(f, "Number Error: {} at pos {:?}", desc, pos)
            }
            TokenizeError::Operator(desc, pos) => {
                write!(f, "Operator Error: {} at pos {:?}", desc, pos)
            }
            TokenizeError::Char(desc, pos) => {
                write!(f, "Character Error: {} at pos {:?}", desc, pos)
            }
            TokenizeError::Indent(desc, pos) => {
                write!(f, "Indentation Error: {} at pos {:?}", desc, pos)
            }
            TokenizeError::EndOfFile(desc, pos) => {
                write!(f, "End of File Error: {} at pos {:?}", desc, pos)
            }
        }
    }
}

impl Display for TokenizeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for TokenizeError {}
