use crate::internal_prelude::*;

pub struct RawBitSet {
    buf: RawBitSetBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawSingletonBitSet {
    pub value: usize,
}

impl RawBitSet {
    pub fn empty() -> Self {
        Self {
            buf: RawBitSetBuf::empty(),
        }
    }

    pub fn cardinality(&self) -> usize {
        self.buf.cardinality()
    }

    pub fn contains(&self, needle: usize) -> bool {
        self.buf.contains(needle)
    }

    pub fn clear(&mut self) {
        self.buf.clear()
    }
}

impl BitOrAssign<&Self> for RawBitSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.buf |= &rhs.buf;
    }
}

impl BitOrAssign<RawSingletonBitSet> for RawBitSet {
    fn bitor_assign(&mut self, rhs: RawSingletonBitSet) {
        self.buf |= rhs;
    }
}

impl SubAssign<&Self> for RawBitSet {
    fn sub_assign(&mut self, rhs: &Self) {
        self.buf -= &rhs.buf;
    }
}

impl SubAssign<RawSingletonBitSet> for RawBitSet {
    fn sub_assign(&mut self, rhs: RawSingletonBitSet) {
        self.buf -= rhs;
    }
}

impl IntoIterator for RawBitSet {
    type Item = usize;
    type IntoIter = RawBitSetBufIter;

    fn into_iter(self) -> Self::IntoIter {
        self.buf.into_iter()
    }
}

impl<'a> IntoIterator for &'a RawBitSet {
    type Item = usize;
    type IntoIter = RawBitSetBufRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.buf).into_iter()
    }
}

impl<'a> RawBitSet {
    pub fn iter(&'a self) -> <&'a Self as IntoIterator>::IntoIter {
        (&self.buf).into_iter()
    }
}

impl From<RawSingletonBitSet> for RawBitSet {
    fn from(value: RawSingletonBitSet) -> Self {
        let mut out = RawBitSet::empty();
        out |= value;
        out
    }
}

enum RawBitSetBuf {
    Small([usize; Self::MIN_CHUNK_COUNT]),
    Big(Vec<usize>),
}

impl RawBitSetBuf {
    /// Thanks for `Vec`'s non-zero niche, we get two words for free.
    const MIN_CHUNK_COUNT: usize = 2;

    const CHUNK_WIDTH: usize = usize::BITS as usize;

    fn empty() -> Self {
        Self::Small([0; _])
    }

    fn cardinality(&self) -> usize {
        self.as_slice()
            .iter()
            .map(|chunk| chunk.count_ones() as usize)
            .sum()
    }

    fn as_slice(&self) -> &[usize] {
        match self {
            Self::Small(buf) => &*buf,
            Self::Big(buf) => &buf,
        }
    }

    fn as_mut(&mut self) -> &mut [usize] {
        match self {
            Self::Small(buf) => &mut *buf,
            Self::Big(buf) => &mut *buf,
        }
    }

    fn chunk_count(&self) -> usize {
        self.as_slice().len()
    }

    fn contains(&self, needle: usize) -> bool {
        let i = needle;

        let chunk_index = i / Self::CHUNK_WIDTH;
        let Some(chunk) = self.as_slice().get(chunk_index) else {
            return false;
        };

        let mask: usize = 1usize << (i % Self::CHUNK_WIDTH);

        (chunk & mask) != 0
    }

    fn clear(&mut self) {
        match self {
            Self::Small(buf) => *buf = [0; _],

            Self::Big(buf) => {
                buf.clear();
                buf.extend(iter::repeat_n(0, Self::MIN_CHUNK_COUNT));
            }
        }
    }

    fn grow_if_needed(&mut self, new_min_chunk_count: usize) {
        let additional_chunk_count = new_min_chunk_count.saturating_sub(self.chunk_count());

        if additional_chunk_count == 0 {
            return;
        }

        let extra_chunks = iter::repeat_n(0, additional_chunk_count);

        match self {
            Self::Small(buf) => {
                *self = Self::Big(buf.iter().copied().chain(extra_chunks).collect());
            }

            Self::Big(buf) => buf.extend(extra_chunks),
        }
    }
}

impl BitOrAssign<&Self> for RawBitSetBuf {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.grow_if_needed(rhs.chunk_count());

        for (l, r) in self.as_mut().into_iter().zip(rhs.as_slice()) {
            *l |= *r;
        }
    }
}

impl BitOrAssign<RawSingletonBitSet> for RawBitSetBuf {
    fn bitor_assign(&mut self, rhs: RawSingletonBitSet) {
        let i = rhs.value;

        let chunk_index = i / Self::CHUNK_WIDTH;
        self.grow_if_needed(chunk_index + 1);
        let chunk = &mut self.as_mut()[chunk_index];

        let mask: usize = 1usize << (i % Self::CHUNK_WIDTH);

        *chunk |= mask;
    }
}

impl SubAssign<&Self> for RawBitSetBuf {
    fn sub_assign(&mut self, rhs: &Self) {
        for (l, r) in self.as_mut().into_iter().zip(rhs.as_slice()) {
            *l &= !*r;
        }
    }
}

impl SubAssign<RawSingletonBitSet> for RawBitSetBuf {
    fn sub_assign(&mut self, rhs: RawSingletonBitSet) {
        let i = rhs.value;

        let chunk_index = i / Self::CHUNK_WIDTH;
        let Some(chunk) = self.as_mut().get_mut(chunk_index) else {
            return;
        };

        let mask: usize = 1usize << (i % Self::CHUNK_WIDTH);

        *chunk &= !mask;
    }
}

impl IntoIterator for RawBitSetBuf {
    type Item = usize;
    type IntoIter = RawBitSetBufIter;

    fn into_iter(self) -> Self::IntoIter {
        RawBitSetBufIter {
            buf: self,
            chunk_index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a RawBitSetBuf {
    type Item = usize;
    type IntoIter = RawBitSetBufRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let buf = self.as_slice();
        let chunk = buf[0];
        RawBitSetBufRefIter {
            buf,
            chunk,
            chunk_index: 0,
        }
    }
}

pub struct RawBitSetBufIter {
    buf: RawBitSetBuf,
    chunk_index: usize,
}

impl Iterator for RawBitSetBufIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.buf.as_mut();
        loop {
            let chunk = s.get_mut(self.chunk_index)?;
            if *chunk == 0 {
                self.chunk_index += 1;
                continue;
            }

            let i =
                self.chunk_index * RawBitSetBuf::CHUNK_WIDTH + (chunk.trailing_zeros() as usize);
            *chunk &= *chunk - 1;
            break Some(i);
        }
    }
}

pub struct RawBitSetBufRefIter<'a> {
    buf: &'a [usize],
    chunk: usize,
    chunk_index: usize,
}

impl Iterator for RawBitSetBufRefIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.chunk_index >= self.buf.len() {
                return None;
            }

            if self.chunk == 0 {
                self.chunk_index += 1;
                self.chunk = *self.buf.get(self.chunk_index)?;
                continue;
            }

            let i = self.chunk_index * RawBitSetBuf::CHUNK_WIDTH
                + (self.chunk.trailing_zeros() as usize);
            self.chunk &= self.chunk - 1;
            break Some(i);
        }
    }
}
