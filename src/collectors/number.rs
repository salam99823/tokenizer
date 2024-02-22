use crate::{privatestructs::ModPeekable, Result, TokenizeError};

/// Method to collect number as Python tokenizer
pub fn collect_number(iter: &mut ModPeekable) -> Result<String> {
    let mut number = String::new();
    while let Some(c) = iter.peek() {
        match c {
            '0'..='9' => {
                number.push(*c);

                iter.next();
            }
            '_' => {
                iter.next();
                match iter.peek() {
                    Some('0'..='9' | '_' | 'e' | 'j') => number.push('_'),
                    Some(c) => {
                        return Err(TokenizeError::Number(
                            format!("Invalid decimal literal: {:?}", format!("{}_{}", number, c)),
                            *iter.pos(),
                        ))
                    }
                    None => {
                        return Err(TokenizeError::Number(
                            "Invalid decimal literal".to_owned(),
                            *iter.pos(),
                        ))
                    }
                }
            }
            '.' => {
                if number.contains('.') {
                    break;
                } else {
                    iter.next();
                    number.push('.');
                }
            }
            'j' => {
                iter.next();
                number.push('j');
                break;
            }
            'e' => {
                if !number.contains('.') {
                    break;
                } else {
                    iter.next();
                    match iter.peek() {
                        Some('-' | '+') => {
                            number.push('e');
                            number.push(iter.next().unwrap())
                        }
                        _ => {
                            iter.next();
                            number.push('e');
                        }
                    }
                }
            }
            _ => break,
        }
    }
    Ok(number)
}
