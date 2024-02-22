use crate::{privatestructs::ModPeekable, Result, TokenizeError};

/// Method to collect operators as Python tokenizer
pub fn collect_operator(iter: &mut ModPeekable) -> Result<String> {
    Ok(match iter.next().unwrap() {
        '=' => match iter.peek() {
            Some('=') => {
                iter.next();
                "==".to_owned()
            }
            _ => "=".to_owned(),
        },
        '+' => match iter.peek() {
            Some('=') => {
                iter.next();
                "+=".to_owned()
            }
            _ => "+".to_owned(),
        },
        '-' => match iter.peek() {
            Some('=') => {
                iter.next();
                "-=".to_owned()
            }
            Some('>') => {
                iter.next();
                "->".to_owned()
            }
            _ => "-".to_owned(),
        },
        '*' => match iter.peek() {
            Some('=') => {
                iter.next();
                "*=".to_owned()
            }
            Some('*') => {
                iter.next();
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        "**=".to_owned()
                    }
                    _ => "**".to_owned(),
                }
            }
            _ => "*".to_owned(),
        },
        '/' => match iter.peek() {
            Some('=') => {
                iter.next();
                "/=".to_owned()
            }
            Some('/') => {
                iter.next();
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        "//=".to_owned()
                    }
                    _ => "//".to_owned(),
                }
            }
            _ => "/".to_owned(),
        },
        '%' => match iter.peek() {
            Some('=') => {
                iter.next();
                "%=".to_owned()
            }
            _ => "%".to_owned(),
        },
        '&' => match iter.peek() {
            Some('=') => {
                iter.next();
                "&=".to_owned()
            }
            _ => "&".to_owned(),
        },
        '|' => match iter.peek() {
            Some('=') => {
                iter.next();
                "|=".to_owned()
            }
            _ => "|".to_owned(),
        },
        '<' => match iter.peek() {
            Some('=') => {
                iter.next();
                "<=".to_owned()
            }
            Some('<') => {
                iter.next();
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        "<<=".to_owned()
                    }
                    _ => "<<".to_owned(),
                }
            }
            Some('>') => "<>".to_owned(),
            _ => "<".to_owned(),
        },
        '>' => match iter.peek() {
            Some('=') => {
                iter.next();
                ">=".to_owned()
            }
            Some('>') => {
                iter.next();
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        ">>=".to_owned()
                    }
                    _ => ">>".to_owned(),
                }
            }
            _ => ">".to_owned(),
        },
        '!' => match iter.peek() {
            Some('=') => {
                iter.next();
                "!=".to_owned()
            }
            _ => "!".to_owned(),
        },
        '^' => match iter.peek() {
            Some('=') => {
                iter.next();
                "^=".to_owned()
            }
            _ => "^".to_owned(),
        },
        ':' => match iter.peek() {
            Some('=') => {
                iter.next();
                ":=".to_owned()
            }
            _ => ":".to_owned(),
        },
        ';' => ";".to_owned(),
        '.' => ".".to_owned(),
        ',' => ",".to_owned(),
        '(' => "(".to_owned(),
        ')' => ")".to_owned(),
        '[' => "[".to_owned(),
        ']' => "]".to_owned(),
        '{' => "{".to_owned(),
        '}' => "}".to_owned(),
        '@' => match iter.peek() {
            Some('=') => {
                iter.next();
                "@=".to_owned()
            }
            _ => "@".to_owned(),
        },
        '$' => "$".to_owned(),
        '?' => "?".to_owned(),
        '~' => "~".to_owned(),
        '`' => "`".to_owned(),
        op => {
            return Err(TokenizeError::Operator(
                format!("Invalid operator: {:?}", op),
                *iter.pos(),
            ))
        }
    })
}
