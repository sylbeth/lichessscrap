use std::iter::Zip;

use memchr::{Memchr, memchr_iter};

pub struct CommentIterator<'c> {
    comment: &'c [u8],
    spaces: Memchr<'c>,
    brackets: Zip<Memchr<'c>, Memchr<'c>>,
}

impl<'c> CommentIterator<'c> {
    pub fn new(comment: &'c [u8]) -> Self {
        Self {
            comment,
            spaces: memchr_iter(b' ', comment),
            brackets: memchr_iter(b'[', comment).zip(memchr_iter(b']', comment)),
        }
    }
}

impl<'c> Iterator for CommentIterator<'c> {
    type Item = (&'c [u8], &'c [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start, end)) = self.brackets.next() {
            let key: &[u8];
            while let Some(sep) = self.spaces.next() {
                if (start..end).contains(&sep) {
                    if self.comment[start + 1] == b'%' {
                        key = &self.comment[start + 2..sep];
                    } else {
                        key = &self.comment[start + 1..sep];
                    }
                    return Some((key, &self.comment[sep + 1..end]));
                }
            }
        }
        None
    }
}
