use crate::{privatestructs::ModPeekable, Result, TokenizeError};

/// Method to collect number as Python tokenizer
pub fn collect_number(iter: &mut ModPeekable) -> Result<String> {
    let mut number = String::new();
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
                            *iter.pos(),
                        ))
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
                                    *iter.pos(),
                                ))
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
