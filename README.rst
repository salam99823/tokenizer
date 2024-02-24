Python-like Tokenizer in Rust
=============================

.. image:: https://img.shields.io/badge/-salam99823%2Ftokenizer-blue?logo=github&label=github
   :target: https://github.com/salam99823/tokenizer

.. image:: https://img.shields.io/crates/v/tokenizer_py
   :target: https://crates.io/crates/tokenizer_py

.. image:: https://img.shields.io/crates/msrv/tokenizer_py?logo=rust
   :target: https://rust-lang.github.io/rfcs/2495-min-rust-version.html

.. image:: https://img.shields.io/docsrs/tokenizer_py/latest?logo=docs.rs
   :target: https://docs.rs/tokenizer_py

.. image:: https://img.shields.io/github/actions/workflow/status/salam99823/tokenizer/rust.yml
   :target: https://github.com/salam99823/tokenizer/actions/workflows/rust.yml

.. image:: https://img.shields.io/crates/l/tokenizer_py

This project implements a Python-like tokenizer in Rust. It can tokenize a string into a sequence of tokens, which are represented by the ``Token`` enum. The supported tokens are:

- ``Token::Name``: a name token, such as a function or variable name.
- ``Token::Number``: a number token, such as a literal integer or floating-point number.
- ``Token::String``: a string token, such as a single or double-quoted string.
- ``Token::OP``: an operator token, such as an arithmetic or comparison operator.
- ``Token::Indent``: an indent token, indicating that a block of code is being indented.
- ``Token::Dedent``: a dedent token, indicating that a block of code is being dedented.
- ``Token::Comment``: a comment token, such as a single-line or multi-line comment.
- ``Token::NewLine``: a newline token, indicating a new line in the source code.
- ``Token::NL``: a token indicating a new line, for compatibility with the original tokenizer.
- ``Token::EndMarker``: an end-of-file marker.

The tokenizer recognizes the following tokens:

- [x] Whitespace: spaces, tabs, and newlines.
- [x] Numbers: integers and floating-point numbers.
  - [x] float: floats numbers.
  - [x] int: integer numbers.
  - [x] complex: complex numbers.
- [x] Names: identifiers and keywords.
- [x] Strings: single- and double-quoted strings.
  - [x] basic-String: single- and double-quoted strings.
  - [x] format-String: format string from python.
  - [x] byte-String: byte string from python.
  - [x] raw-String: raw string.
  - [x] multy-line-String: single- and double-quoted multy-line-string.
  - [ ] combined-string: string with combined prefix.
- [x] Operators: arithmetic, comparison, and other operators.
- [x] Comments: single-line comments.

The tokenizer also provides a ``tokenize`` method that takes a string as input and returns a ``Result`` containing a vector of tokens.

Usage
-----

Add this to your ``Cargo.toml``:

.. code-block:: toml

    [dependencies]
    tokenizer_py = "0.2.0"

Examples
--------

Example of using the tokenizer to tokenize the string "hello world":

.. code-block:: rust

    use tokenizer_py::{tokenize, Token};
    let tokens = tokenize("hello world").unwrap();
    assert_eq!(tokens, vec![
        Token::Name("hello".to_string()), // Token of the name "hello"
        Token::Name("world".to_string()), // Token of the name "world"
        Token::NewLine, // New line token
        Token::EndMarker, // End of text token
    ])

Example of using the BinaryExp structure to evaluate the binary expression "10 + 10":

.. code-block:: rust

    use tokenizer_py::{tokenize, Token};

    struct BinaryExp {
        left: Token,
        center: Token,
        right: Token,
    }

    impl BinaryExp {
        fn new(left: Token, center: Token, right: Token) -> Self {
            BinaryExp { left, center, right }
        }

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