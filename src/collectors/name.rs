use crate::{privat::PeekableCharTracker, OPERATORS};

/// Collects a name from the given iterator.
///
/// The name is collected until a whitespace or an operator is encountered.
/// If a prefix is provided, it is added to the beginning of the name.
///
/// # Arguments
///
/// * `iter` - A mutable reference to a `PeekableCharTracker` iterator.
/// * `prefix` - An optional prefix character.
///
/// # Returns
///
/// * `String` - The collected name.
pub fn collect_name(iter: &mut PeekableCharTracker, prefix: Option<char>) -> String {
    let mut name = String::new();

    // If a prefix is provided, add it to the beginning of the name.
    if let Some(prefix) = prefix {
        name.push(prefix);
    }

    // Collect characters from the iterator until a whitespace or an operator is encountered.
    while let Some(c) = iter.next_if(|c| !c.is_whitespace() && !OPERATORS.contains(*c)) {
        name.push(c);
    }

    name
}