use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;

/// The constant `OPERATORS` contains a string that lists
/// all possible operators that can be used in expressions.
pub const OPERATORS: &str = "=+-*/%&|<>!^:;.,()[]{}@$?~`";

/// An enumeration of Python tokens.
#[derive(Debug, PartialEq)]
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
}


/// An enumeration of possible errors that can occur during tokenization.
pub enum TokenizerError {
    /// An invalid operator was encountered.
    Operator(String),
    /// An invalid number was encountered.
    Number(String),
    /// An invalid indent was encountered.
    Indent(String),
    /// An invalid string was encountered.
    String(String),
}

impl Debug for TokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::Operator(s) => write!(f, "Invalid operator: {:?}", s),
            TokenizerError::Number(s) => write!(f, "Invalid number: {:?}", s),
            TokenizerError::Indent(s) => write!(f, "Invalid indent: {:?}", s),
            TokenizerError::String(s) => write!(f, "Invalid string: {:?}", s),
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
        }
    }
}

/// A simple counter.
/// But inside an ['usize'] number, keep this in mind when using.
///
///['usize']: https://doc.rust-lang.org/std/primitive.usize.html
/// # Examples
///
/// ```
/// use tokenizer_py::Counter;
///
/// let mut counter = Counter::new();
/// assert_eq!(0, *counter.get());
/// counter.inc();
/// counter.inc();
/// assert_eq!(2, *counter.get());
/// counter.dec();
/// assert_eq!(1, *counter.get());
/// ```
#[derive(Debug)]
pub struct Counter {
    count: usize,
}

impl Counter {
    /// Creates a new `Counter` that starts at 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Counter;
    ///
    /// let counter = Counter::new();
    /// assert_eq!(0, *counter.get());
    /// ```
    #[inline]
    pub const fn new() -> Counter {
        Counter { count: 0 }
    }

    /// Returns a reference to the current count.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Counter;
    ///
    /// let mut counter = Counter::new();
    /// assert_eq!(0, *counter.get());
    /// counter.inc();
    /// assert_eq!(1, *counter.get());
    /// ```
    #[inline]
    pub fn get(&self) -> &usize {
        &self.count
    }

    /// Increments the counter by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Counter;
    ///
    /// let mut counter = Counter::new();
    /// assert_eq!(0, *counter.get());
    /// counter.inc();
    /// assert_eq!(1, *counter.get());
    /// ```
    #[inline]
    pub fn inc(&mut self) {
        self.count += 1;
    }

    /// Decrements the counter by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Counter;
    ///
    /// let mut counter = Counter::new();
    /// counter.inc();
    /// counter.inc();
    /// assert_eq!(2, *counter.get());
    /// counter.dec();
    /// assert_eq!(1, *counter.get());
    /// ```
    #[inline]
    pub fn dec(&mut self) {
        self.count -= 1;
    }
}

/// A simple on/off switch.
///
/// # Examples
///
/// ```
/// use tokenizer_py::Switch;
///
/// let mut switch = Switch::new();
/// assert!(!switch.is_on());
/// switch.on();
/// assert!(switch.is_on());
/// switch.off();
/// assert!(!switch.is_on());
/// ```
#[derive(Debug)]
pub struct Switch {
    switch: bool,
}

impl Switch {
    /// Creates a new `Switch` that is initially off.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Switch;
    ///
    /// let switch = Switch::new();
    /// assert!(!switch.is_on());
    /// ```
    #[inline]
    pub const fn new() -> Switch {
        Switch { switch: false }
    }

    /// Returns `true` if the switch is on, `false` if it is off.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Switch;
    ///
    /// let mut switch = Switch::new();
    /// assert!(!switch.is_on());
    /// switch.on();
    /// assert!(switch.is_on());
    /// switch.off();
    /// assert!(!switch.is_on());
    /// ```
    #[inline]
    pub fn is_on(&self) -> bool {
        self.switch
    }

    /// Turns the switch on.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Switch;
    ///
    /// let mut switch = Switch::new();
    /// assert!(!switch.is_on());
    /// switch.on();
    /// assert!(switch.is_on());
    /// ```
    #[inline]
    pub fn on(&mut self) {
        self.switch = true;
    }

    /// Turns the switch off.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Switch;
    ///
    /// let mut switch = Switch::new();
    /// switch.on();
    /// assert!(switch.is_on());
    /// switch.off();
    /// assert!(!switch.is_on());
    /// ```
    #[inline]
    pub fn off(&mut self) {
        self.switch = false;
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
#[derive(Debug)]
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
                match c {
                    '\n' => if is_start_of_line.is_on() | opening.is_on() {
                        line_iter.next();
                        tokens.push(Token::NL);
                    } else {
                        line_iter.next();
                        tokens.push(Token::NewLine);
                        is_start_of_line.on();
                    }
                    ' ' | '\t' => if is_start_of_line.is_on() && *indent_count.get() > 0 {
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
                    } else {
                        line_iter.next();
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
        while let Some(c) = line.peek() {
            if *c == quot {
                line.next();
                break;
            }
            match c {
                '\\' => if let Some(c) = &line.next() {
                    match c {
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        'n' => string.push('\n'),
                        'r' => string.push('\r'),
                        't' => string.push('\t'),
                        'b' => string.push('\x08'),
                        'f' => string.push('\x0C'),
                        _ => return Err(TokenizerError::String(string)),
                    }
                }
                c => string.push(*c),
            }
            line.next();
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
        let tokenizer = Tokenizer::new("hello\n'world'\n2 + 2".to_owned());
        let tokens = tokenizer.tokenize().unwrap();
        let expects = vec![
            Token::Name("hello".to_owned()),
            Token::NewLine,
            Token::String("world".to_owned()),
            Token::NewLine,
            Token::Number("2".to_owned()),
            Token::OP("+".to_owned()),
            Token::Number("2".to_owned()),
            Token::EndMarker,
        ];
        for (actual, expect) in tokens.iter().zip(expects.iter()) {
            assert_eq!(actual, expect);
        }
    }
}