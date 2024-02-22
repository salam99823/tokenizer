use std::fmt::Debug;

/// An enumeration of Python tokens.
///
/// # Example
///
/// ```rust
/// use tokenizer_py::{tokenize, Token};
/// struct BinaryExp{
///     left: Token, center: Token,right: Token,
/// }
///
/// impl BinaryExp {
///     fn new(left: Token, center: Token, right: Token) -> Self {
///         BinaryExp { left, center, right}
///     }
///     fn execute(&self) -> Result<isize, <isize as std::str::FromStr>::Err> {
///         use Token::{Number, OP};
///         match (&self.left, &self.center, &self.right) {
///             (Number(ref left), OP(ref op), Number(ref right)) => {
///               let (left, right) = (
///                 left.parse::<isize>()?, right.parse::<isize>()?
///               );
///               match op.as_str() {
///                 "+" => Ok(left + right),
///                 "-" => Ok(left - right),
///                 "*" => Ok(left * right),
///                 "/" => Ok(left / right),
///                 "%" => Ok(left % right),
///                 _ => panic!("Invalid operator"),
///               }
///             }
///             _ => panic!("Invalid tokens"),
///         }
///     }
/// }
/// let mut tokens = tokenize("10 + 10").unwrap();
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
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    /// Indicates the end of the text.
    EndMarker,
    /// A name token, such as a function, variable, or special name.
    Name(String),
    /// A number token, such as a literal integer or floating-point number.
    Number(String),
    /// A string token, such as a single or double-quoted string.
    String(String),
    /// A newiter token, indicating a new iter in the source code.
    NewLine,
    /// An operator token, such as an arithmetic or comparison operator.
    OP(String),
    /// An indent token, indicating that a block of code is being indented.
    Indent(String),
    /// A dedent token, indicating that a block of code is being dedented.
    Dedent,
    /// A comment token, such as a single-iter or multi-iter comment.
    Comment(String),
    /// A token indicating a new iter, for compatibility with the original tokenizer.
    NL,
    /// A token indicating the start of a formatted string.
    FStringStart(String),
    /// A token indicating the middle of a formatted string.
    FStringMiddle(String),
    /// A token indicating the end of a formatted string.
    FStringEnd(String),
}
