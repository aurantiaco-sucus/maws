
pub trait ByteSliceExt {
    fn find_pattern_naive(&self, pat: &[u8]) -> Option<usize>;
    fn find_pattern_all_naive<'a>(&'a self, pat: &'a [u8]) -> ByteSliceFindPatternAllNaiveIterator<'a>;
}

impl ByteSliceExt for [u8] {
    fn find_pattern_naive(&self, pat: &[u8]) -> Option<usize> {
        if pat.is_empty() {
            return Some(0);
        }
        self.windows(pat.len()).position(|x| x == pat)
    }

    fn find_pattern_all_naive<'a>(&'a self, pat: &'a [u8]) -> ByteSliceFindPatternAllNaiveIterator<'a> {
        ByteSliceFindPatternAllNaiveIterator {
            slice: self,
            pat,
        }
    }
}

pub struct ByteSliceFindPatternAllNaiveIterator<'a> {
    slice: &'a [u8],
    pat: &'a [u8],
}

impl <'a> Iterator for ByteSliceFindPatternAllNaiveIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None
        }
        let i = self.slice.windows(self.pat.len())
            .position(|x| x == self.pat)
            .unwrap_or(self.slice.len());
        let ret = &self.slice[..i];
        if i + self.pat.len() < self.slice.len() {
            self.slice = &self.slice[(i + self.pat.len())..];
        } else {
            self.slice = &self.slice[..0];
        }
        Some(ret)
    }
}

pub trait ByteSliceRefExt {
    fn take_until_dropped_pattern(&mut self, pat: &[u8]) -> Option<&[u8]>;
}

impl ByteSliceRefExt for &[u8] {
    fn take_until_dropped_pattern(&mut self, pat: &[u8]) -> Option<&[u8]> {
        if let Some(i) = self.find_pattern_naive(pat) {
            let cut = &self[..i];
            *self = &self[(i + pat.len())..];
            Some(cut)
        } else {
            None
        }
    }
}