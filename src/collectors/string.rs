use crate::{privat::PeekableCharTracker, Result, TokenizeError};

/// Collects a string from the iterator.
///
/// # Arguments
///
/// * `iter` - A mutable reference to the PeekableCharTracker iterator.
/// * `prefix` - An optional prefix character.
///
/// # Returns
///
/// A Result containing the collected string or an error.
///
pub fn collect_string(iter: &mut PeekableCharTracker, prefix: Option<char>) -> Result<String> {
    let mut string = String::new();

    // Handle prefix character if present
    if let Some(prefix) = prefix {
        match prefix {
            'u' | 'U' | 'b' | 'B' | 'r' | 'R' => string.push(prefix),
            prefix => {
                return Err(TokenizeError::String(
                    format!("Invalid prefix: {:?}", prefix),
                    iter.pos(),
                ))
            }
        }
    }

    // Push the starting quote character
    let quot = iter.next().unwrap();
    string.push(quot);

    // Check if it's a multiline string
    let multi_line = {
        let mut iter_clone = iter.clone();
        iter_clone.next() == Some(quot) && iter_clone.next() == Some(quot)
    };

    // Handle multi line string quotes
    if multi_line {
        string.push(iter.next().unwrap());
        string.push(iter.next().unwrap());
    }

    // Iterate over characters in the string
    while let Some(c) = iter.peek() {
        match c {
            '\n' if !multi_line => {
                return Err(TokenizeError::String(
                    "Unterminated string literal".to_owned(),
                    iter.pos(),
                ))
            }
            c => {
                // Check for closing quote
                if *c == quot {
                    string.push(iter.next().unwrap()); // Handle quote
                    if multi_line {
                        // Check for closing quotes in multiline string
                        match (iter.next(), iter.next()) {
                            (Some(second), Some(third)) if second == third && third == quot => {
                                string.push_str(&format!("{}{}", second, third))
                            }
                            _ => {
                                return Err(TokenizeError::EndOfFile(
                                    "EOF in multi-line string".to_owned(),
                                    iter.pos(),
                                ))
                            }
                        }
                    }
                    break;
                }
                string.push(*c);
            }
        }
        iter.next();
    }

    // Check for unclosed string
    if string.chars().filter(|c| *c == quot).count() < 2 {
        return Err(TokenizeError::String(
            "Unterminated string literal".to_owned(),
            iter.pos(),
        ));
    }

    Ok(string)
}
