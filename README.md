# Python-like Tokenizer in Rust

[![Static Badge](
https://img.shields.io/badge/-salam99823%2Ftokenizer-blue?logo=github&label=github)](
https://github.com/salam99823/tokenizer)
[![Crates.io Version](https://img.shields.io/crates/v/tokenizer_py)](https://crates.io/crates/tokenizer_py)
[![Crates.io MSRV (version)](
https://img.shields.io/crates/msrv/tokenizer_py?logo=rust)](
https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![docs.rs (with version)](
https://img.shields.io/docsrs/tokenizer_py/latest?logo=docs.rs)](
https://docs.rs/tokenizer_py)
[![GitHub Actions Workflow Status](
https://img.shields.io/github/actions/workflow/status/salam99823/tokenizer/rust.yml)](
https://github.com/salam99823/tokenizer/actions/workflows/rust.yml)
![Crates.io License](https://img.shields.io/crates/l/tokenizer_py)

This project implements a Python-like tokenizer in Rust.
It can tokenize a string into a sequence of tokens, which are
represented by the [`Token`] enum. The supported tokens are:

- [`Token::Name`]: a name token, such as a function or variable name.
- [`Token::Number`]: a number token, such as a literal integer or floating-point number.
- [`Token::String`]: a string token, such as a single or double-quoted string.
- [`Token::OP`]: an operator token, such as an arithmetic or comparison operator.
- [`Token::Indent`]: an indent token, indicating that a block of code is being indented.
- [`Token::Dedent`]: a dedent token, indicating that a block of code is being dedented.
- [`Token::Comment`]: a comment token, such as a single-line or multi-line comment.
- [`Token::NewLine`]: a newline token, indicating a new line in the source code.
- [`Token::NL`]: a token indicating a new line, for compatibility with the original tokenizer.
- [`Token::EndMarker`]: an end-of-file marker.

The tokenizer recognizes the following tokens:

- [x] `Whitespace`: spaces, tabs, and newlines.
- [x] `Numbers`: integers and floating-point numbers.
  - [x] `float`: floats numbers.
  - [x] `int`: integer numbers.
  - [x] `complex`: complex numbers.
- [x] `Names`: identifiers and keywords.
- [x] `Strings`: single- and double-quoted strings.
  - [x] `basic-String`: single- and double-quoted strings.
  - [x] `format-String`: format string from python.
  - [x] `byte-String`: byte string from python.
  - [x] `raw-String`: raw string.
  - [ ] `multy-line-String`: single- and double-quoted multy-line-string.
- [x] `Operators`: arithmetic, comparison, and other operators.
- [x] `Comments`: single-line comments.

The tokenizer also provides a `tokenize`
method that takes a string as input and returns a `Result` containing a vector
of tokens.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokenizer_py = "0.1.5"
```

## Exemples

### Example of using the tokenizer to tokenize the string "hello world"

```rust
use tokenizer_py::{tokenize, Token};

let tokens = tokenize("hello world").unwrap();
assert_eq!(tokens, vec![
    Token::Name("hello".to_string()), // Token of the name "hello"
    Token::Name("world".to_string()), // Token of the name "world"
    Token::NewLine, // New line token
    Token::EndMarker, // End of text token
]);
```

### Example of using the BinaryExp structure to evaluate the binary expression "10 + 10"

```rust
use tokenizer_py::{tokenize, Token};

// Structure representing a binary expression 
struct BinaryExp {
    left: Token,
    center: Token,
    right: Token,
}

impl BinaryExp {
    // Method for creating a new instance of BinaryExp
    fn new(left: Token, center: Token, right: Token) -> Self {
        BinaryExp { left, center, right }
    }
    // Method for executing the binary expression
    fn execute(&self) -> Result<isize, <isize as std::str::FromStr>::Err> {
        use Token::{Number, OP};
        match (&self.left, &self.center, &self.right) {
            (Number(ref left), OP(ref op), Number(ref right)) => {
                let (left, right) = (
                    left.parse::<isize>()?, right.parse::<isize>()?
                );
                match op.as_str() {
                    "+" => Ok(left + right),
                    "-" => Ok(left - right),
                    "*" => Ok(left * right),
                    "/" => Ok(left / right),
                    "%" => Ok(left % right),
                    _ => panic!("Invalid operator"), // Invalid operator
                }
            }
            _ => panic!("Invalid tokens"), // Invalid tokens
        }
    }
}
let mut tokens = tokenize("10 + 10").unwrap();
let _ = tokens.pop(); // Remove Token::EndMarker
let _ = tokens.pop(); // Remove Token::NewLine
let binexp = BinaryExp::new(
tokens.pop().unwrap(),
tokens.pop().unwrap(),
tokens.pop().unwrap()
);
assert_eq!(binexp.execute(), Ok(20)); // Checking the execution result
```

[`Token::Name`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.Name

[`Token::Number`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.Number

[`Token::String`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.String

[`Token::OP`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.OP

[`Token::Indent`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.Indent

[`Token::Dedent`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.Dedent

[`Token::Comment`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.Comment

[`Token::NewLine`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.NewLine

[`Token::NL`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.NL

[`Token::EndMarker`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#variant.EndMarker

[`Token`]: https://docs.rs/tokenizer_py/latest/tokenizer_py/enum.Token.html#
