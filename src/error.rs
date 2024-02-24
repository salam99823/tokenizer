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
                let (line, character) = pos;
                write!(f, "Escape Sequence Error: {} at pos {}:{}", desc, line, character)
            }
            TokenizeError::String(desc, pos) => {
                let (line, character) = pos;
                write!(f, "String Error: {} at pos {}:{}", desc, line, character)
            }
            TokenizeError::Number(desc, pos) => {
                let (line, character) = pos;
                write!(f, "Number Error: {} at pos {}:{}", desc, line, character)
            }
            TokenizeError::Operator(desc, pos) => {
                let (line, character) = pos;
                write!(f, "Operator Error: {} at pos {}:{}", desc, line, character)
            }
            TokenizeError::Char(desc, pos) => {
                let (line, character) = pos;
                write!(f, "Character Error: {} at pos {}:{}", desc, line, character)
            }
            TokenizeError::Indent(desc, pos) => {
                let (line, character) = pos;
                write!(f, "Indentation Error: {} at pos {}:{}", desc, line, character)
            }
            TokenizeError::EndOfFile(desc, pos) => {
                let (line, character) = pos;
                write!(f, "End of File Error: {} at pos {}:{}", desc, line, character)
            }
        }
    }
}

impl Display for TokenizeError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for TokenizeError {}
