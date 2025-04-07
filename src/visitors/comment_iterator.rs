pub struct CommentIterator<'c> {
    comment: &'c [u8],
    #[cfg(feature = "memchr")]
    spaces: memchr::Memchr<'c>,
    #[cfg(feature = "memchr")]
    brackets: std::iter::Zip<memchr::Memchr<'c>, memchr::Memchr<'c>>,
    #[cfg(not(feature = "memchr"))]
    characters: std::iter::Enumerate<std::slice::Iter<'c, u8>>,
}

impl<'c> CommentIterator<'c> {
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
