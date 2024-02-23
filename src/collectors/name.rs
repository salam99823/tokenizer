use crate::{privat::ModPeekable, OPERATORS};

/// Method to collect names as Python tokenizer
pub fn collect_name(iter: &mut ModPeekable, prefix: Option<char>) -> String {
    let mut name = String::new();
    if let Some(prefix) = prefix {
        name.push(prefix);
    }
    while let Some(c) = iter.next_if(|c| !c.is_whitespace() && !OPERATORS.contains(*c)) {
        name.push(c);
    }
    name
}