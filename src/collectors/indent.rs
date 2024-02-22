use crate::ModPeekable;

/// Method to collect padding as Python tokenizer
pub fn collect_indent(iter: &mut ModPeekable) -> String {
    let mut new_indent = String::new();
    while let Some(c2) = iter.peek() {
        match c2 {
            '\t' => new_indent.push('\t'),
            ' ' => new_indent.push(' '),
            _ => break,
        }

        iter.next();
    }
    new_indent
}
