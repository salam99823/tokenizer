use crate::{privat::ModPeekable, Result, TokenizeError, OPERATORS};

/// Method to collect operators as Python tokenizer
pub fn collect_operator(iter: &mut ModPeekable, operator: char) -> Result<String> {
    if !OPERATORS.contains(operator) {
        Err(TokenizeError::Operator(
            format!("Invalid operator: {:?}", operator),
            *iter.pos(),
        ))
    } else {
        Ok(match (operator, iter.peek()) {
            (c, Some('=')) if "=+-*/%&|<>!^:@".contains(c) => {
                iter.next();
                format!("{}=", c)
            }
            ('-', Some('>')) => {
                iter.next();
                "->".to_owned()
            }
            ('<', Some('>')) => {
                iter.next();
                "<>".to_owned()
            }
            (c1, Some(c2)) if c1 == *c2 && "*/<>".contains(c1) => {
                let c2 = *c2;
                iter.next();
                match iter.peek() {
                    Some('=') => {
                        iter.next();
                        format!("{}{}=", c1, c2)
                    }
                    _ => format!("{}{}", c1, c2),
                }
            }
            (c, _) => c.to_string(),
        })
    }
}
