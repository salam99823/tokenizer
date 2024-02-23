use crate::{privatestructs::ModPeekable, Result, TokenizeError};

/// Method to collect string as Python tokenizer
pub fn collect_string(iter: &mut ModPeekable, c: Option<char>) -> Result<String> {
    let mut string = String::new();

    let quot = iter.next().unwrap();
    if let Some(c) = c {
        match c {
            'f' | 'u' | 'b' | 'r' => string.push(c),
            c => {
                return Err(TokenizeError::String(
                    format!("Invalid prefix: {:?}", c),
                    *iter.pos(),
                ))
            }
        }
    }
    string.push(quot);
    while let Some(c) = iter.peek() {
        match c {
            '\\' => {
                if !string.starts_with('r') {
                    if let Some(c) = &iter.next() {
                        match *c {
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            '\'' => string.push('\''),
                            'n' => string.push('\n'),
                            'r' => string.push('\r'),
                            't' => string.push('\t'),
                            'b' => string.push('\x08'),
                            'f' => string.push('\x0C'),
                            'v' => string.push('\x0D'),
                            'a' => string.push('\x07'),
                            '\n' => {
                                continue;
                            }
                            c => {
                                let msg = format!("\\{}", c);
                                return Err(TokenizeError::EscapeSeq(
                                    format!("Unexpected escape sequence: {:?}", msg),
                                    *iter.pos(),
                                ));
                            }
                        }
                    } else {
                        return Err(TokenizeError::EndOfFile(
                            "Unexpected EndOfFile".to_owned(),
                            *iter.pos(),
                        ));
                    }
                } else {
                    string.push('\\');
                }
            }
            '\n' => {
                return Err(TokenizeError::String(
                    "Not cloused string".to_owned(),
                    *iter.pos(),
                ))
            }
            c => {
                if *c == quot {
                    string.push(*c);

                    iter.next();
                    break;
                }
                string.push(*c);
            }
        }

        iter.next();
    }
    if string.chars().filter(|c| *c == quot).count() < 2 {
        return Err(TokenizeError::String(
            "Not cloused string".to_owned(),
            *iter.pos(),
        ));
    }
    Ok(string)
}
