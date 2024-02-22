use collectors::{
    collect_comment, collect_fstring, collect_indent, collect_name, collect_number,
    collect_operator, collect_string,
};
pub use error::TokenizeError;

use privatestructs::ModPeekable;
use std::fmt::Display;
pub use token::Token;

mod collectors;
mod error;
mod privatestructs;

#[cfg(test)]
mod tests;
mod token;

/// The constant `OPERATORS` contains a string that lists
/// all possible operators that can be used in expressions.
pub const OPERATORS: &str = "=+-*/%&|<>!^:;.,()[]{}@$?~`";

/// An alias of type `Result<T>` for a standard Rust
/// result with a possible error of type `TokenizeError`.
pub type Result<T> = std::result::Result<T, TokenizeError>;

/// Tokinizes the text.
///
/// # Examples
///
/// ```
/// use tokenizer_py::{Token, tokenize};
///
/// let tokens = tokenize("hello\nworld").unwrap();
///
/// assert_eq!(tokens, vec![
///     Token::Name("hello".to_string()),
///     Token::NewLine,
///     Token::Name("world".to_string()),
///     Token::NewLine,
///     Token::EndMarker,
/// ]);
/// ```
pub fn tokenize(text: impl Display) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let text = format!("{}\n", text);
    let mut iter = ModPeekable::new(text.chars().peekable());
    let mut ind_stack: Vec<usize> = vec![0];
    while let Some(c) = iter.peek() {
        match *c {
            'r' | 'f' | 'b' | 'u' => {
                let c = iter.next();
                if let Some('\'' | '"') = iter.peek() {
                    if let Some('f') = c {
                        collect_fstring(&mut iter, &mut tokens)?;
                    } else {
                        tokens.push(Token::String(collect_string(&mut iter, c)?));
                    }
                } else {
                    tokens.push(Token::Name(collect_name(&mut iter, c)));
                }
            }
            '\'' | '"' => tokens.push(Token::String(collect_string(&mut iter, None)?)),
            '0'..='9' => tokens.push(Token::Number(collect_number(&mut iter)?)),
            '\n' => {
                if iter.pos().1 == 0 {
                    tokens.push(Token::NL);
                } else {
                    tokens.push(Token::NewLine);
                }

                iter.next();
                let new_ind = collect_indent(&mut iter);
                let last_ind = *ind_stack.last().unwrap();
                if new_ind.len() > last_ind {
                    ind_stack.push(new_ind.len());
                    tokens.push(Token::Indent(new_ind.clone()));
                }
                while new_ind.len() < *ind_stack.last().unwrap() {
                    ind_stack.pop();
                    tokens.push(Token::Dedent);
                }
                continue;
            }
            '#' => tokens.push(Token::Comment(collect_comment(&mut iter))),
            c => {
                if c.is_alphabetic() || c == '_' {
                    tokens.push(Token::Name(collect_name(&mut iter, None)));
                } else if OPERATORS.contains(c) {
                    tokens.push(Token::OP(collect_operator(&mut iter)?));
                } else if c.is_whitespace() {
                    iter.next();
                } else {
                    return Err(TokenizeError::Char(
                        format!("Unexpected char: {:?}", c),
                        *iter.pos(),
                    ));
                }
            }
        };
    }
    while *ind_stack.last().unwrap() > 0 {
        ind_stack.pop();
        tokens.push(Token::Dedent);
    }
    tokens.push(Token::EndMarker);
    Ok(tokens)
}
