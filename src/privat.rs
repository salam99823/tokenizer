use std::iter::Peekable;
use std::str::Chars;

pub struct ModPeekable<'a> {
    iter: Peekable<Chars<'a>>,
    pos: (usize, usize),
}

impl<'a> ModPeekable<'a> {
    #[inline]
    pub const fn new(iter: Peekable<Chars<'a>>) -> Self {
        ModPeekable { iter, pos: (1, 1) }
    }
    #[inline]
    pub fn pos(&self) -> &(usize, usize) {
        &self.pos
    }
    #[inline]
    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }
    #[inline]
    pub fn next_if(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        let c = self.iter.next_if(func);
        self.chek_br(c)
    }
    #[inline]
    fn chek_br(&mut self, c: Option<char>) -> Option<char> {
        match c {
            Some('\n') => {
                self.pos.0 += 1;
                self.pos.1 = 1;
                Some('\n')
            }
            other => {
                self.pos.1 += 1;
                other
            }
        }
    }
    #[inline]
    pub fn is_start_of_line(&self) -> bool {
        self.pos.1 == 1
    }
}

impl Iterator for ModPeekable<'_> {
    type Item = char;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.iter.next();
        self.chek_br(c)
    }
}

impl Clone for ModPeekable<'_> {
    #[inline]
    fn clone(&self) -> Self {
        ModPeekable {
            iter: self.iter.clone(),
            pos: self.pos,
        }
    }
}
