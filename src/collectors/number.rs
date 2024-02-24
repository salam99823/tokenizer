use crate::{privat::PeekableCharTracker, Result, TokenizeError};

/// Collects a number as a Python tokenizer.
///
/// # Arguments
///
/// * `iter` - A mutable reference to the PeekableCharTracker iterator.
/// * `digit` - An optional starting digit character.
///
/// # Returns
///
/// A Result containing the collected number as a String or an error.
///
pub fn collect_number(iter: &mut PeekableCharTracker, digit: Option<char>) -> Result<String> {
    let mut number = String::new();

    // Add the starting digit if provided
    if let Some(d) = digit {
        number.push(d);
    }

    // Iterate over characters to collect the number
    while let Some(c) = iter.peek() {
        match c {
            '0'..='9' => {
                number.push(iter.next().unwrap());
            }
            '_' => {
                iter.next();
                match iter.peek() {
                    Some('0'..='9') => number.push('_'),
                    _ => {
                        return Err(TokenizeError::Number(
                            "Invalid decimal literal".to_owned(),
                            iter.pos(),
                        ));
                    }
                }
            }
            '.' if !number.contains('.') => {
                number.push(iter.next().unwrap());
                match iter.peek() {
                    Some('0'..='9') => continue,
                    _ => break,
                }
            }
            'j' => {
                number.push(iter.next().unwrap());
                break;
            }
            'e' if !number.contains('e') => {
                match iter.clone().nth(1) {
                    Some('-' | '+') => {
                        number.push(iter.next().unwrap());
                        number.push(iter.next().unwrap());
                        match iter.peek() {
                            Some('0'..='9') => continue,
                            _ => {
                                return Err(TokenizeError::Number(
                                    "Invalid decimal literal".to_owned(),
                                    iter.pos(),
                                ));
                            }
                        }
                    }
                    _ => break,
                }
            }
            _ => break,
        }
    }

    Ok(number)
}