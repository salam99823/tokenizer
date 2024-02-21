use crate::privatestructs::{Counter, Switch};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;

mod privatestructs;

/// The constant `OPERATORS` contains a string that lists
/// all possible operators that can be used in expressions.
pub const OPERATORS: &str = "=+-*/%&|<>!^:;.,()[]{}@$?~`";

/// An enumeration of Python tokens.
///
/// # Example
///
/// ```rust
/// use tokenizer_py::{Tokenizer, Token};
///
/// struct BinaryExp{
///     left: Token, center: Token,right: Token,
/// }
/// impl BinaryExp {
///     fn new(left: Token, center: Token, right: Token) -> Self {
///         BinaryExp { left, center, right}
///     }
///     fn execute(&self) -> Result<isize, <isize as std::str::FromStr>::Err> {
///         use Token::{Number, OP};
///         match (&self.left, &self.center, &self.right) {
///             (Number(ref left), OP(ref op), Number(ref right)) => match op.as_str() {
///                 "+" => Ok(left.parse::<isize>()? + right.parse::<isize>()?),
///                 "-" => Ok(left.parse::<isize>()? - right.parse::<isize>()?),
///                 "*" => Ok(left.parse::<isize>()? * right.parse::<isize>()?),
///                 "/" => Ok(left.parse::<isize>()? / right.parse::<isize>()?),
///                 "%" => Ok(left.parse::<isize>()? % right.parse::<isize>()?),
///                 _ => panic!("Invalid operator"),
///             }
///             _ => panic!("Invalid tokens"),
///         }
///     }
/// }
///
/// let tokenizer = Tokenizer::new("10 + 10".to_owned());
/// let mut tokens = tokenizer.tokenize().unwrap();
/// let _ = tokens.pop(); // remove EndMarker
///
/// let binexp = BinaryExp::new(
///     tokens.pop().unwrap(),
///     tokens.pop().unwrap(),
///     tokens.pop().unwrap()
/// );
///
/// assert_eq!(binexp.execute(), Ok(20));
///
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    /// Indicates the end of the program.
    EndMarker,
    /// A name token, such as a function or variable name.
    Name(String),
    /// A number token, such as a literal integer or floating-point number.
    Number(String),
    /// A string token, such as a single or double-quoted string.
    String(String),
    /// A newline token, indicating a new line in the source code.
    NewLine,
    /// An operator token, such as an arithmetic or comparison operator.
    OP(String),
    /// An indent token, indicating that a block of code is being indented.
    Indent(String),
    /// A dedent token, indicating that a block of code is being dedented.
    Dedent,
    /// A comment token, such as a single-line or multi-line comment.
    Comment(String),
    /// A token indicating a new line, for compatibility with the original tokenizer.
    NL,
    FStringStart(String),
    FStringMiddle(String),
    FStringEnd(String),
}


/// An enumeration of possible errors that can occur during tokenization.
///
/// # Examples
///
/// ```
/// use tokenizer_py::{Tokenizer, Token, TokenizerError};
///
/// let tokenizer = Tokenizer::new("1..1".to_string());
/// if let Err(err) = tokenizer.tokenize() {
///     assert_eq!(TokenizerError::Number("1..1".to_owned()), err);
/// }
/// ```
#[derive(PartialEq, Eq)]
pub enum TokenizerError {
    /// An invalid operator was encountered.
    Operator(String),
    /// An invalid number was encountered.
    Number(String),
    /// An invalid indent was encountered.
    Indent(String),
    /// An invalid string was encountered.
    String(String),
    /// An invalid escape sequance was encountered.
    EscapeSequance(String),
    /// An unexpected end of the string was encountered.
    EndOfFile,
}

impl Debug for TokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::Operator(s) => write!(f, "Invalid operator: {:?}", s),
            TokenizerError::Number(s) => write!(f, "Invalid number: {:?}", s),
            TokenizerError::Indent(s) => write!(f, "Invalid indent: {:?}", s),
            TokenizerError::String(s) => write!(f, "Invalid string: {:?}", s),
            TokenizerError::EscapeSequance(s) => write!(f, "Invalid escape sequance: {:?}", s),
            TokenizerError::EndOfFile => write!(f, "Unexpected end of file"),
        }
    }
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::Operator(s) => write!(f, "Invalid operator: {}", s),
            TokenizerError::Number(s) => write!(f, "Invalid number: {}", s),
            TokenizerError::Indent(s) => write!(f, "Invalid indent: {}", s),
            TokenizerError::String(s) => write!(f, "Invalid string: {}", s),
            TokenizerError::EscapeSequance(s) => write!(f, "Invalid escape sequance: {}", s),
            TokenizerError::EndOfFile => write!(f, "Unexpected end of file"),
        }
    }
}

impl Error for TokenizerError {
    fn description(&self) -> &str {
        match *self {
            TokenizerError::Operator(ref s) => s,
            TokenizerError::Number(ref s) => s,
            TokenizerError::Indent(ref s) => s,
            TokenizerError::String(ref s) => s,
            TokenizerError::EscapeSequance(ref s) => s,
            TokenizerError::EndOfFile => "Unexpected end of file",
        }
    }
}

/// A struct that can tokenize a string into tokens.
///
/// # Examples
///
/// ```
/// use tokenizer_py::{Tokenizer, Token};
///
/// let tokenizer = Tokenizer::new("hello world".to_string());
/// let tokens = tokenizer.tokenize().unwrap();
/// assert_eq!(tokens, vec![
///     Token::Name("hello".to_string()),
///     Token::Name("world".to_string()),
///     Token::EndMarker,
/// ]);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Tokenizer {
    text: String,
}


impl Tokenizer {
    /// Creates a new tokenizer that will tokenize the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Tokenizer;
    ///
    /// let tokenizer = Tokenizer::new("hello world".to_string());
    /// ```
    #[inline]
    pub const fn new(text: String) -> Tokenizer {
        Tokenizer { text }
    }
    /// Tokenizes the text that was provided to the tokenizer's constructor.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::{Token, Tokenizer};
    ///
    /// let tokenizer = Tokenizer::new("hello\nworld".to_string());
    /// let tokens = tokenizer.tokenize().unwrap();
    ///
    /// assert_eq!(tokens, vec![
    ///     Token::Name("hello".to_string()),
    ///     Token::NewLine,
    ///     Token::Name("world".to_string()),
    ///     Token::EndMarker,
    /// ]);
    /// ```
    pub fn tokenize(&self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens = Vec::new();
        let mut standart_indent = String::new();
        let mut stack_of_indents = Vec::new();
        let mut indent_count = Counter::new();
        let mut line_iter = self.text.chars().peekable();
        let mut is_start_of_line = Switch::new();
        let mut opening = Switch::new();
        while let Some(c) = line_iter.peek() {
            if is_start_of_line.is_on() &&
                stack_of_indents.len() == *indent_count.get() &&
                !stack_of_indents.is_empty() {
                tokens.push(Token::Dedent);
                stack_of_indents.pop();
                indent_count.dec();
            }
            if c.is_whitespace() {
                match (c, (is_start_of_line.is_on(), opening.is_on()), indent_count.get()) {
                    ('\n', (true, true) | (true, false) | (false, true), _) => {
                        line_iter.next();
                        tokens.push(Token::NL);
                    }
                    ('\n', (false, false), _) => {
                        line_iter.next();
                        tokens.push(Token::NewLine);
                        is_start_of_line.on();
                    }
                    (' ' | '\t', (true, _), 1..) => {
                        let new_indent = self.collect_indent(&mut line_iter)?;
                        if standart_indent.is_empty() {
                            standart_indent.push_str(new_indent.clone().as_str())
                        } else if (new_indent.len() % standart_indent.len()) > 0 {
                            return Err(TokenizerError::Indent(new_indent.clone()));
                        }
                        if *indent_count.get() > stack_of_indents.len() {
                            stack_of_indents.push(new_indent.clone());
                            tokens.push(Token::Indent(new_indent));
                        }
                        is_start_of_line.off();
                    }
                    _ => { line_iter.next(); }
                }
            } else if c.is_ascii_digit() {
                tokens.push(Token::Number(self.collect_number(&mut line_iter)?));
                is_start_of_line.off();
            } else if c.is_alphabetic() || *c == '_' {
                tokens.push(Token::Name(self.collect_name(&mut line_iter)));
                is_start_of_line.off();
            } else if "\"'".contains(*c) {
                tokens.push(Token::String(self.collect_string(&mut line_iter)?));
                is_start_of_line.off();
            } else if OPERATORS.contains(*c) {
                let op = self.collect_op(&mut line_iter)?;
                match op.as_str() {
                    ":" => indent_count.inc(),
                    "(" | "{" | "[" => opening.on(),
                    ")" | "}" | "]" => opening.off(),
                    _ => {}
                }
                tokens.push(Token::OP(op));
                is_start_of_line.off();
            } else if c == &'#' {
                tokens.push(Token::Comment(self.collect_comment(&mut line_iter)));
            } else if *c == '\\' {
                line_iter.next();
                match line_iter.peek() {
                    Some('\n') => { line_iter.next(); }
                    Some(s) => return Err(TokenizerError::EscapeSequance(format!("\\{}", s))),
                    None => return Err(TokenizerError::EndOfFile),
                }
            } else {
                line_iter.next();
            }
        }
        tokens.push(Token::EndMarker);
        Ok(tokens)
    }
    /// private method to collect padding as Python tokenizer
    fn collect_indent(&self, line: &mut Peekable<Chars>) -> Result<String, TokenizerError> {
        let mut new_indent = String::new();
        while let Some(c2) = line.peek() {
            match c2 {
                '\t' => new_indent.push('\t'),
                ' ' => new_indent.push(' '),
                _ => break,
            }
            line.next();
        }
        Ok(new_indent)
    }
    /// private method to collect number as Python tokenizer
    fn collect_number(&self, line: &mut Peekable<Chars>) -> Result<String, TokenizerError> {
        let mut number = String::new();
        while let Some(c) = line.next_if(|c| c.is_ascii_digit() || "_.".contains(*c)) {
            number.push(c);
        }
        if number.chars().filter(|c| c == &'.').count() > 1 {
            return Err(TokenizerError::Number(number));
        }
        Ok(number)
    }
    /// private method to collect names as Python tokenizer
    fn collect_name(&self, line: &mut Peekable<Chars>) -> String {
        let mut name = String::new();
        while let Some(c) = line.next_if(
            |c| !c.is_whitespace() && !OPERATORS.contains(*c)) {
            name.push(c);
        }
        name
    }
    /// private method to collect operators as Python tokenizer
    fn collect_op(&self, line: &mut Peekable<Chars>) -> Result<String, TokenizerError> {
        Ok(match line.next().unwrap() {
            '=' => "=".to_owned(),
            '+' => match line.peek() {
                Some('=') => {
                    line.next();
                    "+=".to_owned()
                }
                _ => "+".to_owned(),
            },
            '-' => match line.peek() {
                Some('=') => {
                    line.next();
                    "-=".to_owned()
                }
                _ => "-".to_owned(),
            },
            '*' => match line.peek() {
                Some('=') => {
                    line.next();
                    "*=".to_owned()
                }
                Some('*') => {
                    line.next();
                    match line.peek() {
                        Some('=') => {
                            line.next();
                            "**=".to_owned()
                        }
                        _ => "**".to_owned(),
                    }
                }
                _ => "*".to_owned(),
            },
            '/' => match line.peek() {
                Some('=') => {
                    line.next();
                    "/=".to_owned()
                }
                Some('/') => {
                    line.next();
                    match line.peek() {
                        Some('=') => {
                            line.next();
                            "//=".to_owned()
                        }
                        _ => "//".to_owned(),
                    }
                }
                _ => "/".to_owned(),
            },
            '%' => match line.peek() {
                Some('=') => {
                    line.next();
                    "%=".to_owned()
                }
                _ => "%".to_owned(),
            }
            '&' => match line.peek() {
                Some('=') => {
                    line.next();
                    "&=".to_owned()
                }
                _ => "&".to_owned(),
            },
            '|' => match line.peek() {
                Some('=') => {
                    line.next();
                    "|=".to_owned()
                }
                _ => "|".to_owned(),
            }
            '<' => match line.peek() {
                Some('=') => {
                    line.next();
                    "<=".to_owned()
                }
                Some('<') => {
                    line.next();
                    match line.peek() {
                        Some('=') => {
                            line.next();
                            "<<=".to_owned()
                        }
                        _ => "<<".to_owned(),
                    }
                }
                _ => "<".to_owned(),
            },
            '>' => match line.peek() {
                Some('=') => {
                    line.next();
                    ">=".to_owned()
                }
                Some('>') => {
                    line.next();
                    match line.peek() {
                        Some('=') => {
                            line.next();
                            ">>=".to_owned()
                        }
                        _ => ">>".to_owned(),
                    }
                }
                _ => ">".to_owned(),
            },
            '!' => match line.peek() {
                Some('=') => {
                    line.next();
                    "!=".to_owned()
                }
                _ => "!".to_owned(),
            },
            '^' => match line.peek() {
                Some('=') => {
                    line.next();
                    "^=".to_owned()
                }
                _ => "^".to_owned(),
            }
            ':' => match line.peek() {
                Some('=') => {
                    line.next();
                    ":=".to_owned()
                }
                _ => ":".to_owned(),
            },
            ';' => ";".to_owned(),
            '.' => ".".to_owned(),
            ',' => ",".to_owned(),
            '(' => "(".to_owned(),
            ')' => ")".to_owned(),
            '[' => "[".to_owned(),
            ']' => "]".to_owned(),
            '{' => "{".to_owned(),
            '}' => "}".to_owned(),
            '@' => match line.peek() {
                Some('=') => {
                    line.next();
                    "@=".to_owned()
                }
                _ => "@".to_owned(),
            }
            '$' => "$".to_owned(),
            '?' => "?".to_owned(),
            '~' => "~".to_owned(),
            '`' => "`".to_owned(),
            op => return Err(TokenizerError::Operator(op.to_string()))
        })
    }
    /// private method to collect string as Python tokenizer
    fn collect_string(&self, line: &mut Peekable<Chars>) -> Result<String, TokenizerError> {
        let mut string = String::new();
        let quot = line.next().unwrap();
        string.push(quot);
        while let Some(c) = line.peek() {
            match c {
                '\\' => {
                    line.next();
                    if let Some(c) = &line.next() {
                        match *c {
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            '\'' => string.push('\''),
                            'n' => string.push('\n'),
                            'r' => string.push('\r'),
                            't' => string.push('\t'),
                            'b' => string.push('\x08'),
                            'f' => string.push('\x0C'),
                            'v' => string.push('\x0D'),
                            'a' => string.push('\x07'),
                            '\n' => {
                                continue;
                            }
                            c => return Err(
                                TokenizerError::EscapeSequance(
                                    format!("unexpected escape sequence: '\\{}'", c)), ),
                        }
                    } else {
                        return Err(TokenizerError::EndOfFile);
                    }
                }
                '\n' => return Err(TokenizerError::String(format!("{}\n", string))),
                c => {
                    if *c == quot {
                        string.push(*c);
                        line.next();
                        break;
                    }
                    string.push(*c);
                }
            }
            line.next();
        }
        if string.chars().filter(|c| *c == quot).count() < 2 {
            return Err(TokenizerError::String(string));
        }
        Ok(string)
    }
    /// private method to collect comment as Python tokenizer
    fn collect_comment(&self, line: &mut Peekable<Chars>) -> String {
        let mut comment = String::new();
        while let Some(c) = line.next_if(|c| *c != '\n') {
            comment.push(c);
        }
        comment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_work() {
        let tokenizer = Tokenizer::new(
            "hello\n'world'\n2 + 2\nfor i in range(10):\n    print(i)\npass"
                .to_owned()
        );
        println!("hello\n'world'\n2 + 2\nfor i in range(10):\n    print(i)\npass\n");
        let tokens = tokenizer.tokenize().unwrap();
        let expects = vec![
            Token::Name("hello".to_owned()),
            Token::NewLine,
            Token::String("'world'".to_owned()),
            Token::NewLine,
            Token::Number("2".to_owned()),
            Token::OP("+".to_owned()),
            Token::Number("2".to_owned()),
            Token::NewLine,
            Token::Name("for".to_owned()),
            Token::Name("i".to_owned()),
            Token::Name("in".to_owned()),
            Token::Name("range".to_owned()),
            Token::OP("(".to_owned()),
            Token::Number("10".to_owned()),
            Token::OP(")".to_owned()),
            Token::OP(":".to_owned()),
            Token::NewLine,
            Token::Indent("    ".to_owned()),
            Token::Name("print".to_owned()),
            Token::OP("(".to_owned()),
            Token::Name("i".to_owned()),
            Token::OP(")".to_owned()),
            Token::Dedent,
            Token::EndMarker,
        ];
        for (actual, expect) in tokens.iter().zip(expects.iter()) {
            println!("{:?}", actual);
            assert_eq!(actual, expect);
        }
    }
}