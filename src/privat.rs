use std::iter::Peekable;
use std::str::Chars;

/// A struct that wraps a Peekable iterator over Chars with position tracking.
pub struct PeekableCharTracker<'a> {
    iter: Peekable<Chars<'a>>,
    pos: (usize, usize),
}

impl<'a> PeekableCharTracker<'a> {
    /// Creates a new PeekableCharTracker instance.
    #[inline]
    pub const fn new(iter: Peekable<Chars<'a>>) -> Self {
        PeekableCharTracker { iter, pos: (1, 1) }
    }

    /// Returns a current position.
    #[inline]
    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    /// Peeks at the next character without advancing the iterator.
    #[inline]
    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    /// Advances the iterator if the next character satisfies the given condition.
    #[inline]
    pub fn next_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        let c = self.iter.next_if(func);
        self.check_newline(c)
    }

    /// Helper function to handle newline characters and update position.
    #[inline]
    fn check_newline(&mut self, c: Option<char>) -> Option<char> {
        match c {
            Some('\n') => {
                self.pos.0 += 1;
                self.pos.1 = 1;
                Some('\n')
            }
            Some(other) => {
                self.pos.1 += 1;
                Some(other)
            }
            None => None
        }
    }

    /// Checks if the current position is at the start of a new line.
    #[inline]
    pub fn is_start_of_line(&self) -> bool {
        self.pos.1 == 1
    }
}

impl Iterator for PeekableCharTracker<'_> {
    type Item = char;

    /// Advances the iterator and returns the next character.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.iter.next();
        self.check_newline(c)
    }
}

impl Clone for PeekableCharTracker<'_> {
    /// Clones the PeekableCharTracker instance.
    #[inline]
    fn clone(&self) -> Self {
        PeekableCharTracker {
            iter: self.iter.clone(),
            pos: self.pos,
        }
    }
}