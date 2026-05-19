
pub trait ByteSliceExt {
    fn find_pattern_naive(&self, pat: &[u8]) -> Option<usize>;
    fn find_pattern_all_naive(&self, pat: &[u8]) -> Vec<usize>;
}

impl ByteSliceExt for [u8] {
    fn find_pattern_naive(&self, pat: &[u8]) -> Option<usize> {
        if pat.is_empty() {
            return Some(0);
        }
        self.windows(pat.len()).position(|x| x == pat)
    }

    fn find_pattern_all_naive(&self, pat: &[u8]) -> Vec<usize> {
        let mut cur = self;
        let mut pos = Vec::new();
        if pat.is_empty() {
            return pos;
        }
        while let Some(i) = cur.windows(pat.len()).position(|x| x == pat) {
            cur = &cur[(i + pat.len())..];
            pos.push(i);
        }
        pos
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