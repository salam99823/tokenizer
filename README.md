Python-like Tokenizer in Rust
=============================

[![Static Badge](https://img.shields.io/badge/-salam99823%2Ftokenizer-blue?label=github)](https://github.com/salam99823/tokenizer)
[![Crates.io Version](https://img.shields.io/crates/v/tokenizer_py)](https://crates.io/crates/tokenizer_py)
[![Crates.io MSRV (version)](https://img.shields.io/crates/msrv/tokenizer_py/0.1.1?logo=rust)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![docs.rs (with version)](https://img.shields.io/docsrs/tokenizer_py/0.1.1?logo=docs.rs)](https://docs.rs/tokenizer_py)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/salam99823/tokenizer/rust.yml)](https://github.com/salam99823/tokenizer/actions/workflows/rust.yml)
![Crates.io License](https://img.shields.io/crates/l/tokenizer_py)


This project implements a Python-like tokenizer in Rust. It can tokenize a string into a sequence of tokens, which are
represented by the [`Token`] enum. The supported tokens are:

- [`Token::Name`]: a name token, such as a function or variable name
- [`Token::Number`]: a number token, such as a literal integer or floating-point number
- [`Token::String`]: a string token, such as a single or double-quoted string
- [`Token::OP`]: an operator token, such as an arithmetic or comparison operator
- [`Token::Indent`]: an indent token, indicating that a block of code is being indented
- [`Token::Dedent`]: a dedent token, indicating that a block of code is being dedented
- [`Token::Comment`]: a comment token, such as a single-line or multi-line comment
- [`Token::NewLine`]: a newline token, indicating a new line in the source code
- [`Token::NL`]: a token indicating a new line, for compatibility with the original tokenizer
- [`Token::EndMarker`]: an end-of-file marker

The tokenizer uses a simple state machine to tokenize the input text. It recognizes the following tokens:

- [x] `Whitespace`: spaces, tabs, and newlines
- [x] `Numbers`: integers and floating-point numbers
    - [x] `float`: floats numbers
    - [x] `int`: integer numbers
- [x] `Names`: identifiers and keywords
- [x] `Strings`: single- and double-quoted strings
    - [x] `basic-String`: single- and double-quoted strings
    - [ ] `format-String`: format string from python
    - [ ] `byte-String`: byte string from python
    - [ ] `raw-String`: raw string
    - [ ] `multy-line-String`: single- and double-quoted multy-line-string
- [x] `Operators`: arithmetic, comparison, and other operators
- [x] `Comments`: single-line comments

The tokenizer also provides a `tokenize` method that takes a string as input and returns a `Result` containing a vector
of tokens.

Here is an example of how to use the tokenizer:

```rust
use tokenizer_py::{Tokenizer, Token};

let tokenizer = Tokenizer::new("hello world".to_string());
let tokens = tokenizer.tokenize().unwrap();
assert_eq!(tokens, vec![
    Token::Name("hello".to_string()),
    Token::Name("world".to_string()),
    Token::EndMarker,
]);
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tokenizer_py = "0.1.1"
```

## Error Handling

The tokenizer uses the [`Result`] type to indicate possible errors during tokenization. The possible errors are:

- [`TokenizerError::Operator`]: an invalid operator was encountered
- [`TokenizerError::Number`]: an invalid number was encountered
- [`TokenizerError::Indent`]: an invalid indent was encountered
- [`TokenizerError::String`]: an invalid string was encountered

Here is an example of how to handle these errors:

```rust
match tokenizer.tokenize() {
Ok(tokens) => {
// process tokens
}
Err(TokenizerError::Operator(op)) => {
// handle invalid operator
}
Err(TokenizerError::Number(num)) => {
// handle invalid number
}
Err(TokenizerError::Indent(indent)) => {
// handle invalid indent
}
Err(TokenizerError::String(string)) => {
// handle invalid string
}
}
```
[`TokenizerError::Operator`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.TokenizerError.html#variant.Operator
[`TokenizerError::Number`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.TokenizerError.html#variant.Number
[`TokenizerError::Indent`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.TokenizerError.html#variant.Indent
[`TokenizerError::String`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.TokenizerError.html#variant.String
[`Token::Name`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.Name
[`Token::Number`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.Number
[`Token::String`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.String
[`Token::OP`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.OP
[`Token::Indent`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.Indent
[`Token::Dedent`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.Dedent
[`Token::Comment`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.Comment
[`Token::NewLine`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.NewLine
[`Token::NL`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.NL
[`Token::EndMarker`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#variant.EndMarker
[`Token`]: https://docs.rs/tokenizer_py/0.1.1/tokenizer_py/enum.Token.html#
[`Result`]: https://doc.rust-lang.org/core/result/enum.Result.html