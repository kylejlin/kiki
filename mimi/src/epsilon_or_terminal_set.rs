use crate::internal_prelude::*;

pub struct EpsilonOrTerminalSet {
    buf: EpsilonOrTerminalSetBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EpsilonOrTerminalSingletonSet {
    pub value: EpsilonOrTerminal,
}

impl EpsilonOrTerminalSet {
    pub fn empty() -> Self {
        Self {
            buf: EpsilonOrTerminalSetBuf::empty(),
        }
    }

    pub fn contains(&self, needle: EpsilonOrTerminal) -> bool {
        self.buf.contains(needle)
    }
}

impl BitOrAssign<&Self> for EpsilonOrTerminalSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.buf |= &rhs.buf;
    }
}

impl BitOrAssign<EpsilonOrTerminalSingletonSet> for EpsilonOrTerminalSet {
    fn bitor_assign(&mut self, rhs: EpsilonOrTerminalSingletonSet) {
        self.buf |= rhs;
    }
}

impl From<EpsilonOrTerminalSingletonSet> for EpsilonOrTerminalSet {
    fn from(value: EpsilonOrTerminalSingletonSet) -> Self {
        let mut out = EpsilonOrTerminalSet::empty();
        out |= value;
        out
    }
}

enum EpsilonOrTerminalSetBuf {
    Small([usize; Self::MIN_CHUNK_COUNT]),
    Big(Vec<usize>),
}

impl EpsilonOrTerminalSetBuf {
    /// Thanks for `Vec`'s non-zero niche, we get two words for free.
    const MIN_CHUNK_COUNT: usize = 2;

    const CHUNK_WIDTH: usize = usize::BITS as usize;

    fn empty() -> Self {
        Self::Small([0; _])
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

    fn contains(&self, needle: EpsilonOrTerminal) -> bool {
        let i = needle.as_usize();

        let chunk_index = i / Self::CHUNK_WIDTH;
        let Some(chunk) = self.as_slice().get(chunk_index) else {
            return false;
        };

        let mask: usize = 1usize << (i % Self::CHUNK_WIDTH);

        (chunk & mask) != 0
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

impl BitOrAssign<&Self> for EpsilonOrTerminalSetBuf {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.grow_if_needed(rhs.chunk_count());

        for (l, r) in self.as_mut().into_iter().zip(rhs.as_slice()) {
            *l |= *r;
        }
    }
}

impl BitOrAssign<EpsilonOrTerminalSingletonSet> for EpsilonOrTerminalSetBuf {
    fn bitor_assign(&mut self, rhs: EpsilonOrTerminalSingletonSet) {
        let i = rhs.value.as_usize();

        let chunk_index = i / Self::CHUNK_WIDTH;
        self.grow_if_needed(chunk_index + 1);
        let chunk = &mut self.as_mut()[chunk_index];

        let mask: usize = 1usize << (i % Self::CHUNK_WIDTH);

        *chunk |= mask;
    }
}
