use crate::{privat::PeekableCharTracker, Result, TokenizeError, OPERATORS};

/// Collects an operator from the input iterator.
///
/// # Arguments
///
/// * `iter` - A mutable reference to a `PeekableCharTracker` instance.
/// * `operator` - The current operator character.
///
/// # Returns
///
/// * `Result<String>` - A `Result` containing the collected operator as a `String`.
///
/// # Errors
///
/// * `TokenizeError::Operator` - If the operator is invalid.
pub fn collect_operator(iter: &mut PeekableCharTracker, operator: char) -> Result<String> {
    // Check if the operator is valid.
    if !OPERATORS.contains(operator) {
        // If not, return an error.
        Err(TokenizeError::Operator(
            format!("Invalid operator: {:?}", operator),
            iter.pos(),
        ))
    } else {
        // If the operator is valid, proceed to the next step.
        Ok(match (operator, iter.peek()) {
            // If the next character is '=', and the current operator is one of "+-*/%&|<>!^:@",
            // then collect the operator and the '=' character.
            (c, Some('=')) if "=+-*/%&|<>!^:@".contains(c) => {
                iter.next();
                format!("{}=", c)
            }
            // If the current operator is '-', and the next character is '>',
            // then collect the '->' operator.
            ('-', Some('>')) => {
                iter.next();
                "->".to_owned()
            }
            // If the current operator is '<', and the next character is '>',
            // then collect the '<>' operator.
            ('<', Some('>')) => {
                iter.next();
                "<>".to_owned()
            }
            // If the current operator is the same as the next character, and the current operator is one of "*/<>",
            // then collect the operator and the next character.
            (c1, Some(c2)) if c1 == *c2 && "*/<>".contains(c1) => {
                let c2 = *c2;
                iter.next();
                match iter.peek() {
                    // If the next character is '=', then collect the operator and the '=' character.
                    Some('=') => {
                        iter.next();
                        format!("{}{}=", c1, c2)
                    }
                    // Otherwise, just collect the operator and the next character.
                    _ => format!("{}{}", c1, c2),
                }
            }
            // If none of the above conditions are met, then just collect the current operator.
            (c, _) => c.to_string(),
        })
    }
}
