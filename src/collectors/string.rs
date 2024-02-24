use crate::{privat::ModPeekable, Result, TokenizeError};

/// Method to collect string as Python tokenizer
pub fn collect_string(iter: &mut ModPeekable, prefix: Option<char>) -> Result<String> {
    let mut string = String::new();

    let quot = iter.next().unwrap();

    if let Some(prefix) = prefix {
        match prefix {
            'u' | 'b' | 'r' => string.push(prefix),
            prefix => {
                return Err(TokenizeError::String(
                    format!("Invalid prefix: {:?}", prefix),
                    *iter.pos(),
                ))
            }
        }
    }
    string.push(quot);

    let multi_line = {
        let mut iter_clone = iter.clone();
        iter_clone.next() == Some(quot) && iter_clone.next() == Some(quot)
    };
    if multi_line {
        string.push(iter.next().unwrap());
        string.push(iter.next().unwrap());
    }
    while let Some(c) = iter.peek() {
        match c {
            '\\' if !string.starts_with('r') => {
                let i = iter.next();
                println!("{:?}", i);
                if let Some(c) = &i {
                    match *c {
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        'n' => string.push('\n'),
                        'r' => string.push('\r'),
                        't' => string.push('\t'),
                        'b' => string.push('\u{0008}'),
                        'v' => string.push('\u{000B}'),
                        'f' => string.push('\u{000C}'),
                        'a' => string.push('\u{0007}'),
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
            }
            '\\' => string.push('\\'),
            '\n' if !multi_line => {
                return Err(TokenizeError::String(
                    "Not cloused string".to_owned(),
                    *iter.pos(),
                ))
            }
            c => {
                if *c == quot {
                    string.push(iter.next().unwrap());
                    if multi_line {
                        string.push(iter.next().unwrap());
                        string.push(iter.next().unwrap());
                    }
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
