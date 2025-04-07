pub struct CommentIterator<'c> {
    comment: &'c [u8],
    spaces: memchr::Memchr<'c>,
    brackets: std::iter::Zip<memchr::Memchr<'c>, memchr::Memchr<'c>>,
}

impl<'c> CommentIterator<'c> {
    pub fn new(comment: &'c [u8]) -> Self {
        Self {
            comment,
            spaces: memchr::memchr_iter(b' ', comment),
            brackets: memchr::memchr_iter(b'[', comment).zip(memchr::memchr_iter(b']', comment)),
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
