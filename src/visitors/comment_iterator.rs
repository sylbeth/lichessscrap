//! Iterator for comments on moves of a PGN file.

/// Iterator for comments on moves.
pub struct CommentIterator<'c> {
    /// The comment to iterate over.
    comment: &'c [u8],
    /// A memchr iterator for spaces in the comment.
    #[cfg(feature = "memchr")]
    spaces: memchr::Memchr<'c>,
    /// A memchr iterator for brackets in the comment.
    #[cfg(feature = "memchr")]
    brackets: std::iter::Zip<memchr::Memchr<'c>, memchr::Memchr<'c>>,
    /// A character iterator for the characters in the comment.
    #[cfg(not(feature = "memchr"))]
    characters: std::iter::Enumerate<std::slice::Iter<'c, u8>>,
}

impl<'c> CommentIterator<'c> {
    /// Creates a new comment iterator for the given comment.
    pub fn new(comment: &'c [u8]) -> Self {
        Self {
            comment,
            #[cfg(feature = "memchr")]
            spaces: memchr::memchr_iter(b' ', comment),
            #[cfg(feature = "memchr")]
            brackets: memchr::memchr_iter(b'[', comment).zip(memchr::memchr_iter(b']', comment)),
            #[cfg(not(feature = "memchr"))]
            characters: comment.iter().enumerate(),
        }
    }
}

impl<'c> Iterator for CommentIterator<'c> {
    type Item = (&'c [u8], &'c [u8]);

    #[cfg(feature = "memchr")]
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

    #[cfg(not(feature = "memchr"))]
    fn next(&mut self) -> Option<Self::Item> {
        let (mut start, mut sep) = (0, 0);
        while let Some((i, c)) = self.characters.next() {
            match c {
                b' ' => sep = i,
                b'[' => start = i + 1,
                b']' => return Some((&self.comment[start..sep], &self.comment[sep + 1..i])),
                _ => (),
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::CommentIterator;

    #[test]
    /// Tests whether the comment iterator actually works or not.
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
