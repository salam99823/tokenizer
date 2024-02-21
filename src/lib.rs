use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;

#[cfg(test)]
mod tests;

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
/// let tokenizer = Tokenizer::new("10 + 10");
/// let mut tokens = tokenizer.tokenize().unwrap();
/// let _ = tokens.pop(); // remove Token::EndMarker
/// let _ = tokens.pop(); // remove Token::NewLine
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
    /// Indicates the end of the text.
    EndMarker,
    /// A name token, such as a function or variable or special name.
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

    /// not yet supported,
    /// i will be gratefull for your help in implementation
    FStringStart(String),
    /// not yet supported,
    /// i will be gratefull for your help in implementation
    FStringMiddle(String),
    /// not yet supported,
    /// i will be gratefull for your help in implementation
    FStringEnd(String),
}

/// An enumeration of possible errors that can occur during tokenization.
///
/// # Examples
///
/// ```
/// use tokenizer_py::{Tokenizer, Token, TokenizerError};
///
/// let tokenizer = Tokenizer::new("1..1");
/// if let Err(err) = tokenizer.tokenize() {
///     assert_eq!(TokenizerError("Invalid number: \"1..1\"".to_owned()), err);
/// }
/// ```
#[derive(PartialEq, Eq)]
pub struct TokenizerError(pub String);

impl Debug for TokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TokenizerError {
    fn description(&self) -> &str {
        &self.0
    }
}

/// A struct that can tokenize a string into tokens.
///
/// # Examples
///
/// ```
/// use tokenizer_py::{Tokenizer, Token};
///
/// let tokenizer = Tokenizer::new("hello world");
/// let tokens = tokenizer.tokenize().unwrap();
/// assert_eq!(tokens, vec![
///     Token::Name("hello".to_string()),
///     Token::Name("world".to_string()),
///     Token::NewLine,
///     Token::EndMarker,
/// ]);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Tokenizer<'a> {
    text: &'a str,
}

impl Tokenizer<'_> {
    /// Creates a new tokenizer that will tokenize the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::Tokenizer;
    ///
    /// let tokenizer = Tokenizer::new("hello world");
    /// ```
    #[inline]
    pub const fn new(text: &str) -> Tokenizer {
        Tokenizer { text }
    }
    /// Tokenizes the text that was provided to the tokenizer's constructor.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokenizer_py::{Token, Tokenizer};
    ///
    /// let tokenizer = Tokenizer::new("hello\nworld");
    /// let tokens = tokenizer.tokenize().unwrap();
    ///
    /// assert_eq!(tokens, vec![
    ///     Token::Name("hello".to_string()),
    ///     Token::NewLine,
    ///     Token::Name("world".to_string()),
    ///     Token::NewLine,
    ///     Token::EndMarker,
    /// ]);
    /// ```
    pub fn tokenize(&self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut text = self.text.to_owned();
        text.push('\n');
        let mut iter: Peekable<Chars> = text.chars().peekable();
        let mut is_start_of_string = false;
        let mut ind_stack: Vec<usize> = vec![0];
        while let Some(c) = iter.peek() {
            match *c {
                'r' | 'f' | 'b' | 'u' => {
                    let c = iter.next();
                    if let Some('\'' | '"') = iter.peek() {
                        tokens.push(Token::String(self.collect_string(&mut iter, c)?));
                    } else {
                        tokens.push(Token::Name(self.collect_name(&mut iter, c)));
                    }
                }
                '\'' | '"' => tokens.push(Token::String(self.collect_string(&mut iter, None)?)),
                '0'..='9' => tokens.push(Token::Number(self.collect_number(&mut iter)?)),
                '\n' => {
                    if is_start_of_string {
                        tokens.push(Token::NL);
                    } else {
                        tokens.push(Token::NewLine);
                    }
                    iter.next();
                    let new_ind = self.collect_indent(&mut iter)?;
                    let last_ind = *ind_stack.last().unwrap();
                    if new_ind.len() > last_ind {
                        ind_stack.push(new_ind.len());
                        tokens.push(Token::Indent(new_ind.clone()));
                    }
                    while new_ind.len() < *ind_stack.last().unwrap() {
                        ind_stack.pop();
                        tokens.push(Token::Dedent);
                    }
                    is_start_of_string = true;
                    continue;
                }
                '#' => tokens.push(Token::Comment(self.collect_comment(&mut iter))),
                c => {
                    if c.is_alphabetic() || c == '_' {
                        tokens.push(Token::Name(self.collect_name(&mut iter, None)));
                    } else if OPERATORS.contains(c) {
                        tokens.push(Token::OP(self.collect_op(&mut iter)?));
                    } else if c.is_whitespace() {
                        iter.next();
                    } else {
                        return Err(TokenizerError(format!("Unexpected char: {:?}", c)));
                    }
                }
            };
            is_start_of_string = false;
        }
        while *ind_stack.last().unwrap() > 0 {
            ind_stack.pop();
            tokens.push(Token::Dedent);
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
        while let Some(c) = line.next_if(|c| matches!(c, '0'..='9' | '_' | '.')) {
            number.push(c);
        }
        if number.chars().filter(|c| c == &'.').count() > 1 {
            return Err(TokenizerError(format!("Invalid number: {:?}", number)));
        }
        Ok(number)
    }
    /// private method to collect names as Python tokenizer
    fn collect_name(&self, line: &mut Peekable<Chars>, c: Option<char>) -> String {
        let mut name = String::new();
        if let Some(c) = c {
            name.push(c);
        }
        while let Some(c) = line.next_if(|c| !c.is_whitespace() && !OPERATORS.contains(*c)) {
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
                Some('>') => {
                    line.next();
                    "->".to_owned()
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
            },
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
            },
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
                Some('>') => "<>".to_owned(),
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
            },
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
            },
            '$' => "$".to_owned(),
            '?' => "?".to_owned(),
            '~' => "~".to_owned(),
            '`' => "`".to_owned(),
            op => return Err(TokenizerError(format!("Invalid operator: {:?}", op))),
        })
    }
    /// private method to collect string as Python tokenizer
    fn collect_string(
        &self,
        line: &mut Peekable<Chars>,
        c: Option<char>,
    ) -> Result<String, TokenizerError> {
        let mut string = String::new();
        let quot = line.next().unwrap();
        if let Some(ref c) = c {
            match *c {
                'f' | 'u' | 'b' | 'r' => string.push(*c),
                c => return Err(TokenizerError(format!("Invalid prefix: {:?}", c))),
            }
        }
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
                            c => {
                                let msg = format!("\\{}", c);
                                return Err(TokenizerError(format!(
                                    "Unexpected escape sequence: {:?}",
                                    msg
                                )));
                            }
                        }
                    } else {
                        return Err(TokenizerError("Unexpected EndOfFile".to_owned()));
                    }
                }
                '\n' => return Err(TokenizerError(format!("{}\n", string))),
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
            return Err(TokenizerError(string));
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
