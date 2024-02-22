use crate::{privatestructs::ModPeekable, OPERATORS};

/// Method to collect names as Python tokenizer
pub fn collect_name(iter: &mut ModPeekable, c: Option<char>) -> String {
    let mut name = String::new();
    if let Some(c) = c {
        name.push(c);
    }
    while let Some(c) = iter.next_if(|c| !c.is_whitespace() && !OPERATORS.contains(*c)) {
        name.push(c);
    }
    name
}