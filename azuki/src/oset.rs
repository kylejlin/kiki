use crate::internal_prelude::*;

/// An ordered set.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Oset<T> {
    raw: Vec<T>,
}

impl<T> Oset<T> {
    pub fn new() -> Self {
        Self { raw: Vec::new() }
    }
}

impl<T> Default for Oset<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Oset<T>
where
    T: Ord,
{
    pub fn insert(&mut self, item: T) {
        match self.raw.binary_search(&item) {
            Ok(_) => {}
            Err(i) => self.raw.insert(i, item),
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        self.raw.binary_search(item).is_ok()
    }

    pub fn extend_with_oset(&mut self, other: Self) {
        if other.raw.is_empty() {
            return;
        }
        if self.raw.is_empty() {
            self.raw = other.raw;
            return;
        }

        let mut raw = Vec::with_capacity(self.raw.len() + other.raw.len());
        let mut left = std::mem::take(&mut self.raw).into_iter().peekable();
        let mut right = other.raw.into_iter().peekable();

        while let (Some(left_item), Some(right_item)) = (left.peek(), right.peek()) {
            match left_item.cmp(right_item) {
                Ordering::Less => raw.push(left.next().unwrap()),
                Ordering::Equal => {
                    raw.push(left.next().unwrap());
                    right.next();
                }
                Ordering::Greater => raw.push(right.next().unwrap()),
            }
        }

        raw.extend(left);
        raw.extend(right);
        self.raw = raw;
    }
}

impl<T> FromIterator<T> for Oset<T>
where
    T: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut raw: Vec<T> = iter.into_iter().collect();
        raw.sort_unstable();
        raw.dedup();
        Self { raw }
    }
}

impl<T> IntoIterator for Oset<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Oset<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter()
    }
}

impl<T> Deref for Oset<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<T> Extend<T> for Oset<T>
where
    T: Ord,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.raw.extend(iter);
        self.raw.sort_unstable();
        self.raw.dedup();
    }
}
