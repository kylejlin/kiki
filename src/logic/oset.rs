#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Oset<T> {
    raw: Vec<T>,
}

impl<T> Oset<T> {
    pub fn new() -> Self {
        Self { raw: Vec::new() }
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
        raw.sort();
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
