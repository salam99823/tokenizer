# Python-like Tokenizer in Rust

This project implements a Python-like tokenizer in Rust. It can tokenize a string into a sequence of tokens, which are represented by the `Token` enum. The supported tokens are:

- `Name`: a name token, such as a function or variable name
- `Number`: a number token, such as a literal integer or floating-point number
- `String`: a string token, such as a single or double-quoted string
- `OP`: an operator token, such as an arithmetic or comparison operator
- `Indent`: an indent token, indicating that a block of code is being indented
- `Dedent`: a dedent token, indicating that a block of code is being dedented
- `Comment`: a comment token, such as a single-line or multi-line comment
- `NewLine`: a newline token, indicating a new line in the source code
- `NL`: a token indicating a new line, for compatibility with the original tokenizer
- `EndMarker`: an end-of-file marker

The tokenizer uses a simple state machine to tokenize the input text. It recognizes the following tokens:

- Whitespace: spaces, tabs, and newlines
- Numbers: integers and floating-point numbers
- Names: identifiers and keywords
- Strings: single- and double-quoted strings
- Operators: arithmetic, comparison, and other operators
- Comments: single- and multi-line comments

The tokenizer also provides a `tokenize` method that takes a string as input and returns a `Result` containing a vector of tokens.

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

## Error Handling

The tokenizer uses the `Result` type to indicate possible errors during tokenization. The possible errors are:

- `Operator`: an invalid operator was encountered
- `Number`: an invalid number was encountered
- `Indent`: an invalid indent was encountered
- `String`: an invalid string was encountered

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