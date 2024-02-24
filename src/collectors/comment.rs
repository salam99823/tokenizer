use crate::privat::PeekableCharTracker;

/// Collects a comment from the given iterator.
///
/// The comment is terminated by a newline character.
///
/// # Arguments
///
/// * `iter` - A mutable reference to a `PeekableCharTracker` iterator.
///
/// # Returns
///
/// * `String` - The collected comment.
pub fn collect_comment(iter: &mut PeekableCharTracker) -> String {
    let mut comment = String::new();

    // Iterate over the characters in the iterator until a newline character is encountered.
    while let Some(c) = iter.next_if(|c| *c != '\n') {
        // Add the current character to the comment string.
        comment.push(c);
    }

    // Return the collected comment.
    comment
}