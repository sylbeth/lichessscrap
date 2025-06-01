//! Iterator for comments on moves of a PGN file.

use std::iter::Zip;

use memchr::{memchr_iter, Memchr};

/// Iterator for comments on moves.
pub struct CommentIterator<'c> {
    /// The comment to iterate over.
    comment: &'c [u8],
    /// A memchr iterator for spaces in the comment.
    spaces: Memchr<'c>,
    /// A memchr iterator for brackets in the comment.
    brackets: Zip<Memchr<'c>, Memchr<'c>>,
}

impl<'c> CommentIterator<'c> {
    /// Creates a new comment iterator for the given comment.
    pub fn new(comment: &'c [u8]) -> Self {
        Self {
            comment,
            spaces: memchr_iter(b' ', comment),
            brackets: memchr_iter(b'[', comment).zip(memchr::memchr_iter(b']', comment)),
        }
    }
}

impl<'c> Iterator for CommentIterator<'c> {
    type Item = (&'c [u8], &'c [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start, end)) = self.brackets.next() {
            while let Some(sep) = self.spaces.next() {
                if (start..end).contains(&sep) {
                    return Some((&self.comment[start + 1..sep], &self.comment[sep + 1..end]));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::CommentIterator;

    /// Tests whether the comment iterator actually works or not.
    #[test]
    pub fn comment_iterator_test() {
        let mut comment_iter = CommentIterator::new(b" [%eval 0.17] [%clk 0:00:30] ");
        assert_eq!(
            comment_iter.next(),
            Some((b"%eval".as_slice(), b"0.17".as_slice()))
        );
        assert_eq!(
            comment_iter.next(),
            Some((b"%clk".as_slice(), b"0:00:30".as_slice()))
        );
        assert_eq!(comment_iter.next(), None);
    }
}
