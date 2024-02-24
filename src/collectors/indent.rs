use crate::PeekableCharTracker;

/// Collects an indent from the input iterator.
///
/// # Arguments
///
/// * `iter` - A mutable reference to a `PeekableCharTracker` instance.
///
/// # Returns
///
/// * `String` - A `String` containing the collected indent.
///
pub fn collect_indent(iter: &mut PeekableCharTracker) -> String {
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
