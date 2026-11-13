use crate::internal_prelude::*;

pub struct EpsilonOrTerminalSet {
    buf: EpsilonOrTerminalSetBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EpsilonOrTerminalSingletonSet {
    pub value: EpsilonOrTerminal,
}

impl EpsilonOrTerminalSet {
    pub fn empty(terminal_count: usize) -> Self {
        Self {
            buf: EpsilonOrTerminalSetBuf::empty(terminal_count),
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

enum EpsilonOrTerminalSetBuf {
    Small([usize; Self::MIN_CHUNK_COUNT]),
    Big(Vec<usize>),
}

impl EpsilonOrTerminalSetBuf {
    /// Thanks for `Vec`'s non-zero niche, we get two words for free.
    const MIN_CHUNK_COUNT: usize = 2;

    const CHUNK_WIDTH: usize = usize::BITS as usize;

    fn empty(terminal_count: usize) -> Self {
        let required_min_chunk_count = (1 + terminal_count) / Self::CHUNK_WIDTH;

        if required_min_chunk_count <= Self::MIN_CHUNK_COUNT {
            Self::Small([0; _])
        } else {
            Self::Big(vec![0; required_min_chunk_count])
        }
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
        assert!(chunk_index < self.chunk_count());
        let chunk = self.as_slice()[chunk_index];

        let mask: usize = 1usize << (i % Self::CHUNK_WIDTH);

        (chunk & mask) != 0
    }
}

impl BitOrAssign<&Self> for EpsilonOrTerminalSetBuf {
    fn bitor_assign(&mut self, rhs: &Self) {
        assert_eq!(self.chunk_count(), rhs.chunk_count());
        for (l, r) in self.as_mut().into_iter().zip(rhs.as_slice()) {
            *l |= *r;
        }
    }
}

impl BitOrAssign<EpsilonOrTerminalSingletonSet> for EpsilonOrTerminalSetBuf {
    fn bitor_assign(&mut self, rhs: EpsilonOrTerminalSingletonSet) {
        let i = rhs.value.as_usize();

        let chunk_index = i / Self::CHUNK_WIDTH;
        assert!(chunk_index < self.chunk_count());
        let chunk = &mut self.as_mut()[chunk_index];

        let mask: usize = 1usize << (i % Self::CHUNK_WIDTH);

        *chunk |= mask;
    }
}

impl EpsilonOrTerminalSingletonSet {
    pub fn with_terminal_count(self, terminal_count: usize) -> EpsilonOrTerminalSet {
        let mut out = EpsilonOrTerminalSet::empty(terminal_count);
        out |= self;
        out
    }
}
