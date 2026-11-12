use crate::internal_prelude::*;

/// Newtype tag.
pub struct Ntt<T> {
    _raw: PhantomData<T>,
}
impl<T> Clone for Ntt<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Ntt<T> {}
impl<T> Debug for Ntt<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Tag<{}>", std::any::type_name::<T>())
    }
}
impl<T> PartialEq for Ntt<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl<T> Eq for Ntt<T> {}
impl<T> PartialOrd for Ntt<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Ntt<T> {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}
impl<T> Default for Ntt<T> {
    fn default() -> Self {
        Self { _raw: PhantomData }
    }
}
impl<T> Hash for Ntt<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self._raw.hash(hasher)
    }
}
impl<T> Ntt<T> {
    pub const fn new() -> Self {
        Self { _raw: PhantomData }
    }
}
