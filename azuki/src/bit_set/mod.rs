use crate::internal_prelude::*;

use raw::*;

mod raw;

pub trait UsizePun {
    fn into_usize(self) -> usize;

    fn from_usize(n: usize) -> Self;
}

pub type EpsilonOrTerminalSet = TaggedBitSet<EpsilonOrTerminal>;

pub type EpsilonOrTerminalSingletonSet = TaggedSingletonBitSet<EpsilonOrTerminal>;

pub struct TaggedBitSet<K> {
    raw: RawBitSet,
    _ntt: Ntt<K>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaggedSingletonBitSet<K> {
    pub value: K,
}

impl<K> TaggedBitSet<K>
where
    K: UsizePun,
{
    pub fn empty() -> Self {
        Self {
            raw: RawBitSet::empty(),
            _ntt: Ntt::new(),
        }
    }

    pub fn cardinality(&self) -> usize {
        self.raw.cardinality()
    }

    pub fn contains(&self, needle: K) -> bool {
        self.raw.contains(needle.into_usize())
    }

    pub fn clear(&mut self) {
        self.raw.clear()
    }
}

impl<K> BitOrAssign<&Self> for TaggedBitSet<K>
where
    K: UsizePun,
{
    fn bitor_assign(&mut self, rhs: &Self) {
        self.raw |= &rhs.raw;
    }
}

impl<K> BitOrAssign<TaggedSingletonBitSet<K>> for TaggedBitSet<K>
where
    K: UsizePun,
{
    fn bitor_assign(&mut self, rhs: TaggedSingletonBitSet<K>) {
        self.raw |= RawSingletonBitSet {
            value: rhs.value.into_usize(),
        };
    }
}

impl<K> SubAssign<&Self> for TaggedBitSet<K>
where
    K: UsizePun,
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.raw -= &rhs.raw;
    }
}

impl<K> SubAssign<TaggedSingletonBitSet<K>> for TaggedBitSet<K>
where
    K: UsizePun,
{
    fn sub_assign(&mut self, rhs: TaggedSingletonBitSet<K>) {
        self.raw -= RawSingletonBitSet {
            value: rhs.value.into_usize(),
        };
    }
}

impl<K> IntoIterator for TaggedBitSet<K>
where
    K: UsizePun,
{
    type Item = K;
    type IntoIter = TaggedBitSetIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        TaggedBitSetIter {
            raw: self.raw.into_iter(),
            _ntt: Ntt::new(),
        }
    }
}

impl<'a, K> IntoIterator for &'a TaggedBitSet<K>
where
    K: UsizePun,
{
    type Item = K;
    type IntoIter = TaggedBitSetRefIter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        TaggedBitSetRefIter {
            raw: (&self.raw).into_iter(),
            _ntt: Ntt::new(),
        }
    }
}

impl<'a, K> TaggedBitSet<K>
where
    K: UsizePun,
{
    pub fn iter(&'a self) -> <&'a Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<K> From<TaggedSingletonBitSet<K>> for TaggedBitSet<K>
where
    K: UsizePun,
{
    fn from(value: TaggedSingletonBitSet<K>) -> Self {
        let mut out = Self::empty();
        out |= value;
        out
    }
}

pub struct TaggedBitSetIter<K> {
    raw: RawBitSetBufIter,
    _ntt: Ntt<K>,
}

impl<K> Iterator for TaggedBitSetIter<K>
where
    K: UsizePun,
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(K::from_usize)
    }
}

pub struct TaggedBitSetRefIter<'a, K> {
    raw: RawBitSetBufRefIter<'a>,
    _ntt: Ntt<K>,
}

impl<K> Iterator for TaggedBitSetRefIter<'_, K>
where
    K: UsizePun,
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(K::from_usize)
    }
}
