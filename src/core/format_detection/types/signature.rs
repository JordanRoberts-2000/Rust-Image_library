#[derive(Clone, Copy)]
pub struct Signature {
    pub pattern: &'static [u8],
    pub mask: Option<&'static [u8]>,
    pub offset: usize,
}

impl Signature {
    #[inline]
    pub fn matches(&self, bytes: &[u8]) -> bool {
        let start = self.offset;
        let end = start + self.pattern.len();

        if bytes.len() < end {
            return false;
        }

        let win = &bytes[start..end];
        match self.mask {
            None => win == self.pattern,
            Some(m) => {
                win.iter().zip(m).zip(self.pattern).all(|((&b, &mm), &p)| (b & mm) == (p & mm))
            }
        }
    }
}
