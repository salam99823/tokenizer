use std::iter::Peekable;
use std::str::Chars;

pub struct ModPeekable<'a> {
    iter: Peekable<Chars<'a>>,
    pos: (usize, usize),
}

impl<'a> ModPeekable<'a> {
    #[inline]
    pub const fn new(iter: Peekable<Chars<'a>>) -> Self {
        ModPeekable { iter, pos: (0, 0) }
    }
    #[inline]
    pub fn pos(&self) -> &(usize, usize) {
        &self.pos
    }
    #[inline]
    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }
    pub fn next_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        match self.iter.next_if(func) {
            Some('\n') => {
                self.pos.0 += 1;
                self.pos.1 = 0;
                Some('\n')
            }
            other => {
                self.pos.1 += 1;
                other
            }
        }
    }
}

impl Iterator for ModPeekable<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some('\n') => {
                self.pos.0 += 1;
                self.pos.1 = 0;
                Some('\n')
            }
            other => {
                self.pos.1 += 1;
                other
            }
        }
    }
}
