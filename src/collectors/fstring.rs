use crate::{privat::PeekableCharTracker, tokenize, Result, Token, TokenizeError};

/// Collects an f-string literal from the input iterator and adds tokens to the provided vector.
///
/// # Arguments
///
/// * `iter` - A mutable reference to the PeekableCharTracker iterator.
/// * `tokens` - A mutable reference to the vector of tokens to store the collected tokens.
/// * `prefix` - The prefix character indicating the type of f-string ('f', 'F').
///
/// # Returns
///
/// A Result indicating success or an error of type TokenizeError.
///
/// # Errors
///
/// * `TokenizeError::*` - for many reasons
pub fn collect_fstring(
    iter: &mut PeekableCharTracker,
    tokens: &mut Vec<Token>,
    prefix: char,
) -> Result<()> {
    // Get the first quote character
    let quot = iter.next().unwrap();

    // Check if the f-string is a multi-line f-string
    let multi_line = {
        let mut iter_clone = iter.clone();
        iter_clone.next() == Some(quot) && iter_clone.next() == Some(quot)
    };

    // Create a new FStringStart token and push it to the tokens vector
    tokens.push(Token::FStringStart(format!(
        "{}{}",
        prefix,
        if multi_line {
            iter.nth(1);
            quot.to_string().repeat(3)
        } else {
            quot.to_string()
        }
    )));

    // Continue iterating through the characters in the f-string
    while let Some(c) = iter.next_if(|c| *c != quot) {
        match c {
            '{' => {
                let mut inner = String::new();
                inner.push('{');
                while let Some(c) = iter.next_if(|c| *c != '}') {
                    inner.push(c);
                }
                inner.push(iter.next().unwrap());
                tokens.extend(match tokenize(inner) {
                    Ok(i) => i,
                    Err(e) => {
                        return Err({
                            let pos = iter.pos();
                            use TokenizeError::*;
                            match e {
                                EscapeSeq(msg, (iter_num, char_num)) => {
                                    EscapeSeq(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                String(msg, (iter_num, char_num)) => {
                                    String(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                Number(msg, (iter_num, char_num)) => {
                                    Number(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                Operator(msg, (iter_num, char_num)) => {
                                    Operator(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                Char(msg, (iter_num, char_num)) => {
                                    Char(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                Indent(msg, (iter_num, char_num)) => {
                                    Indent(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                EndOfFile(msg, (iter_num, char_num)) => {
                                    EndOfFile(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                            }
                        })
                    }
                });
                tokens.pop(); // Delete NewLine
                tokens.pop(); // Delente EndMarker
            }
            c => {
                let mut fstring_midle = String::new();
                fstring_midle.push(c);
                while let Some(c) = iter.next_if(|c| *c != quot && *c != '{') {
                    if c == '\n' && !multi_line {
                        return Err(TokenizeError::String(
                            "Unterminated f-string literal".to_owned(),
                            iter.pos(),
                        ));
                    }
                    fstring_midle.push(c);
                }
                if !fstring_midle.is_empty() {
                    tokens.push(Token::FStringMiddle(fstring_midle))
                }
            }
        }
    }
    // Create a new FStringEnd token and push it to the tokens vector
    tokens.push(Token::FStringEnd(if multi_line {
        // Move the iterator forward by 3 to skip the closing triple-quote characters
        iter.next();
        match (iter.next(), iter.next()) {
            // If the closing triple-quote characters are found, create a string of 3 quote characters
            (Some(second), Some(third)) if second == third && third == quot => {}
            // Otherwise, return an error
            _ => {
                return Err(TokenizeError::String(
                    "Unterminated triple-quoted f-string literal".to_owned(),
                    iter.pos(),
                ))
            }
        }
        quot.to_string().repeat(3)
    } else {
        // Move the iterator forward by 1 to skip the closing quote character
        iter.next();
        // Create a string of 1 quote character
        quot.to_string()
    }));
    Ok(())
}
