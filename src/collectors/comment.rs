use crate::privat::ModPeekable;

/// Method to collect comment as Python tokenizer
pub fn collect_comment(iter: &mut ModPeekable) -> String {
    let mut comment = String::new();
    while let Some(c) = iter.next_if(|c| *c != '\n') {
        comment.push(c);
    }
    comment
}